use std::fmt;
use std::sync::Arc;

use cache::multi_cache::MultiCache;

//The default cache time to live is 500ms = 0.5 seconds.
/// A generic cacher that takes a cache_load closure.
#[derive(Default, Debug, Clone)]
pub struct Cacher;

/// A generic cacher trait that takes a closure
///
impl Cacher {
    pub fn new() -> Cacher {
        Cacher {}
    }

    pub fn contains_key(&self, lru: &Box<MultiCache<String, String>>, key: String) -> bool {
        lru.contains_key(&key)
    }

    pub fn get(&self, lru: &Box<MultiCache<String, String>>, key: String) -> Option<Arc<String>> {
        debug!("« Flock GET: cached ≈ {}", key);
        lru.get(&key)
    }

    pub fn insert(
        &self,
        lru: &Box<MultiCache<String, String>>,
        key: String,
        value: Option<String>,
        existing_val_size: usize,
    ) -> Option<Arc<String>> {       
        match value {
            Some(v) => {
                debug!("» Flock PUT: Some cached ≈ {}", key);
                &mut lru.put(key, v.clone(), v.capacity(), existing_val_size);
                Some(Arc::new(v))
            }
            None => {
                debug!("» Flock PUT: None cached ≈ {}", key);
                None
            }
        }
    }
}

impl fmt::Display for Cacher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flock Cacher => is ok.")
    }
}
