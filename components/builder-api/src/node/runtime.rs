// Copyright 2018 The Rio Advancement Inc

use std::sync::Arc;
use std::fmt;
use std::io;
use std::thread;

use config::Config;
use rio_net::http::middleware::BlockchainConn;

use events::{HandlerPart, InternalEvent};
use node::internal::InternalPart;
use events::error::{into_other, other_error};

use futures::{Future, Sink};
use futures::sync::mpsc;

use tokio_core::reactor::Core;

use protocol::api::audit::Envelope;
use entitlement::licensor::Client;

/// External messages.
#[derive(Debug)]
pub enum ExternalMessage {
    PeerAdd(Envelope),
}

/// Transactions sender.
#[derive(Clone, Debug)]
pub struct ApiSender(pub mpsc::Sender<ExternalMessage>);

/// Channel between the `RuntimeHandler` and events source.
#[derive(Debug)]
pub struct RuntimeChannel {
    /// Channel for api requests.
    pub api_requests: (mpsc::Sender<ExternalMessage>, mpsc::Receiver<ExternalMessage>),
    /// Channel for internal events.
    pub internal_events: (mpsc::Sender<InternalEvent>, mpsc::Receiver<InternalEvent>),
}

impl RuntimeChannel {
    /// Creates `RuntimeChannel` with the given pool capacities.
    pub fn new(buffer_sizes: usize) -> RuntimeChannel {
        RuntimeChannel {
            api_requests: mpsc::channel(buffer_sizes),
            internal_events: mpsc::channel(buffer_sizes),
        }
    }
}

/// Handler
pub struct RuntimeHandler {
    pub config: Box<BlockchainConn>,
    pub license: Box<Client>,
}

impl fmt::Debug for RuntimeHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeHandler ")
    }
}

impl ApiSender {
    /// Creates new `ApiSender` with given channel.
    pub fn new(inner: mpsc::Sender<ExternalMessage>) -> ApiSender {
        ApiSender(inner)
    }

    /// Add peer to peer list
    pub fn peer_add(&self, envl: Envelope) -> io::Result<()> {
        let msg = ExternalMessage::PeerAdd(envl);
        self.0.clone().send(msg).wait().map(drop).map_err(
            into_other,
        )
    }
}

pub struct Runtime {
    channel: RuntimeChannel,
    handler: RuntimeHandler,
}

impl Runtime {
    pub fn new(config: Arc<Config>) -> Self {
        Runtime {
            channel: RuntimeChannel::new(1024),
            handler: RuntimeHandler {
                config: Box::new(BlockchainConn::new(&*config.clone())),
                license: Box::new(Client::new(&*config.clone())),
            },
        }
    }
    /// Launches omessages handler.
    /// This may be used if you want to customize api with the `ApiContext`.
    pub fn start(self) -> io::Result<()> {
        let (handler_part, internal_part) = self.into_reactor();

        thread::spawn(move || {
            let mut core = Core::new()?;
            let handle = core.handle();
            core.run(internal_part.run(handle)).map_err(|_| {
                other_error(
                    "An error in the `RuntimeHandler:InternalPart` thread occurred",
                )
            })
        });


        thread::spawn(move || {
            let mut core = Core::new()?;
            core.run(handler_part.run()).map_err(|_| {
                other_error("An error in the `RuntimeHandler` thread occurred")
            })
        });

        Ok(())
    }

    /// Returns `RuntimeHandler`.
    pub fn handler(&self) -> &RuntimeHandler {
        &self.handler
    }

    /// Returns channel.
    pub fn channel(&self) -> ApiSender {
        ApiSender::new(self.channel.api_requests.0.clone())
    }

    fn into_reactor(self) -> (HandlerPart<RuntimeHandler>, InternalPart) {
        let (internal_tx, internal_rx) = self.channel.internal_events;

        let handler_part = HandlerPart {
            handler: self.handler,
            internal_rx,
            api_rx: self.channel.api_requests.1,
        };

        let internal_part = InternalPart { internal_tx };
        (handler_part, internal_part)
    }
}
