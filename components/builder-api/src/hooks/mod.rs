//!The prenup startup hooks 

pub mod before;
pub mod differ;
pub mod secrets;
pub mod settings;


use error::Result;

//The trait responsible for providing a workload to be executed 
//By default, we say that the startup hook needs to executed.
//This can be controlled by overriding satisfied function.
// Example: The satisfied is true, if the file pullcache/appstores.yaml exists
// The satisfied is true by default
pub trait BeforeHook {
    fn satisfied(&self) -> bool {
        true
    }

    //Override the before to provide the workload to be executed
    fn before(&self) -> Result<()>;
}
