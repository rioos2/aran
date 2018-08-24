// Copyright 2018 The Rio Advancement Inc

//! A collection of startup hooks

use protocol::api::node;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;


/// This is a workload hook function: is a closure that is responsible for presenting a startup
/// workload hook
type HookFn = Box<Fn() -> Option<String> + 'static + Send + Sync>;

/// The hook service function wrapper that is responsible for providing the prenup startup
/// routine workload hook encapsulation.
/// This has the key of the registered hook function example differ_hookah
#[derive(Clone)]
pub struct HookServiceFn {
    key: String,
    hook: Arc<HookFn>,
}

impl HookServiceFn {
    pub fn new(key: String, c: HookFn) -> Self {
        HookServiceFn {
            key: key,
            hook: Arc::new(c),
        }
    }

    fn key(&self) -> String {
        self.key.clone()
    }
}

impl fmt::Display for HookServiceFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HookServiceFn ")
    }
}

/// Wrapper around the standard `handler functions` to assist in formatting errors or success
#[derive(Clone)]
pub struct AHooks {
    pub hooks: BTreeMap<String, Box<HookServiceFn>>,
}

//Responsible for managing the registered Hooks. Ideally this shall be trait.
impl AHooks {
    pub fn new() -> Self {
        let hooks_map = BTreeMap::new();
        AHooks { hooks: hooks_map }
    }

    /// Registers the prenup startup HookServiceFn instances inside this ahook.
    /// A map stores the (key, hook service function) as a pair.
    pub fn register(&mut self, v: Box<HookServiceFn>) {
        self.hooks.insert(v.key(), v);
    }

    /// Returns the hookservice for the keys
    /// The registered hooks are invoked.
    ///
    pub fn get(&self, key: &str) -> ::std::option::Option<String> {
        (self.hooks.get(key).unwrap().hook)()
    }
}

impl fmt::Display for AHooks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AHooks => ({})", self.hooks.len())
    }
}
