
use ws;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use regex::Regex;
use bytes::Bytes;
use db::data_store::DataStoreConn;
use serde_json;
use serde_json::Value;
use watch::handler::WatchHandler;
use watch::handler::LISTENERS;

use std::rc::Rc;
use openssl::ssl::{SslAcceptor, SslStream};
use mio::tcp::TcpStream;
use nodesrv::node_ds::NodeDS;

use schedule_recv;

/// WebSocket server using trait objects to route
/// to an infinitely extensible number of handlers
// A WebSocket handler that routes connections to different boxed handlers by resource
pub struct Router {
    pub watchhandler: WatchHandler,
    pub sender: ws::Sender,
    pub inner: Box<ws::Handler>,
    pub datastore: Box<DataStoreConn>,
    pub register: Arc<Mutex<mpsc::SyncSender<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>>>,
    pub ssl: Rc<SslAcceptor>,
}

impl ws::Handler for Router {

    fn upgrade_ssl_server(&mut self, sock: TcpStream) -> ws::Result<SslStream<TcpStream>> {
        self.ssl.accept(sock).map_err(From::from)
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // Clone the sender so that we can move it into the child handler
        let out = self.sender.clone();
        let db = self.datastore.clone();
        let reg = self.register.clone();        
<<<<<<< HEAD
       
=======

>>>>>>> origin/2-0-stable
        let re = Regex::new("/api/v1/accounts/(\\w+)/watch").unwrap();        
        if re.is_match(req.resource()) {
            self.inner = Box::new(Data {
                    ws: out,
                    path: req.resource().to_string(),
                    datastore: db,
                    register: reg,
                    watchhandler: self.watchhandler.clone(),
                })
        } else {
            //TODO - use it for other websocket urls
            match req.resource() {
                // Route to a data handler
                "/api/v1/healthz/overall" => {      
                    self.inner = Box::new(Metrics {
                        ws: out,
                        datastore: db,
                        watchhandler: self.watchhandler.clone(),
                })          
                }  
                // Use the default child handler, NotFound
                _ => {
                    ()
                },
            }
            ()
        }

        // Delegate to the child handler
        self.inner.on_request(req)
    }

    // Pass through any other methods that should be delegated to the child.
    //
    // You could probably use a macro for this if you have many different
    // routers or were building some sort of routing framework.

    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err);
    }
}

// This handler returns a 404 response to all handshake requests
pub struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // This handler responds to all requests with a 404
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}

// This handler sends some data to the client and then terminates the connection on the first
// message received, presumably confirming receipt of the data
struct Data {
    ws: ws::Sender,
    watchhandler: WatchHandler,
    path: String,
    datastore: Box<DataStoreConn>,
    register: Arc<Mutex<mpsc::SyncSender<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>>>,
}

impl ws::Handler for Data {
    //when open a new socket connection first server collects all account specific data from database and 
    //send it to response. then start watch database, if any changes made into database, 
    //then server collect that data and send it.
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let re = Regex::new("/(\\w+)/watch").expect("regex");
        let captures = re.captures(&self.path).expect("captures");
        let id: String = captures.get(1).expect("1").as_str().parse().expect("parse");

        let register = self.register.clone();
        let sender = self.ws.clone();
        let watchhandler = self.watchhandler.clone();

        let send_wrap = match register.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let (ty, ry) = mpsc::channel();     
        let res_sender = Arc::new(Mutex::new(ty));  

        for listener in LISTENERS.iter() {
            thread::sleep(Duration::from_millis(1000)); 
            send_wrap.send((listener.to_string(), res_sender.clone())).unwrap();
            //when got new websocket connection, then server load list data
            //from database and send to it.                                                   
            match watchhandler.load_list_data(&listener, id.clone()) {
                Some(res) => {     
                    match sender.send(res) {
                        Ok(_success) => {}
                        Err(_err) => {
                            break;
                        }
                    }
                }
                None => {}
            }
        }

        thread::spawn(move || {
            loop {
                match ry.recv() {
                    Ok(msg) => {
                        //send data to response channel for particular accountant
                        //check response account id and request account id, if equal it could send response to channel
                        //otherwise skip it
                        let s = String::from_utf8(msg.to_vec()).expect("Found invalid UTF-8");  
                        let v: Value = serde_json::from_str(&s).unwrap();
                   
                        if v["data"]["object_meta"]["account"] == id.to_string() {
                            match sender.send(s) {
                                Ok(_success) => {}
                                Err(_err) => {
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
        Ok(())
    }
   
}


// This handler sends some data to the client and then terminates the connection on the first
struct Metrics {
    ws: ws::Sender,
    watchhandler: WatchHandler,
    datastore: Box<DataStoreConn>,
}

impl ws::Handler for Metrics {
    // when handler got a new connection, to collect overall metrics data from prometheus in time period 
    // scheduling time - 10s
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> { 
        let sender = self.ws.clone();
        let prom = self.watchhandler.prom_client();

        let tick = schedule_recv::periodic_ms(10000);
        thread::spawn(move || {
            loop {            
                match NodeDS::healthz_all(&prom) {
                    Ok(Some(health_all)) => {
                        let res = serde_json::to_string(&health_all).unwrap();
                        match sender.send(res) {
                            Ok(_success) => {}
                            Err(_err) => {break;}
                        }
                    }
                    Err(_err) => {break;},
                    Ok(None) => (),
                }
                tick.recv().unwrap();
            }
        });
        Ok(())
    }

}
