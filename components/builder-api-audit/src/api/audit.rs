// Copyright 2018 The Rio Advancement Inc
//

// Import necessary types from crates.

use bodyparser;
use iron::prelude::*;
use iron::Handler;
use router::Router;

use api::StorageInternalValue;

use exonum::api::{Api, ApiError};
use exonum::blockchain::{gen_prefix, ApiContext, Blockchain, Service, Transaction};
use exonum::crypto::{Hash, PublicKey, SecretKey};
use exonum::encoding;
use exonum::encoding::serialize::json::reexport as serde_json;
use exonum::messages::{Message, RawTransaction};
use exonum::node::{ApiSender, TransactionSend};
use exonum::storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot};

use protocol::api::audit::Envelope;

// // // // // // // // // // CONSTANTS // // // // // // // // // //

// Define service ID for the service trait.

const SERVICE_ID: u16 = 1;

// Define constants for transaction types within the service.

const TX_CREATE_ENVELOPE_ID: u16 = 1;

// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //

// Declare the data to be stored in the blockchain. In the present case,
// declare a type for storing information about the wallet and its balance.

/// Declare a [serializable][1] struct and determine bounds of its fields
/// with `encoding_struct!` macro.
///
/// [1]: https://exonum.com/doc/architecture/serialization
encoding_struct! {
    struct StorageValueEnvlProposeData {
        tx_propose: TxCreateEnvelope,
    }
}

/// Blockchain's account height (account_id).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountHeight(pub String);

/// If the account id isnot an integer, then we consider it to be a system level one
/// eg: rioos_system numbered as 0001.
impl From<AccountHeight> for u64 {
    fn from(val: AccountHeight) -> Self {
        val.0.parse::<u64>().unwrap_or(00001)
    }
}

// // // // // // // // // // DATA LAYOUT // // // // // // // // // //

/// Create schema of the key-value storage implemented by `RocksDB`. In the
/// present case a `Fork` of the database is used.

pub struct HabitatSchema<T> {
    view: T,
}

/// Declare layout of the data. Use an instance of [`ProofMapIndex`]
/// to keep audits in storage. Index values are serialized `Wrapped Envelope Tx - StorageValueEnvlProposeData` structs.
///
impl<T> HabitatSchema<T>
where
    T: AsRef<Snapshot>,
{
    pub fn new(snapshot: T) -> HabitatSchema<T> {
        HabitatSchema { view: snapshot }
    }

    /// Returns table that represents a map of hashed event
    /// Refer  propose_data_by_audit_hash_mut
    pub fn propose_data_by_audit_hash(
        &self,
    ) -> ProofMapIndex<&T, Hash, StorageValueEnvlProposeData> {
        ProofMapIndex::new("habitat.audit_proposes", &self.view)
    }

    /// Returns table that keeps a list of transactions for the each account.
    /// Refer  audit_txs_mut
    pub fn audit_txs(&self, aheight: AccountHeight) -> ProofListIndex<&T, Hash> {
        let height: u64 = aheight.into();
        ProofListIndex::with_prefix("habitat.audit_txs", gen_prefix(&height), &self.view)
    }

    /// Get a list of audit for an account height from the storage.
    #[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
    pub fn audits_by_account(&self, aheight: AccountHeight) -> Vec<Option<Envelope>> {
        let envelopes_table = self.audit_txs(aheight.clone());

        let result = envelopes_table
            .into_iter()
            .map(|envl_hash| {
                self.propose_data_by_audit_hash().get(&envl_hash).map(|p| {
                    let cfg = <Envelope as StorageInternalValue>::from_bytes(
                        p.tx_propose().envl().as_bytes().into(),
                    );
                    cfg
                })
            })
            .collect();

        result
    }

    /// Returns the root_hash
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.propose_data_by_audit_hash().root_hash()]
    }
}

impl<'a> HabitatSchema<&'a mut Fork> {
    /// Mutable version of `propose_data_by_audit_hash` index.
    /// [(hash1, storage_value_envl)]
    /// This is a map of every hashed envelop key has its value of StorageValueProposedData (envelope)
    pub fn propose_data_by_audit_hash_mut(
        &mut self,
    ) -> ProofMapIndex<&mut Fork, Hash, StorageValueEnvlProposeData> {
        ProofMapIndex::new("habitat.audit_proposes", &mut self.view)
    }

    // Mutable reference to the audit_txs
    // This is per list for every account that stores as follows.
    // [accountid, [hash1, hash2]] where hash1, hash2 are the hash keys for
    // accessing the envelop data from hasbitat.audit_proposes map.
    fn audit_txs_mut(&mut self, aheight: AccountHeight) -> ProofListIndex<&mut Fork, Hash> {
        let height: u64 = aheight.into();
        ProofListIndex::with_prefix("habitat.audit_txs", gen_prefix(&height), &mut self.view)
    }

    /// Load the Envelope from the bytes stored in TxCreateEnvelope using accessor envl()
    /// Create a hash of Envelope
    /// Create a wrapper for storing TxCreateEnvelope as StorageValueEnvlProposeData
    /// Put  [hash of envelope, StorageValueEnvlProposeData] in ProofMapIndex
    /// Push [hash_of envelope] into ProofListIndex(account_id)
    /// Refer propose_data_audit_hash_mut, audit_txs_mut for more information.
    pub fn push_audit(&mut self, tx_propose: TxCreateEnvelope) -> bool {
        let cfg =
            <Envelope as StorageInternalValue>::from_bytes(tx_propose.envl().as_bytes().into());
        let cfg_hash = &StorageInternalValue::hash(&cfg);

        let propose_data_by_audit = StorageValueEnvlProposeData::new(tx_propose);

        {
            let mut propose_data_by_audit_hash_table = self.propose_data_by_audit_hash_mut();
            debug_assert!(propose_data_by_audit_hash_table.get(cfg_hash).is_none());
            propose_data_by_audit_hash_table.put(cfg_hash, propose_data_by_audit);
        }

        self.audit_txs_mut(AccountHeight(cfg.get_account()))
            .push(*cfg_hash);
        true
    }
}

