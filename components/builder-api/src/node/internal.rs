// Copyright 2018 The Rio Advancement Inc
//

use futures::{self, Future, Sink};
use futures::sync::mpsc;
use tokio_core::reactor::Handle;

use std::io;

use events::error::into_other;
use events::{InternalEvent, tobox};

#[derive(Debug)]
pub struct InternalPart {
    pub internal_tx: mpsc::Sender<InternalEvent>,
}

impl InternalPart {
    pub fn run(self, _handle: Handle) -> Box<Future<Item = (), Error = io::Error>> {
        let internal_tx = self.internal_tx.clone();

        let f = futures::lazy(move || internal_tx.send(InternalEvent::EntitlementTimeout).map(drop).map_err(into_other)).map_err(|_| panic!("Can't execute shutdown"));
        tobox(f)
    }
}
