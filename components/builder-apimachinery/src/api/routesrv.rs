use std::fmt;
use std::hash::Hasher;

use super::super::sharding::InstaId;

/// Defines a contract for protocol messages to be routed through `RouteSrv`.
pub trait Routable {
    /// Type of the route key
    type H: RouteKey + fmt::Display;

    /// Return a `RouteKey` for `RouteSrv` to know which key's value to route on.
    ///
    /// If `Some(T)`, the message will be routed by hashing the value of the route key and modding
    /// it against the shard count. This is known as "randomly deterministic routing".
    ///
    /// If `None`, the message will be randomly routed to an available node.
    fn route_key(&self) -> Option<Self::H>;
}

/// Provides an interface for hashing the implementing type for `Routable` messages.
///
/// Some types contain "hints" that help `RouteSrv` to identify the destination shard for a
/// message. You can leverage this trait to take any hints into account. See the implementation of
/// this trait for `InstaId` for an example.
pub trait RouteKey {
    /// Hashes a route key providing a route hash.
    fn hash(&self, hasher: &mut Hasher) -> u64;
}

impl RouteKey for String {
    fn hash(&self, hasher: &mut Hasher) -> u64 {
        hasher.write(self.as_bytes());
        hasher.finish()
    }
}

impl RouteKey for InstaId {
    fn hash(&self, _hasher: &mut Hasher) -> u64 {
        self.shard()
    }
}

impl RouteKey for u64 {
    fn hash(&self, _hasher: &mut Hasher) -> u64 {
        *self
    }
}
