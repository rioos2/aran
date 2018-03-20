// Copyright 2018 The Rio Advancement Inc
//

use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

use bytes::Bytes;
use futures;
use futures::Stream;
use regex::Regex;

use httpbis::*;
use httpbis::Headers;

use db::data_store::DataStoreConn;

pub struct ServiceImpl {
    pub datastore: Box<DataStoreConn>,
    pub sender: Arc<Mutex<mpsc::SyncSender<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>>>,
}

impl Service for ServiceImpl {
    fn start_request(&self, headers: Headers, _req: HttpPartStream) -> Response {
        println!("Path ================> {:?}", headers.path());
        let re = Regex::new("/(\\w+)/watch").expect("regex");
        let captures = re.captures(headers.path()).expect("captures");
        let name: String = captures.get(1).expect("1").as_str().parse().expect("parse");
        let send = self.sender.clone();

        let send_wrap = match send.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let (ty, ry) = mpsc::channel();
        send_wrap.send((name, Arc::new(Mutex::new(ty)))).unwrap();

        let (tx, rx) = futures::sync::mpsc::channel(50);

        thread::spawn(move || {
            let mut tx = tx;
            loop {
                match ry.recv() {
                    Ok(msg) => {
                        //when client disconnect their watch request then this "is_disconnected()" method returns true
                        //then we break the thread                      
                        match tx.try_send(Bytes::from(msg)) {
                            Ok(_success) => {}
                            Err(err) => {
                                if err.is_disconnected() {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        });

        Response::headers_and_bytes_stream(Headers::ok_200(), rx.map_err(|_| unreachable!()))
    }
}
