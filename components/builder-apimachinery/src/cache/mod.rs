// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server
mod multi_cache;
mod flock;
pub mod inject;

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use serde_json;
use api;

use cache::multi_cache::MultiCache;
use cache::inject::{PlanFeeder, FactoryFeeder, EndPointsFeeder, VolumeFeeder, MetricFeeder, ServicesFeeder, PermissionFeeder};

use self::flock::Cacher;

pub const PULL_DIRECTLY: Option<bool> = Some(true);
pub const PULL_INVALDATED: Option<bool> = None;

pub const CACHE_PREFIX_PLAN: &'static str = "_plan";
pub const CACHE_PREFIX_FACTORY: &'static str = "_factory";
pub const CACHE_PREFIX_ENDPOINT: &'static str = "_endpoint";
pub const CACHE_PREFIX_VOLUME: &'static str = "_volume";
pub const CACHE_PREFIX_METRIC: &'static str = "_metric";
pub const CACHE_PREFIX_SERVICE: &'static str = "_service";

pub const CACHE_PREFIX_PERMISSION: &'static str = "_permission";


/// The fake type that decides how to pull the data from cache (invalidate or just from cache)
/// PULL_DIRECTLY: This loads from the cache is present, if the copy isn't there then it applies the function closure |_v| to cache the entry
/// PULL_INVALDATED: This invalidates the cache and loads a fresh copy

pub type PullFromCache = Option<bool>;

/// The cache service function closure that is responsible for invalidating a cache item. This does a live load
type LiveFn = Box<Fn(api::base::IdGet) -> Option<String> + 'static + Send + Sync>;

/// The default cache size we can accomodate. Once the cache is filled its starts to pop out the least used (LRU) principle.
const DEFAULT_CACHE_BYTE_SIZE: usize = 4028;

/// The cache service function wrapper that is responsible for invalidating a cache item. This does a live load
/// This has the key of the service function example _plan, _volume, _factory
#[derive(Clone)]
pub struct NewCacheServiceFn {
    key: String,
    live: Arc<LiveFn>,
    cacher: Cacher,
}

impl NewCacheServiceFn {
    pub fn new(key: String, c: LiveFn) -> Self {
        NewCacheServiceFn {
            key: key,
            live: Arc::new(c),
            cacher: Cacher::new(),
        }
    }
}

impl fmt::Display for NewCacheServiceFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CacheServiceFn => ({:?}) ({:?})", self.key, self.cacher)
    }
}

#[allow(unused_variables, unused_mut)]
pub trait CacheService: Send + Sync {
    //The prefix key of the cache service
    fn key(&self) -> String;

    //The invalidate apply function for the cache
    fn apply(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>);

    //The getter for the cache
    fn get(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>) -> Option<Arc<String>>;

    //The invalidator for the cache
    fn invalidate(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>) -> Option<Arc<String>>;

    //The cache id as its stored in the lru multi cache. This will be of the format
    // _plan_<id>
    fn cache_id(&self, id: api::base::IdGet) -> String;

    //The wrapper that handled the lru multi cache. This is a fascade to the actual cache. We have this function as it
    //helps to easily change the underlying lru cache implementation.
    fn cache(&self) -> &Cacher;
}

/// The Cache service function wrapper that implements the cache service.
impl CacheService for NewCacheServiceFn {
    fn key(&self) -> String {
        self.key.clone()
    }

    fn apply(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>) {
        info!("✔ apply cache ≈ {}", id);
        self.cache().insert(
            lru,
            self.cache_id(id.clone()).clone(),
            (self.live)(id),
        )
    }

    fn get(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>) -> Option<Arc<String>> {
        let _self = self.cache();
        let _cache_id = self.cache_id(id.clone());

        match _self.get(lru, _cache_id.clone()) {
            Some(value) => {
                info!("✔ get: cachefn ≈ {}", _cache_id.clone());
                Some(value.to_owned())
            }
            None => {
                info!("✘ get: cachefn ≈ {}", _cache_id.clone());
                self.invalidate(id, lru)
            }
        }
    }

    fn invalidate(&self, id: api::base::IdGet, lru: &Box<MultiCache<String, String>>) -> Option<Arc<String>> {
        info!("✔ get: invalidate ≈ {}", id);
        self.apply(id.clone(), lru);
        self.cache().get(lru, self.cache_id(id).clone())
    }

    fn cache_id(&self, id: api::base::IdGet) -> String {
        format!("{}_{}", &self.key(), id.get_id().clone())
    }

    /// Returns the cacher
    fn cache(&self) -> &Cacher {
        &self.cacher
    }
}

