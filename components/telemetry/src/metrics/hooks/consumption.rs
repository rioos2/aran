// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for setting the sensei nodes during the startup.

use metrics::PromResponse;
use metrics::hooks::BeforeMetrics;
use protocol::api::node;
use serde_json;

pub struct Consumption {
    content: PromResponse,
}

impl Consumption {
    pub fn new(content: PromResponse) -> Consumption {
        Consumption { content: content }
    }
    fn get_content(&mut self) -> Option<String> {
        let counters: node::Counters = self.content.clone().into();
        serde_json::to_string(&counters).ok()
    }
}

impl BeforeMetrics for Consumption {
    fn before(&mut self) -> Option<String> {
        self.get_content()
    }
}
