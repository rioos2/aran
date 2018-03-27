// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use futures::{self, Future, Sink, Stream};
use futures::sync::mpsc;
use tokio_core::reactor::{Handle, Timeout};

use std::io;

use events::error::{into_other, other_error};
use events::{InternalEvent, tobox};

#[derive(Debug)]
pub struct InternalPart {
    pub internal_tx: mpsc::Sender<InternalEvent>,
}

impl InternalPart {
    pub fn run(self, handle: Handle) -> Box<Future<Item = (), Error = io::Error>> {

        let internal_tx = self.internal_tx.clone();

        let f = futures::lazy(move || {
            internal_tx
                .send(InternalEvent::Shutdown)
                .map(drop)
                .map_err(into_other)
        }).map_err(|_| panic!("Can't execute shutdown"));
        tobox(f)
    }
}
