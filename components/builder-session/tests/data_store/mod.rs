use protocol::{sessionsrv, servicesrv, asmsrv, originsrv, constants};
use db::data_store::DataStoreConn;
use session::models::session;
use rand;
const ORIGIN: &'static str = "Origin";

#[test]
fn create_account() {
    let conn = Broker::connect().unwrap();
    let mut account_create = sessionsrv::SessionCreate::new();
    let api_key = rand::random::<u64>().to_string();
    account_create.set_apikey(api_key);
    account_create.set_token("ty987645yhnbfert".to_string());
    account_create.set_name("suganya".to_string());
    account_create.set_email("suga@riocorp.io".to_string());
    account_create.set_first_name("suagnya".to_string());
    account_create.set_last_name("k".to_string());
    account_create.set_phone("1234567890".to_string());
    account_create.set_password("12345678".to_string());
    account_create.set_states("true".to_string());
    account_create.set_suspend("suapendes".to_string());
    account_create.set_approval("approve".to_string());
    account_create.set_roles(vec!["role/rioos:superuser".to_string()]);
    account_create.set_registration_ip_address("192.1.1.1".to_string());
    account_create.set_trust_level("platinum".to_string());

    let session = session::Datastore::find_or_create_account_via_session(
        &conn,
        &account_create,
        true,
        false,
        "select_or_insert_account_v1",
    ).expect("Should create account");
    assert!(session.get_id() != "", "Created account has an ID");
    assert_eq!(session.get_email(), "suga@riocorp.io");
    assert_eq!(session.get_name(), "suganya");
}
