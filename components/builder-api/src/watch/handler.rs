// Copyright 2018 The Rio Advancement Inc
//

//! The Watch stream handler
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use rand::RngCore;
use error::Result;
use watch;
use watch::messages::Messages;
use fallible_iterator::FallibleIterator;
use postgres::notification::Notification;
use db::data_store::DataStoreConn;
use db::error::Error as DbError;
use protocol::api::base::IdGet;
use telemetry::metrics::prometheus::PrometheusClient;
use api::security::config::SecurerConn;
use bytes::Bytes;
use rand::OsRng;
use regex::Regex;
use serde_json;
use serde_json::Value;

pub const LISTENERS: [&'static str; 2] = ["assemblyfactorys", "assemblys"];

#[derive(Clone)]
pub struct MyInner {
    v: Vec<(u32, String, Arc<Mutex<mpsc::Sender<Bytes>>>)>,
    datastore: Box<DataStoreConn>,
    prom: Box<PrometheusClient>,
    securer: Box<SecurerConn>,
}

#[derive(Clone)]
pub struct WatchHandler {
    pub datastore: Box<DataStoreConn>,
    inner: Arc<Mutex<MyInner>>,
    outer: MyInner,
    prom: Box<PrometheusClient>,
}

impl WatchHandler {
    pub fn new(datastore: Box<DataStoreConn>, prom: Box<PrometheusClient>, securer: Box<SecurerConn>) -> Self {
        let vec = Vec::<(u32, String, Arc<Mutex<mpsc::Sender<Bytes>>>)>::new();
        let ds = &datastore;
        let inner = MyInner {
            v: vec,
            datastore: ds.clone(),
            prom: prom.clone(),
            securer: securer.clone(),
        };
        WatchHandler {
            inner: Arc::new(Mutex::new(inner.clone())),
            datastore: ds.clone(),
            outer: inner.clone(),
            prom: prom.clone(),
        }
    }

    pub fn prom_client(&self) -> Box<PrometheusClient> {
        self.prom.clone()
    }

    pub fn datastore(&self) -> Box<DataStoreConn> {
        self.datastore.clone()
    }

    //start listening all psql triggers
    //when listener get the data from triggers then send it to the handler channel
    //listener notifies any datas(like secrets, jobs,...) send to the channel
    pub fn notifier(&self, sender: Arc<Mutex<mpsc::Sender<Notification>>>, listeners: Vec<&str>) -> Result<()> {
        let conn = self.datastore.pool.get_shard(0).unwrap();

        for listener in listeners {
            let mut owned_string: String = "LISTEN ".to_owned();
            let another_owned_string: String = "_trigger".to_owned();
            owned_string.push_str(&listener);
            owned_string.push_str(&another_owned_string);

            &conn.query(&owned_string, &[]).map_err(DbError::AsyncFunctionCheck);
        }

        thread::spawn(move || {
            let notifications = conn.notifications();
            let mut it = notifications.blocking_iter();
            let send = sender.clone();
            let send_wrap = send.lock().unwrap();

            // could not use 'loop' here because it does not compile in --release mode
            // since Ok() is unreachable.
            #[allow(while_true)]
            while true {
                match it.next() {
                    Ok(Some(notification)) => {
                        send_wrap.send(notification).unwrap();
                    }
                    Err(err) => println!("Streamer: Watch handler got err {:?}", err),
                    _ => {}
                }
            }
        });

        Ok(())
    }

    //register new requests "Myinner" vec storage
    //It is used for streaming data to specific requests
    pub fn register(&mut self, recv: mpsc::Receiver<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>) {
        let local_self = self.inner.clone();

        thread::spawn(move || {
            loop {
                //let msg = recv.recv().unwrap();
                match recv.recv() {
                    Ok(msg) => {
                        local_self.lock().unwrap().register(msg);
                    }
                    _ => {}
                }
            }
        });
    }

    //publish the response to requester
    pub fn publisher(&self, recv: mpsc::Receiver<Notification>) {
        let local_self = self.inner.clone();

        thread::spawn(move || {
            loop {
                //let msg = recv.recv().unwrap();
                match recv.recv() {
                    Ok(msg) => {
                        local_self.lock().unwrap().publish(msg);
                    }
                    _ => {}
                }
            }
        });
    }

    //get list data for particular account
    pub fn load_list_data(&self, typ: &str, act_id: String) -> Option<String> {
        self.outer.list_data(typ, act_id)
    }
}

pub type Peer = (String, Arc<Mutex<mpsc::Sender<Bytes>>>);

pub type PeerWithRandom = (u32, String, Arc<Mutex<mpsc::Sender<Bytes>>>);

impl MyInner {
    //store sender(requester) details into handler vec storage
    //generate random unique number for remove purpose because can't delete vec tuple without unique data
    fn register(&mut self, peer: Peer) {
        let mut rng = OsRng::new().expect("Error opening random number generator");
        self.v.push((rng.next_u32(), peer.0, peer.1));
    }

    //remove sender(requester) details into handler vec storage using unique number
    fn pop(&mut self, peer: PeerWithRandom) {
        self.v.retain(|ref x| x.0 != peer.0);
    }

    //send response to same type of watch clients from registered handler vec storage
    fn publish(&mut self, msg: Notification) {
        let peers = self.v.clone();
        let re = Regex::new("(\\w+)_trigger").expect("regex");
        let captures = re.captures(&msg.channel).expect("captures");
        let name: String = captures.get(1).expect("1").as_str().parse().expect("parse");

        for (a, b, c) in peers {
            if b == name {
                self.send_to_addr((a, b, c), msg.clone(), name.clone());
            }
        }
    }

    //get data from psql using id
    //and send it to requester
    fn send_to_addr(&mut self, peer: PeerWithRandom, msg: Notification, name: String) {
        let receiver = peer.2.clone();
        match receiver.lock() {
            Ok(recv) => {
                let res = self.get_data(msg, name);
                match recv.send(res) {
                    Ok(_success) => {}
                    Err(_err) => self.pop(peer),
                }
            }
            Err(p_err) => {
                println!("Poison Error: {}", p_err);
            }
        };
    }

    fn list_data(&self, typ: &str, act_id: String) -> Option<String> {
        let idget = IdGet::with_account(act_id);
        let one_ref_ds = &self.datastore;
        let res = match self.uppercase_first_letter(typ).parse().unwrap() {
            Messages::Assemblys => watch::messages::handle_assembly_list(idget, one_ref_ds.clone(), self.prom.clone()),
            Messages::Assemblyfactorys => watch::messages::handle_assemblyfactory_list(idget, one_ref_ds.clone()),
            Messages::Secrets => watch::messages::handle_secrets_list(idget, one_ref_ds.clone(), self.securer.clone()),
            Messages::Services => None,
            Messages::Ingress => None,
            Messages::Nodes => None,
            Messages::Senseis => None,
            Messages::Jobs => None,
            Messages::Horizontalscaling => None,
            Messages::Networks => None,
            Messages::Storagespool => None,
            Messages::Storageconnectors => None,
            Messages::Datacenters => None,
            Messages::Verticalscaling => None,
            Messages::Settingsmap => None,
            Messages::Endpoints => None,
            Messages::Origins => None,
            Messages::Plans => None,
            Messages::Serviceaccounts => None,
            Messages::Builds => None,
            Messages::Buildconfigs => None,
        };
        info!("==> watch handler list_data >> cloned");
        res
    }

    fn get_data(&self, msg: Notification, name: String) -> Bytes {
        let v: Value = serde_json::from_str(&msg.payload).unwrap();
        let idget = IdGet::with_id(v["data"].to_string());
        let typ = v["type"].to_string();

        let one_ref_ds = &self.datastore;

        let res = match self.uppercase_first_letter(&name).parse().unwrap() {
            Messages::Assemblys => watch::messages::handle_assembly(idget, typ, one_ref_ds.clone(), self.prom.clone()),
            Messages::Assemblyfactorys => watch::messages::handle_assemblyfactory(idget, typ, one_ref_ds.clone()),
            Messages::Services => watch::messages::handle_services(idget, typ, one_ref_ds.clone()),
            Messages::Ingress => watch::messages::handle_ingress(idget, typ, one_ref_ds.clone()),
            Messages::Nodes => watch::messages::handle_nodes(idget, typ, one_ref_ds.clone(), self.prom.clone()),
            Messages::Senseis => watch::messages::handle_senseis(idget, typ, one_ref_ds.clone()),
            Messages::Secrets => watch::messages::handle_secrets(idget, typ, one_ref_ds.clone(), self.securer.clone()),
            Messages::Jobs => watch::messages::handle_jobs(idget, typ, one_ref_ds.clone()),
            Messages::Horizontalscaling => watch::messages::handle_horizontalscaling(idget, typ, one_ref_ds.clone(), self.prom.clone()),
            Messages::Networks => watch::messages::handle_networks(idget, typ, one_ref_ds.clone()),
            Messages::Storagespool => watch::messages::handle_storagespool(idget, typ, one_ref_ds.clone()),
            Messages::Storageconnectors => watch::messages::handle_storageconnectors(idget, typ, one_ref_ds.clone()),
            Messages::Datacenters => watch::messages::handle_datacenters(idget, typ, one_ref_ds.clone()),
            Messages::Verticalscaling => watch::messages::handle_verticalscaling(idget, typ, one_ref_ds.clone(), self.prom.clone()),
            Messages::Settingsmap => watch::messages::handle_settingsmap(idget, typ, one_ref_ds.clone()),
            Messages::Endpoints => watch::messages::handle_endpoints(idget, typ, one_ref_ds.clone()),
            Messages::Origins => watch::messages::handle_origins(idget, typ, one_ref_ds.clone()),
            Messages::Plans => watch::messages::handle_plans(idget, typ, one_ref_ds.clone()),
            Messages::Serviceaccounts => watch::messages::handle_serviceaccounts(idget, typ, one_ref_ds.clone()),
            Messages::Builds => watch::messages::handle_builds(idget, typ, one_ref_ds.clone()),
            Messages::Buildconfigs => watch::messages::handle_builds_config(idget, typ, one_ref_ds.clone()),
        };
        info!("==> watch handler get_data >> cloned");
        return res;
    }

    fn uppercase_first_letter(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }
}
