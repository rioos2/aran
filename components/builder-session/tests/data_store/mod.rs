use protocol::{sessionsrv, servicesrv, asmsrv, originsrv};
use db::data_store::Broker;
use session::session_ds::SessionDS;
use rand;
const ORIGIN: &'static str = "Origin";


fn create_force_account() -> sessionsrv::Session {
    let mut ac = sessionsrv::SessionCreate::new();
    let conn = Broker::connect().unwrap();
    let api_key = rand::random::<u64>().to_string();
    ac.set_apikey(api_key);
    ac.set_email("vino@info.io".to_string());
    ac.set_token("t34567#$%dfgbnmkjhgfdfgbn".to_string());
    ac.set_password("123456789".to_string());
    SessionDS::find_or_create_account_via_session(&conn, &ac, true, false, "select_or_insert_account_v1").expect("Should create account")
}

// fn origin_create_fn() -> originsrv::Origin {
//     let conn = Broker::connect().unwrap();
//     let mut origin_create = originsrv::Origin::new();
//     let uid: sessionsrv::Session = create_force_account();
//     let mut object_meta = servicesrv::ObjectMetaData::new();
//     object_meta.set_origin("rioos1".to_string());
//     object_meta.set_name("rioos1".to_string());
//     object_meta.set_uid(uid.get_id());
//     origin_create.set_object_meta(object_meta);
//     origin_create.set_type_meta(asmsrv::TypeMeta::new(constants::ORIGIN));
//
//     SessionDS::origin_create(&conn, &origin_create).unwrap()
//
//
// }

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

    let session = SessionDS::find_or_create_account_via_session(
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


#[test]
fn get_account_by_email() {
    let conn = Broker::connect().unwrap();
    let account: sessionsrv::Session = create_force_account();
    let mut ac = sessionsrv::AccountGet::new();
    ac.set_email(account.get_email().to_string());
    let get_ac = SessionDS::get_account(&conn, &ac)
        .expect("Should run without error")
        .expect("Account should exist");

    assert_eq!(account.get_email(), get_ac.get_email());

}

#[test]
fn get_account_by_id() {
    let conn = Broker::connect().unwrap();
    let account: sessionsrv::Session = create_force_account();
    let mut ac = sessionsrv::AccountGetId::new();
    ac.set_id(account.get_id().to_string());
    let get_ac = SessionDS::get_account_by_id(&conn, &ac)
        .expect("Should run without error")
        .expect("Account should exist");

    assert_eq!(account.get_id(), get_ac.get_id());

}

#[test]
fn origin_create() {
    let conn = Broker::connect().unwrap();
    let mut origin_create = originsrv::Origin::new();
    let uid: sessionsrv::Session = create_force_account();
    let mut object_meta = servicesrv::ObjectMetaData::new();
    object_meta.set_origin("rioos".to_string());
    object_meta.set_name("rioos".to_string());
    object_meta.set_uid(uid.get_id());
    origin_create.set_object_meta(object_meta);
    origin_create.set_type_meta(asmsrv::TypeMeta::new(ORIGIN));

    let origin = SessionDS::origin_create(&conn, &origin_create)
        .unwrap()
        .expect("Should create origin");
    assert!(origin.get_id() != "", "Created origin has id");
    assert_eq!(origin.get_object_meta().get_origin(), "rioos");

}

// #[test]
// fn origin_get() {
//     let conn = Broker::connect().unwrap();
//     let org: originsrv::Origin = origin_create_fn();
//     let mut org_name = asmsrv::IdGet::new();
//     org_name.set_name(org.get_object_meta().get_origin().to_string());
//     let get_org = SessionDS::origin_show(&conn, &org_name).expect("Should run without error");
//
//     assert_eq!(
//         org.get_object_meta().get_origin(),
//         get_org.get_object_meta().get_origin()
//     );
// }
