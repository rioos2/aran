pub mod roles;
pub mod authorizer;

//API resources that wish to expand its own using a cache can do so, by implementing
//this trait. The with_cache building the expander with the  behaviour  by defining
//what are the resources the cache needs to manage, and how does it do so.
//Every expandersender shall provide cache_closures of loading a cache to the expander.
//The expander supports multiple cache_closures.
//This is a singular expander meaning, if an id is provided it can provide the cache entry.
trait ExpanderSender: 'static + Send {
    fn with_cache(&mut self);
}
