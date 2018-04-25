// Copyright 2018 The Rio Advancement Inc
//


use futures::{self, Future, Sink};
use futures::sync::mpsc;
use tokio_core::reactor::Handle;

use std::io;

use events::error::into_other;
use events::{InternalEvent, tobox};

#[derive(Debug, Clone)]
pub struct InternalPart {
    pub internal_tx: mpsc::Sender<InternalEvent>,
}

impl InternalPart {
    pub fn run(self) {
        let internal_tx = self.internal_tx.clone();
        internal_tx
            .send(InternalEvent::EntitlementTimeout)
            .map(drop)
            .wait()
            .map_err(into_other);
    }
}