/// Wrapper around the standard `handler functions` to assist in formatting errors or success
#[derive(Clone)]
pub struct InMemoryExpander {
    pub cached: BTreeMap<String, Box<NewCacheServiceFn>>,
    lru: Box<MultiCache<String, String>>,
}

//Responsible for managing the inmemoryexpander. Ideally this shall be trait.
impl InMemoryExpander {
    pub fn new() -> Self {
        let cached_map = BTreeMap::new();
        let lru = MultiCache::<String, String>::new(DEFAULT_CACHE_BYTE_SIZE);

        InMemoryExpander {
            cached: cached_map,
            lru: Box::new(lru),
        }
    }

    /// Appends the NewCacheServiceFn instances inside this expander.
    /// A map stores the (key, cache service function) as a pair.
    pub fn with(&mut self, v: Box<NewCacheServiceFn>) {
        self.cached.insert(v.key(), v);
    }

    /// Returns the cacheservice for the prefix keys
    /// - CACHE_PREFIX_PLAN, CACHE_PREFIX_FACTORY, CACHE_PREFIX_VOLUME, CACHE_PREFIX_ENDPOINT,
    /// - CACHE_PREFIX_METRIC
    fn cache_service_for(&self, key: String) -> ::std::option::Option<&Box<NewCacheServiceFn>> {
        self.cached.get(&key).map(|p| p)
    }

    /// Returns the cache value for the cache_id
    /// - key_<id> eg:
    /// - CACHE_PREFIX_PLAN_<id>
    /// - CACHE_PREFIX_FACTORY_<id>
    /// - CACHE_PREFIX_VOLUME_<id>
    /// - CACHE_PREFIX_ENDPOINT_<id>
    /// - CACHE_PREFIX_METRIC
    /// Fetches from the cache first, if it isn't there then it runs the apply by loading data once.
    fn cached_value_for(&self, key: String, id: api::base::IdGet) -> Option<String> {
        let mut _lru = &self.lru;

        match self.cache_service_for(key.clone()) {
            Some(cachefn) => cachefn.get(id, _lru).map(|a| a.clone().deref().to_string()),
            _ => None,
        }
    }

    /// Returns the invalidated cache value for the cache_id
    /// - key_<id> eg:
    /// - CACHE_PREFIX_PLAN_<id>
    /// - CACHE_PREFIX_FACTORY_<id>
    /// - CACHE_PREFIX_VOLUME_<id>
    /// - CACHE_PREFIX_ENDPOINT_<id>
    /// - CACHE_PREFIX_METRIC
    /// Does an apply for the id and fetches that data from the cache,
    /// This will be the latest value of the id at any particular instant.
    fn cached_invalidate_for(&self, key: String, id: api::base::IdGet) -> Option<String> {
        let mut _lru = &self.lru;
        debug!("» Cache Invalidate key: {:?}", key.clone());
        debug!("» Cache Invalidate id: {:?}", id);
        match self.cache_service_for(key.clone()) {
            Some(cachefn) => {
                cachefn.invalidate(id, _lru).map(|a| {
                    a.clone().deref().to_string()
                })
            }
            _ => None,
        }
    }

    /// Expands a structure with the plan information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_plan<P: PlanFeeder>(&self, p: &mut P, force: Option<bool>) {
        let pid = p.pget_id();

        let opt_found_as_str = {
            force.map_or_else(
                || {
                    debug!("» Plan Invalidate fn for ≈ {}", pid);
                    self.cached_invalidate_for(CACHE_PREFIX_PLAN.to_string(), pid.clone())
                        .clone()
                },
                |_v| {
                    debug!("» Plan cache get fn for ≈ {}", pid);
                    self.cached_value_for(CACHE_PREFIX_PLAN.to_string(), pid.clone())
                        .clone()
                },
            )
        };

