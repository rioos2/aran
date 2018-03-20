/*// Copyright 2018 The Rio Advancement Inc
//

extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_aran_blockchain as blockchain;
extern crate exonum;
extern crate exonum_testkit;

use exonum::crypto::{self, PublicKey, SecretKey};
use exonum::messages::Message;
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

use protocol::api::audit::Envelope;


// Import datatypes used in tests from the crate where the service is defined.
use blockchain::api::audit::{TxCreateEnvelope, ApiResponseEnvelopePost, Habitat};

/// Wrapper for the habitat service API allowing to easily use it
/// (compared to `TestKitApi` calls).
struct HabitatApi {
    inner: TestKitApi,
}

impl HabitatApi {
    /// Generates a wallet creation transaction with a random key pair, sends it over HTTP,
    /// and checks the synchronous result (i.e., the hash of the transaction returned
    /// within the `TransactionResponse` struct).
    /// Note that the transaction is not immediately added to the blockchain, but rather is put
    /// to the pool of unconfirmed transactions.
    fn create_audit(&self, name: &str) -> (TxCreateEnvelope, SecretKey) {
        let (pubkey, key) = crypto::gen_keypair();
        // Create a presigned transaction
        let tx = TxCreateEnvelope::new(&pubkey, name, &key);

        let tx_info: ApiResponseEnvelopePost = self.inner
            .post(ApiKind::Service("habitat"), "v1/audits", &tx);
        assert_eq!(tx_info.tx_envl_hash, tx.hash());
        (tx, key)
    }

     /// Gets the state of a particular wallet using an HTTP request.
    fn get_audit(&self, account_id: &str) -> Vec<Envelope> {
        self.inner.get(
            ApiKind::Service("habitat"),
            &format!("v1/audits/{}", account_id.to_string()),
        )
    }
    
}

/// Creates a testkit together with the API wrapper defined above.
fn create_testkit() -> (TestKit, HabitatApi) {
    let testkit = TestKitBuilder::validator()
        .with_service(Habitat)
        .create();
    let api = HabitatApi {
        inner: testkit.api(),
    };
    (testkit, api)
}

/// Check that the wallet creation transaction works when invoked via API.
#[test]
fn test_create_audit() {
    let (mut testkit, api) = create_testkit();
    // Create and send a transaction via API
    let (tx, _) = api.create_audit("Alice");
    testkit.create_block();

    // Check that the user indeed is persisted by the service
    let envl = api.get_audit("Alice");
    //assert_eq!(envl.pub_key(), tx.pub_key());
}

/// Check that a transfer from a non-existing wallet fails as expected.
#[test]
fn test_get_audits() {
    let (mut testkit, api) = create_testkit();

    let (tx_alice, key_alice) = api.create_audit("Alice");
    let (tx_bob, _) = api.create_audit("Bob");
    let wallet = api.get_audit("Alice");
    //assert_eq!(wallet.balance(), 100);
}

*/
