
#[cfg(feature="ssl")] 
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
#[cfg(feature = "ssl")]
use openssl::pkcs12::Pkcs12;

#[cfg(feature = "ssl")]
use std::rc::Rc;
#[cfg(feature = "ssl")]
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod, SslStream};

#[cfg(feature = "ssl")]
use ws::util::TcpStream;

/// WebSocket server using trait objects to route
/// to an infinitely extensible number of handlers
// A WebSocket handler that routes connections to different boxed handlers by resource
#[cfg(feature="ssl")]
pub struct Router {
    pub watchhandler: WatchHandler,
    pub sender: ws::Sender,
    pub inner: Box<ws::Handler>,
    pub datastore: Box<DataStoreConn>,
    pub register: Arc<Mutex<mpsc::SyncSender<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>>>,
    pub ssl: Rc<SslAcceptor>,
}

#[cfg(feature="ssl")] 
impl ws::Handler for Router {

    fn upgrade_ssl_server(&mut self, sock: TcpStream) -> ws::Result<SslStream<TcpStream>> {
        self.ssl.accept(sock)
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // Clone the sender so that we can move it into the child handler
        let out = self.sender.clone();
        let db = self.datastore.clone();
        let reg = self.register.clone();

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
            /*match req.resource() {
                // Route to a data handler
                "/api/v1/logs" => {                
                }  
                // Use the default child handler, NotFound
                _ => {
                    ()
                },
            }*/
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
#[cfg(feature="ssl")] 
pub struct NotFound;

#[cfg(feature="ssl")] 
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
#[cfg(feature="ssl")] 
struct Data {
    ws: ws::Sender,
    watchhandler: WatchHandler,
    path: String,
    datastore: Box<DataStoreConn>,
    register: Arc<Mutex<mpsc::SyncSender<(String, Arc<Mutex<mpsc::Sender<Bytes>>>)>>>,
}

#[cfg(feature="ssl")] 
impl ws::Handler for Data {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("Path ================> {:?}", self.path);
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
                        Err(err) => {
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
                        //when client disconnect their watch request then this "is_disconnected()" method returns true
                        //then we break the thread   
                        let s = String::from_utf8(msg.to_vec()).expect("Found invalid UTF-8");  
                        let v: Value = serde_json::from_str(&s).unwrap();
                   
                        if v["data"]["object_meta"]["account"] == id.to_string() {
                            match sender.send(s) {
                                Ok(_success) => {}
                                Err(err) => {
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