        p.pfeed(opt_found_as_str.clone().and_then({
            |found_as_str| {
                let plan: Option<api::blueprint::Plan> = serde_json::from_str(&found_as_str).ok();
                plan
            }
        }))
    }

    /// Expands a structure with the factory information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_factory<F: FactoryFeeder>(&self, f: &mut F, force: Option<bool>) {
        let fid = f.fget_id();
        let opt_found_as_str = {
            force.map_or_else(
                || {
                    debug!("» Assemblyfactory Invalidate fn for ≈ {}", fid);
                    self.cached_invalidate_for(CACHE_PREFIX_FACTORY.to_string(), fid.clone())
                        .clone()
                },
                |_v| {
                    debug!("» Assemblyfactory cache fn for ≈ {}", fid);
                    self.cached_value_for(CACHE_PREFIX_FACTORY.to_string(), fid.clone())
                        .clone()
                },
            )
        };

        f.ffeed(opt_found_as_str.and_then({
            |found_as_str| {
                let factory: Option<api::deploy::AssemblyFactory> = serde_json::from_str(&found_as_str).ok();
                factory
            }
        }))
    }

    /// Expands a structure with the endpoint information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_endpoints<E: EndPointsFeeder>(&self, e: &mut E, force: Option<bool>) {
        let eid = e.eget_id();
        let opt_found_as_str = {
            force.map_or_else(
                || {
                    debug!("» Endpoints Invalidate fn for ≈ {}", eid);
                    self.cached_invalidate_for(CACHE_PREFIX_ENDPOINT.to_string(), eid.clone())
                        .clone()
                },
                |_v| {
                    debug!("» Endpoints cache fn for ≈ {}", eid);
                    self.cached_value_for(CACHE_PREFIX_ENDPOINT.to_string(), eid.clone())
                        .clone()
                },
            )
        };

        e.efeed(opt_found_as_str.and_then({
            |found_as_str| {
                let endpoint: Option<api::endpoints::EndPoints> = serde_json::from_str(&found_as_str).ok();
                endpoint
            }
        }))
    }


    /// Expands a structure with the service information.
    /// If force is Some, then it applies the function closure |_s| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_services<V: ServicesFeeder>(&self, s: &mut V, force: Option<bool>) {
        let sid = s.sget_id();
        let opt_found_as_str = {
            force.map_or_else(
                || {
                    self.cached_invalidate_for(CACHE_PREFIX_SERVICE.to_string(), sid.clone())
                        .clone()
                },
                |_v| {
                    self.cached_value_for(CACHE_PREFIX_SERVICE.to_string(), sid.clone())
                        .clone()
                },
            )
        };

        s.sfeed(opt_found_as_str.and_then({
            |found_as_str| {
                let service: Option<Vec<api::linker::Services>> = serde_json::from_str(&found_as_str).ok();
                service
            }
        }))
    }


    /// Expands a structure with the volume information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_volumes<V: VolumeFeeder>(&self, v: &mut V, force: Option<bool>) {
        let vid = v.vget_id();
        let opt_found_as_str = {
            force.map_or_else(
                || {
                    debug!("» Volumes Invalidate fn for ≈ {}", vid);
                    self.cached_invalidate_for(CACHE_PREFIX_VOLUME.to_string(), vid.clone())
                        .clone()
                },
                |_v| {
                    debug!("» Volumes cache fn for ≈ {}", vid);
                    self.cached_value_for(CACHE_PREFIX_VOLUME.to_string(), vid.clone())
                        .clone()
                },
            )
        };

        v.vfeed(opt_found_as_str.and_then({
            |found_as_str| {
                let volume: Option<Vec<api::volume::Volumes>> = serde_json::from_str(&found_as_str).ok();
                volume
            }
        }))
    }



    /// Expands a structure with the volume information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_permission<V: PermissionFeeder>(&self, v: &mut V, force: Option<bool>) {
        let vid = v.p_get_id();
        let opt_found_as_str = {
            force.map_or_else(
                || {
                    debug!("» Permission Invalidate fn for ≈ {}", vid);
                    self.cached_invalidate_for(CACHE_PREFIX_PERMISSION.to_string(), vid.clone())
                        .clone()
                },
                |_v| {
                    debug!("» Permission cache fn for ≈ {}", vid);
                    self.cached_value_for(CACHE_PREFIX_PERMISSION.to_string(), vid.clone())
                        .clone()
                },
            )
        };

        v.p_feed(opt_found_as_str.and_then({
            |found_as_str| {
                let volume: Option<Vec<api::authorize::Permissions>> = serde_json::from_str(&found_as_str).ok();
                volume
            }
        }))
    }


    /// Expands a structure with the metrics information.
    /// If force is Some, then it applies the function closure |_v| which loads from the cache is present.
    /// If force is None, then it invalidates the cache and loads a fresh copy
    pub fn with_metrics<M: MetricFeeder>(&self, m: &mut M, force: Option<bool>) {
        let mid = m.mget_id();

        let opt_found_as_str = {
            force.map_or_else(
                || {
                    self.cached_invalidate_for(CACHE_PREFIX_METRIC.to_string(), mid.clone())
                        .clone()
                },
                |_v| {
                    self.cached_value_for(CACHE_PREFIX_METRIC.to_string(), mid.clone())
                        .clone()
                },
            )
        };

        m.mfeed(opt_found_as_str.and_then({
            |found_as_str| {
                let metric: Option<BTreeMap<String, String>> = serde_json::from_str(&found_as_str).ok();
                metric
            }
        }))
    }
}

impl fmt::Display for InMemoryExpander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IM => ({}) ({:?})", self.cached.len(), self.lru)
    }
}
