use api;
use std::collections::BTreeMap;

pub trait PlanFeeder: Send {
    fn pget_id(&mut self) -> api::base::IdGet;

    fn pfeed(&mut self, p: Option<api::blueprint::Plan>);
}

pub trait FactoryFeeder: Send {
    fn fget_id(&mut self) -> api::base::IdGet;

    fn ffeed(&mut self, a: Option<api::deploy::AssemblyFactory>);
}

pub trait EndPointsFeeder: Send {
    fn eget_id(&mut self) -> api::base::IdGet;

    fn efeed(&mut self, e: Option<api::endpoints::EndPoints>);
}

pub trait VolumeFeeder: Send {
    fn vget_id(&mut self) -> api::base::IdGet;

    fn vfeed(&mut self, v: Option<Vec<api::volume::Volumes>>); //change it to mount::Volume
}

pub trait MetricFeeder: Send {
    fn mget_id(&mut self) -> api::base::IdGet;

    fn mfeed(&mut self, v: Option<BTreeMap<String, String>>);
}

pub trait PermissionFeeder: Send {
    fn p_get_id(&mut self) -> api::base::IdGet;

    fn p_feed(&mut self, v: Option<Vec<api::authorize::Permissions>>);
}


pub trait ServicesFeeder: Send {
    fn sget_id(&mut self) -> api::base::IdGet;

    fn sfeed(&mut self, v: Option<Vec<api::linker::Services>>);
}

pub trait BlockchainFactoryFeeder: Send {
    fn bget_id(&mut self) -> api::base::IdGet;

    fn bfeed(&mut self, a: Option<api::deploy::BlockchainFactory>);
}