// // // // // // // // // // TRANSACTIONS // // // // // // // // // //

/// Create a new envelope.
/// pub_key: of the storer.
/// envl: The Envelope in byte
/// secret_key: The secret key of the storer *This is fields isn't
/// visible here, but handled in the macro message!
message! {
    struct TxCreateEnvelope {
        const TYPE = SERVICE_ID;
        const ID = TX_CREATE_ENVELOPE_ID;

        pub_key:     &PublicKey,
        envl:        &str,
    }
}

// // // // // // // // // // CONTRACTS // // // // // // // // // //

/// Execute a transaction.
impl Transaction for TxCreateEnvelope {
    /// Verify integrity of the transaction by checking the transaction
    /// signature.
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    /// Apply logic to the storage when executing the transaction.
    fn execute(&self, view: &mut Fork) {
        let mut schema = HabitatSchema::new(view);

        schema.push_audit(self.clone());
    }
}

// // // // // // // // // // REST API // // // // // // // // // //

/// Implement the node API.
#[derive(Clone)]
struct HabitatApi {
    channel: ApiSender,
    blockchain: Blockchain,
    public_key: PublicKey,
    secret_key: SecretKey,
}

/// The structure returned by the REST API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponseEnvelopePost {
    pub tx_envl_hash: Hash,
    pub envl_hash: Hash,
}

/// Shortcut to get data on audits from habitat.
impl HabitatApi {
    /// Endpoint for dumping all audits from the storage.
    fn get_audits(&self, req: &mut Request) -> IronResult<Response> {
        let view = self.blockchain.fork();
        let schema = HabitatSchema::new(view);

        let id: Result<String, ApiError> = {
            match req.extensions.get::<Router>().unwrap().find("account_id") {
                Some(account) => match account.parse::<u64>() {
                    Ok(account) => Ok(account.to_string()),
                    Err(_) => {
                        return Err(ApiError::IncorrectRequest(
                            "account_id must be numeric".into(),
                        ))?
                    }
                },
                None => return Err(ApiError::IncorrectRequest("Empty account_id".into()))?,
            }
        };

        let envls: Vec<Option<Envelope>> = schema.audits_by_account(AccountHeight(id?));

        self.ok_response(&serde_json::to_value(&envls).unwrap())
    }

    ///Returns two hashes
    /// - transaction envelope as hash
    /// - envelope as hash
    /// Creates a TxCreateEnvelope with the service public and secret key along with
    ///         envelope creates as bytes.
    fn post_audit_tx(&self, envl: Envelope) -> Result<ApiResponseEnvelopePost, ApiError> {
        warn!("» ※ » POST AUDIT TX === ✔");

        let envl_hash = envl.hash();

        let tx_envl = TxCreateEnvelope::new(
            &self.public_key,
            &String::from_utf8(envl.into_bytes().as_slice().to_vec()).unwrap(),
            &self.secret_key,
        );

        let tx_envl_hash = tx_envl.hash();

        let ch = self.channel.clone();
        warn!("» ※ » POST AUDIT TX SENT TO CHANNEL === ✔");
        ch.send(Box::new(tx_envl))?;

        let res = ApiResponseEnvelopePost {
            tx_envl_hash: tx_envl_hash,
            envl_hash: envl_hash,
        };

        Ok(res)
    }
}

/// Implement the `Api` trait.
/// `Api` facilitates conversion between transactions/read requests and REST
/// endpoints; for example, it parses `POSTed` JSON into the binary transaction
/// representation used in Exonum internally.
impl Api for HabitatApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_audit = move |req: &mut Request| -> IronResult<Response> {
            warn!("» ※ » POST AUDIT STARTS === ✔");
            match req.get::<bodyparser::Struct<Envelope>>() {
                Ok(Some(envl)) => {
                    warn!("» ※ » POST AUDIT PARSED === ✔");
                    let info = self_.post_audit_tx(envl)?;
                    self_.ok_response(&serde_json::to_value(info).unwrap())
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };

        let self_ = self.clone();
        let get_audits = move |req: &mut Request| self_.get_audits(req);

        // Bind handlers to specific routes.
        router.post("/v1/audits", post_audit, "post_audit");

        router.get("/v1/accounts/:account_id/audits", get_audits, "get_audits");

        //
    }
}

// // // // // // // // // // SERVICE DECLARATION // // // // // // // // // //

/// Define the service.
pub struct Habitat;

/// Implement a `Service` trait for the service.
impl Service for Habitat {
    fn service_name(&self) -> &'static str {
        "habitat"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    /// Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let trans: Box<Transaction> = match raw.message_type() {
            TX_CREATE_ENVELOPE_ID => Box::new(TxCreateEnvelope::from_raw(raw)?),
            _ => {
                return Err(encoding::Error::IncorrectMessageType {
                    message_type: raw.message_type(),
                });
            }
        };
        Ok(trans)
    }

    /// `Habitat` returns a vector, containing the single [root_hash][1]
    /// of [all audit table]
    /// Thus, `state_hash` is affected by any new valid audit event added.
    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        let schema = HabitatSchema::new(snapshot);
        schema.state_hash()
    }

    /// Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = HabitatApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
            public_key: ctx.public_key().clone(),
            secret_key: ctx.secret_key().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}
