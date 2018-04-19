// Copyright 2018 The Rio Advancement Inc
//

//! Streamer that does the watch for the api
//!
use std;
use std::io;
use std::sync::{mpsc, Arc, Mutex};

use watch::handler::LISTENERS;
use watch::handler::WatchHandler;
use telemetry::metrics::prometheus::PrometheusClient;
use db::data_store::DataStoreConn;
use rio_net::http::middleware::SecurerConn;
use config::Config;
use watch::socket_service::{Router, NotFound};
use ws;
use openssl::pkcs12::Pkcs12;

use std::rc::Rc;
use openssl::ssl::{SslAcceptorBuilder, SslMethod};

use openssl::x509::X509Ref;

pub type TLSPair = Option<(String, Vec<u8>, String)>;

#[derive(Debug)]
pub struct Websocket {
    port: u16,
    config: Arc<Config>,
}

impl Websocket {
    pub fn new(port: u16, config: Arc<Config>) -> Self {
        Websocket { port: port.clone(), config: config.clone() }
    }

    //start websocket server
    //if rioos_home dir have apiserver.pfx file then server start using wss protocol,
    //otherwise start using ws protocol
    pub fn start(self, tls_pair: TLSPair) -> io::Result<()> {
        let ods = tls_pair.clone().and(DataStoreConn::new().ok());

        match ods {
            Some(ds) => {
                let mut watchhandler = WatchHandler::new(Box::new(ds.clone()), Box::new(PrometheusClient::new(&*self.config.clone())), Box::new(SecurerConn::new(&*self.config.clone())));

                let (db_sender, db_receiver) = mpsc::channel();
                let (reg_sender, reg_receiver) = mpsc::sync_channel(1);

                let send = Arc::new(Mutex::new(db_sender));
                let register = Arc::new(Mutex::new(reg_sender));

                watchhandler.notifier(send.clone(), LISTENERS.to_vec()).unwrap();

                watchhandler.publisher(db_receiver);

                watchhandler.register(reg_receiver);

                let tls_tuple = tls_pair.clone().unwrap(); //no panic, as ods handles it.

                let pkcs12 = Pkcs12::from_der(&tls_tuple.1).unwrap();
                let parsed = pkcs12.parse(&tls_tuple.2).unwrap();

                let acceptor = Rc::new(SslAcceptorBuilder::mozilla_intermediate(SslMethod::tls(), &parsed.pkey, &parsed.cert, std::iter::empty::<X509Ref>()).unwrap().build());

                let address = format!("{}:{}", self.config.http.listen.to_string(), self.port.to_string());
                // Listen on an address and call the closure for each connection

                ws::Builder::new()
                    .with_settings(ws::Settings {
                        //TODO: when we use wss scheme then change it to "true"
                        encrypt_server: false,
                        ..ws::Settings::default()
                    })
                    .build(|out: ws::Sender| {
                        Router {
                            watchhandler: watchhandler.clone(),
                            sender: out,
                            // Default to returning a 404 when the route doesn't match.
                            // You could default to any handler here.
                            inner: Box::new(NotFound),
                            datastore: Box::new(ds.clone()),
                            register: register.clone(),
                            ssl: acceptor.clone(),
                        }
                    })
                    .unwrap()
                    .listen(address)
                    .unwrap();
            }
            None => {
                return Ok(());
            }
        }
        Ok(())
    }
}
