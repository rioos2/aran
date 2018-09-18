//!The prenup startup hooks

pub mod before;
pub mod consumption;
pub mod instance;
pub mod metric;
mod process;
mod disk;
mod network;


//The trait responsible for providing a workload to be executed
//By default, we say that the startup hook needs to executed.
//This can be controlled by overriding satisfied function.
// Example: The satisfied is true, if the file pullcache/appstores.yaml exists
// The satisfied is true by default
pub trait BeforeMetrics: Send + Sized + 'static {
    //Override the before to provide the workload to be executed
    fn before(&mut self) -> Option<String>;
}
