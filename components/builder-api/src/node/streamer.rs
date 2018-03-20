// Copyright 2018 The Rio Advancement Inc
//

//! Streamer that does the watch for the api
//!
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use watch::handler::WatchHandler;
use watch::service::ServiceImpl;
use rio_net::metrics::prometheus::PrometheusClient;
use config::Config;

use tls_api::TlsAcceptorBuilder as tls_api_TlsAcceptorBuilder;
use tls_api_openssl;

use httpbis;

use rio_net::http::middleware::SecurerConn;
use db::data_store::DataStoreConn;

pub type TLSPair = Option<(String, Vec<u8>, String)>;

#[derive(Debug)]
pub struct Streamer {
    watch_port: u16,
    config: Arc<Config>,
}

impl Streamer {
    pub fn new(watch_port: u16, config: Arc<Config>) -> Self {
        Streamer {
            watch_port: watch_port.clone(),
            config: config.clone(),
        }
    }

    pub fn start(self, tls_pair: TLSPair) -> io::Result<()> {
        let ods = tls_pair.clone().and(DataStoreConn::new().ok());
        let streamer_thread = match ods {
            Some(ds) => {
                let mut watchhandler = WatchHandler::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                    Box::new(SecurerConn::new(&*self.config.clone())),
                );

                let (db_sender, db_receiver) = mpsc::channel();
                let (reg_sender, reg_receiver) = mpsc::sync_channel(1);

                let send = Arc::new(Mutex::new(db_sender));

                watchhandler.notifier(send.clone()).unwrap();

                watchhandler.publisher(db_receiver);

                watchhandler.register(reg_receiver);

                let thread = thread::spawn(move || {
                    let mut conf = httpbis::ServerConf::new();
                    conf.alpn = Some(httpbis::ServerAlpn::Require);

                    let tls_tuple = tls_pair.clone().unwrap(); //no panic, as ods handles it.

                    let mut tls_acceptor = tls_api_openssl::TlsAcceptorBuilder::from_pkcs12(&tls_tuple.1, &tls_tuple.2).expect("acceptor builder");

                    tls_acceptor
                        .set_alpn_protocols(&[b"h2"])
                        .expect("set_alpn_protocols");

                    let mut server = httpbis::ServerBuilder::new();
                    println!("watch streamer running on port {} ", self.watch_port);
                    server.set_port(self.watch_port);
                    server.set_tls(tls_acceptor.build().expect("tls acceptor"));
                    server.conf = conf;

                    server.service.set_service(
                        "/api/v1",
                        Arc::new(ServiceImpl {
                            datastore: Box::new(ds),
                            sender: Arc::new(Mutex::new(reg_sender)),
                        }),
                    );

                    let running = server.build().expect("server");

                    info!("watch streamer started");
                    info!(
                        "watch streamer running: https://localhost:{}/",
                        running.local_addr().port().unwrap()
                    );
                    loop {
                        thread::park();
                    }
                });
                Some(thread)
            }
            None => None,
        };
        if let Some(streamer_thread) = streamer_thread {
            streamer_thread.join().unwrap();
        }
        Ok(())
    }
}
