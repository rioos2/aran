//softwarekey/state.rs
// -----------
//! The purpose of this is to store the transient state updates that happen while the NativeSDK interact with SoftwareKey.

use protocol::api::base::{MetaFields, WhoAmITypeMeta};
use protocol::api::licenses::{Licenses, TRIAL};
use protocol::api::schema::type_meta_url;
use std::collections::BTreeMap;

/*A tuple that provides the license sub products and the default trial count to be activated
By default the ninjas are 5: Why
A site evaulates 1. Ninja node for digital cloud, 2. Ninja node for containers 3/4/5 for OpenIO or selfhealing
Senseis are restricted to 1. */
const PROVIDER: &'static str = "SoftwareKey";
const PRODUCT: &'static str = "Rio/OS v2";
pub const SUB_PRODUCTS: [(&'static str, i32, i32); 2] = [("senseis", 5, 5), ("ninjas", 10, 10)];

pub struct State {
    no_of_activations_available: i32, //the number of activations available for the license
    status: String, // status of the the license
    no_of_days_to_expire: String, //the number of days to expire
}
impl State {
    pub fn new() -> State {
        State {
            no_of_activations_available: 0,
            status: TRIAL.to_string(),
            no_of_days_to_expire: "".to_string(),
        }

    }

    pub fn mk_new(&self, name: &str) -> Licenses {
        let mut license = Licenses::new();

        let m = license.mut_meta(
            license.object_meta(),
            name.to_string(),
            license.get_account(),
        );

        let jackie = license.who_am_i();

        license.set_meta(type_meta_url(jackie), m);

        license.set_status(self.get_status());
        license.set_expired(self.get_no_of_days_to_expire());

        license.set_activation(self.calculate_limits(name));
        license.set_provider_name(PROVIDER.to_string());
        license.set_product(PRODUCT.to_string());
        license
    }

    pub fn current(&self, name: &str, license_id: &str, password: &str) -> Licenses {
        let mut license = Licenses::new();
        license.set_activation(self.current_calculate_limits(name));
        license.set_provider_name(name.to_string());
        license.set_status(self.get_status());
        license.set_expired(self.get_no_of_days_to_expire());
        license.set_license_id(license_id.to_string());
        license.set_password(password.to_string());
        license
    }

    fn calculate_limits(&self, name: &str) -> BTreeMap<String, i32> {
        let mut limits_map = BTreeMap::new();
        SUB_PRODUCTS
            .iter()
            .filter(|x| x.0 == name)
            .map(|l| {
                limits_map.insert("total_number_of_activations".to_string(), l.1);
                limits_map.insert("no_of_activations_available".to_string(), l.2);
            })
            .collect::<Vec<_>>();
        limits_map
    }

    fn current_calculate_limits(&self, name: &str) -> BTreeMap<String, i32> {
        let mut limits_map = BTreeMap::new();
        SUB_PRODUCTS
            .iter()
            .filter(|x| x.0 == name)
            .map(|l| {
                limits_map.insert("total_number_of_activations".to_string(), l.2);
                limits_map.insert(
                    "no_of_activations_available".to_string(),
                    self.get_no_of_activations_available(),
                );
            })
            .collect::<Vec<_>>();
        limits_map
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn set_no_of_days_to_expire(&mut self, no_of_days_to_expire: String) {
        self.no_of_days_to_expire = no_of_days_to_expire;
    }

    fn get_no_of_days_to_expire(&self) -> String {
        self.no_of_days_to_expire.clone()
    }

    pub fn set_no_of_activations_available(&mut self, no_of_activations_available: i32) {
        self.no_of_activations_available = no_of_activations_available;
    }

    pub fn get_no_of_activations_available(&self) -> i32 {
        self.no_of_activations_available
    }
}
