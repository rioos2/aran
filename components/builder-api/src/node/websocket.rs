// Copyright 2018 The Rio Advancement Inc
//

//! Streamer that does the watch for the api
//!
use std::io;
use std::sync::{mpsc, Arc, Mutex, RwLock};

use watch::handler::WatchHandler;
use rio_net::metrics::prometheus::PrometheusClient;
use config::Config;

use rio_net::http::middleware::SecurerConn;
use db::data_store::DataStoreConn;

use websocket;
use websocket::async::Server;
use websocket::OwnedMessage;
use futures;
use futures::sync::mpsc as futurempsc;
use tokio_core;
use tokio_core::reactor::{Handle, Remote, Core};
use futures_cpupool::CpuPool;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use websocket::server::InvalidConnection;
use futures::{Future, Sink, Stream};
use std::fmt::Debug;
use bytes::Bytes;
use common::uri::URI;
use serde_json;
use serde_json::Value;

//use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
//use std::fs::File;
//use std::io::{Read};

pub type TLSPair = Option<(String, Vec<u8>, String)>;
type Id = u32;

#[derive(Debug)]
pub struct Websocket {
    port: u16,
    config: Arc<Config>,
}

impl Websocket {
    pub fn new(port: u16, config: Arc<Config>) -> Self {
        Websocket {
            port: port.clone(),
            config: config.clone(),
        }
    }

    pub fn start(self, tls_pair: TLSPair) -> io::Result<()> {
        let ods = tls_pair.clone().and(DataStoreConn::new().ok());
        let listeners = vec![
            "services",
            "assemblyfactorys",
            "assemblys",
        ];
        
        match ods {
            Some(ds) => {
                let mut core = Core::new().expect("Failed to create Tokio event loop");
                let handle = core.handle();
                let remote = core.remote();
                let pool = Rc::new(CpuPool::new_num_cpus());

                let mut watchhandler = WatchHandler::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                    Box::new(SecurerConn::new(&*self.config.clone())),
                );

                let (db_sender, db_receiver) = mpsc::channel();
                let (data_send, data_recv) = futurempsc::unbounded();
                let (send_channel_out, send_channel_in) = futurempsc::unbounded();

                let send = Arc::new(Mutex::new(db_sender));

                watchhandler.notifier(send.clone(), listeners).unwrap();

                watchhandler.socket_publisher(db_receiver, data_send);

                let address = format!("{}:{}", self.config.http.listen.to_string(), self.port.to_string());

                //TODO - for wss server
                /*let tls_tuple = tls_pair.clone().unwrap(); //no panic, as ods handles it.
                let path = String::from_utf8(&tls_tuple.1).expect("Found invalid UTF-8");
                let mut file = File::open(path).unwrap();
                let mut pkcs12 = vec![];
                file.read_to_end(&mut pkcs12).unwrap();
                let pkcs12 = Pkcs12::from_der(&pkcs12, &tls_tuple.2).unwrap();
                let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();
                let server = Server::bind_secure(address, acceptor, &handle).expect("Failed to create server");*/
               
                let server = Server::bind(address, &handle).expect("Failed to create server");
                let connections = Arc::new(RwLock::new(HashMap::new()));

                let conn_id = Rc::new(RefCell::new(Counter::new()));
                let connections_inner = connections.clone();
                // Handle new connection
                let connection_handler = server.incoming()
                        // we don't wanna save the stream if it drops
                        .map_err(|InvalidConnection { error, .. }| error)
                        .for_each(move |(upgrade, addr)| {
                            let connections_inner = connections_inner.clone();
                            println!("Got a connection from: {}", addr);

                            //parse account id from requested websocket url
                            //and stored into connections hash
                            let uri = URI::new(upgrade.request.subject.1.clone());
                            let ref_id = uri.id();                            
                            
                            let conn_id = conn_id.clone();
                            let f = upgrade
                                    .accept()
                                    .and_then(move |(framed, _)| {
                                        let id = conn_id
                                            .borrow_mut()
                                            .next()
                                            .expect("maximum amount of ids reached");
                                        let (sink, _) = framed.split();                                     
                                        
                                        connections_inner.write().unwrap().insert(id, (sink,ref_id));
                                        Ok(())
                                    });
                            spawn_future(f, "Handle new connection", &handle);
                            Ok(())
                        }).map_err(|_| ());
               

                // Handle sending messages to a client
                let connections_inner = connections.clone();
                let remote_inner = remote.clone();
                let send_handler = pool.spawn_fn(move || {
                    let connections = connections_inner.clone();
                    let remote = remote_inner.clone();
                    send_channel_in.for_each(move |(id, msg): (Id, String)| {
                        let connections = connections.clone();
                        let tuple = connections.write()
                                    .unwrap()
                                    .remove(&id)
                                    .expect("Tried to send to invalid client id",);
                        let ref_id = tuple.1.clone();
                        println!("Sending message '{}' to id {}", msg, id);
                        let f = tuple.0.send(OwnedMessage::Text(msg))
                                    .and_then(move |sink| {
                                        connections.write().unwrap().insert(id, (sink, ref_id));
                                        Ok(())
                                     });                       
                        remote.spawn(move |_| f.map_err(|_| ()));
                        Ok(())
                    }).map_err(|_| ())
                });

                // when data_recv channel get data then in this section send data to response send channel 
                let connections_inner = connections.clone();
                let remote_inner = remote.clone();
                let main_loop = pool.spawn_fn(move || {
                    let connections = connections_inner.clone();
                    let remote = remote_inner.clone();
                    data_recv.for_each(move |msg: Bytes| {
                        update(connections.clone(), send_channel_out.clone(), &remote, msg);
                        Ok(())
                    }).map_err(|_| ())
                });
                
                //start all spawn future threads in parallel
                let handlers = connection_handler.select2(main_loop.select2(send_handler));
                core.run(handlers).map_err(|_| println!("Error while running core loop")).unwrap();
            }                
            None => {
                return Ok(());
            },
        }
        
        Ok(())
    }       
}   

fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
    where F: Future<Item = I, Error = E> + 'static,
          E: Debug
{
    handle.spawn(f.map_err(move |e| println!("Error in {}: '{:?}'", desc, e))
                  .map(move |_| println!("{}: Finished.", desc)));
}

type SinkContent = websocket::client::async::Framed<tokio_core::net::TcpStream,
                                                    websocket::async::MessageCodec<OwnedMessage>>;
type SplitSink = futures::stream::SplitSink<SinkContent>;

// check account_id of received data, if it is equal then it is send to response channel,
// otherwise skip it
fn update(
    connections: Arc<RwLock<HashMap<Id, (SplitSink, String)>>>,
    channel: futurempsc::UnboundedSender<(Id, String)>,
    remote: &Remote,
    msg: Bytes,
) {
    remote.spawn(move |handle| {
                for (id, tuple) in connections.read().unwrap().iter() {
                    let s = String::from_utf8(msg.to_vec()).expect("Found invalid UTF-8");
                    let v: Value = serde_json::from_str(&s).unwrap();
                   
                    if v["data"]["object_meta"]["account"] == tuple.1.to_string() {
                        let f = channel.clone().send((*id, s));
                        spawn_future(f, "Send message to write handler", handle);
                    }  
                }
                Ok(())
    });
}

//Counter - it counts for incoming connections
struct Counter {
    count: Id,
}
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = Id;

    fn next(&mut self) -> Option<Id> {
        if self.count != Id::max_value() {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}