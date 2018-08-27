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

pub trait VolumesFeeder: Send {
    fn vget_id(&mut self) -> api::base::IdGet;

    fn vfeed(&mut self, v: Option<Vec<api::volume::Volumes>>); //change it to mount::Volume
}

pub trait MetricsFeeder: Send {
    fn mget_id(&mut self) -> api::base::IdGet;

    fn mfeed(&mut self, v: Option<BTreeMap<String, String>>);
}

pub trait ServicesFeeder: Send {
    fn sget_id(&mut self) -> api::base::IdGet;

    fn sfeed(&mut self, v: Option<Vec<api::linker::Services>>);
}

pub trait PermissionsFeeder: Send {
    fn iget_id(&mut self) -> api::base::IdGet;

    fn ifeed(&mut self, v: Option<Vec<api::authorize::Permissions>>);
}

pub trait AccountsFeeder: Send {
    fn iget_id(&mut self) -> api::base::IdGet;

    fn ifeed(&mut self, v: Option<api::session::Account>);
}

pub trait ServiceAccountFeeder: Send {
    fn iget_id(&mut self) -> api::base::IdGet;

    fn ifeed(&mut self, v: Option<api::service_account::ServiceAccount>);
}

pub trait StacksFeeder: Send {
    fn bget_id(&mut self) -> api::base::IdGet;

    fn bfeed(&mut self, a: Option<api::deploy::StacksFactory>);
}

pub trait LicensesFeeder: Send {
    fn iget_id(&mut self) -> api::base::IdGet;

    fn ifeed(&mut self, v: Option<String>);
}

pub trait MembersFeeder: Send {
    fn eget_id(&mut self) -> api::base::IdGet;

    fn efeed(&mut self, e: Option<Vec<api::invitations::Invitations>>);
}

pub trait TeamsFeeder: Send {
    fn eget_id(&mut self) -> api::base::IdGet;

    fn efeed(&mut self, e: Option<api::authorize::Teams>);
}

