use std::collections::BTreeMap;
use std::fmt;

use api::deploy::PHASE_PENDING;
use chrono;
use chrono::prelude::*;
use chrono_humanize;

// These are internal finalizer values for rioos-like APIs, must be qualified name
// unless defined here

const FINALIZER_ORPHAN_DEPENDENTS: &'static str = "orphan";
// const FINALIZER_DELETE_DEPENDENTS: &'static str = "foregroundDeletion";

/// TypeMeta describes an individual object in an API response or request
// with strings representing the type of the object and its API schema version.
// Structures that are versioned or persisted should inline TypeMeta.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TypeMeta {
    // Kind is a string value representing the REST resource this object represents.
    // Servers may infer this from the endpoint the client submits requests to.
    // Cannot be updated.
    // In CamelCase.
    pub kind: String,
    // APIVersion defines the versioned schema of this representation of an object.
    // Servers should convert recognized schemas to the latest internal value, and
    // may reject unrecognized values.
    pub api_version: String,
}

// Move it to SchemaRegistry that will build TypeMeta for the SchemaRegistry::current() schema
impl TypeMeta {
    pub fn new() -> TypeMeta {
        ::std::default::Default::default()
    }

    pub fn with(kind: String, version: String) -> TypeMeta {
        TypeMeta {
            kind: kind,
            api_version: version,
        }
    }
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
///This is essentially ObjectMetaMutator, mutates as per the following rules.
/// CREATE:
/// ------
/// When an object is created the name, origin, created_at, DELETION_GRACE_PERIOD_SECONDS are added for all the
/// api resources.
///- cluster_name (datacenter) is provided then, its added as cluster_name
/// When the cluster_name (datacenter) is empty, which can happen for resources like /datacenter, we leave it empty.
/// - origin: Every user has their own origin with a default created during their signup.
/// For the cluster management api's the OriginSystem attached to an administrative userid is  created.
/// UPDATE:
/// ------
/// When an object is updated the fields that can be updated are
/// - labels (This does a merge of the keys and removes the ones that doesn't exist)
/// - annotations (This does a merge of the keys and removes the ones that doesn't exist)
/// DELETE:
/// ------
/// When an object is deleted the fields the deleted_at is set and a garbage collection is performed.
/// - deleted_at
/// - finalizers (This does a merge of the keys and removes the ones that doesn't exist)
///   All the finalizers are run before garbage collection.
/// - owner_references:   All the owner references are nuked and the children of the owner references are posted for deletion
/// We have a dependent cascading delete to be performed before its garbage collected
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectMeta {
    /// Name is unique within a namespace.  Name is required when creating resources, although
    /// some resources may allow a client to request the generation of an appropriate name
    /// automatically. Name is primarily intended for creation idempotence and configuration
    /// definition.
    ///example: A name that appears when creating
    ///                    network, origin, storagepool,
    ///                    datacenter - chennai
    ///                    account - this is going to be email.
    #[serde(default)]
    pub name: String,
    /// Account defines the account within which name must be unique.  This will have the account_id of objects created.
    /// This will be blank for accounts created.
    #[serde(default)]
    pub account: String, //default for node and account
    /// created_at is a timestamp representing the server time when this object was
    /// created. It is not guaranteed to be set in happens-before order across separate operations.
    /// Clients may not set this value. It is represented in RFC3339 form and is in Utc.
    /// Null for lists.
    /// Read-only.    //
    #[serde(default)]
    pub created_at: String,
    /// deleted_at is RFC 3339 date and time at which this resource will be deleted. This
    /// field is set by the api server when a graceful deletion is requested by the user, and is not
    /// directly settable by a client. The resource is expected to be deleted (no longer visible
    /// from resource lists, and not reachable by name) after the time in this field.
    /// As long as the finalizers list contains items, deletion is blocked.
    /// Once the deleted_at is set, this value may not be unset or be set further into the
    /// future, although it may be shortened or the resource may be deleted prior to this time.
    /// For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react
    /// by sending a graceful termination signal to the containers in the pod. After that 30 seconds,
    /// the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup,
    /// remove the pod from the API. In the presence of network partitions, this object may still
    /// exist after this timestamp, until an administrator or automated process can determine the
    /// resource is fully terminated.
    /// If not set, graceful deletion of the object has not been requested.
    ///
    /// Populated by the api when a graceful deletion is requested.
    /// Read-only.
    #[serde(default)]
    pub deleted_at: String,
    /// Number of seconds allowed for this object to gracefully terminate before
    /// it will be removed from the system. Only set when deletionTimestamp is also set.
    /// May only be shortened.
    #[serde(default)]
    pub deletion_grace_period_seconds: u32,
    /// Map of string keys and values that can be used to organize and categorize
    /// (scope and select) objects. May match selectors of replication controllers
    /// and services.
    /// +optional
    #[serde(default)]
    pub labels: BTreeMap<String, String>,

    /// Annotations is an unstructured key value map stored with a resource that may be
    /// set by external tools to store and retrieve arbitrary metadata. They are not
    /// queryable and should be preserved when modifying objects.
    /// +optional
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
    /// List of objects depended by this object. If ALL objects in the list have
    /// been deleted, this object will be garbage collected. If this object is managed by a controller,
    /// then an entry in this list will point to this controller, with the controller field set to true.
    /// There cannot be more than one managing controller.
    /// +optional
    #[serde(default)]
    pub owner_references: Vec<OwnerReferences>,
    /// An initializer is a controller which enforces some system invariant at object creation time.
    /// This field is a list of initializers that have not yet acted on this object. If nil or empty,
    /// this object has been completely initialized. Otherwise, the object is considered uninitialized
    /// and is hidden (in list/watch and get calls) from clients that haven't explicitly asked to
    /// observe uninitialized objects.
    ///
    /// When an object is created, the system will populate this list with the current set of initializers.
    /// Only privileged users may set or modify this list. Once it is empty, it may not be modified further
    /// by any user.
    #[serde(default)]
    pub initializers: Initializers,
    /// Must be empty before the object is deleted from the registry. Each entry
    /// is an identifier for the responsible component that will remove the entry
    /// from the list. If the deletionTimestamp of the object is non-nil, entries
    /// in this list can only be removed.
    /// +optional
    /// +patchStrategy=merge
    #[serde(default)]
    pub finalizers: Vec<String>,

    /// The name of the cluster(datacenter/location) which the object belongs to.
    /// This is used to distinguish resources with same name and namespace in different clusters.
    /// This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
    #[serde(default)]
    pub cluster_name: String,
}

impl ObjectMeta {
    pub fn new() -> ObjectMeta {
        ::std::default::Default::default()
    }
}

/// Trait for all types that could be an object in RIO/OS.
pub trait MetaFields {
    ///  Implementing structure will set the type_meta and object_meta
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta);

    ///  Implementing structure will send their metadata
    fn object_meta(&self) -> ObjectMeta;

    //Return the typemeta of your object
    fn type_meta(&self) -> TypeMeta;

    //Returns a new ObjectMeta.
    fn mut_meta(&self, mut current: ObjectMeta, name: String, account: String) -> ObjectMeta {
        current.name = name;
        current.account = account;
        current.created_at = Utc::now().to_rfc3339();
        current.finalizers = vec![FINALIZER_ORPHAN_DEPENDENTS.to_string()];
        current.deletion_grace_period_seconds = 30;
        current
    }

    ////Field management for ObjectMeta
    ///
    fn get_name(&self) -> String {
        self.object_meta().name.clone()
    }

    fn set_name(&self, name: String) {
        self.object_meta().name = name
    }

    fn get_account(&self) -> String {
        self.object_meta().account.clone()
    }

    fn set_account(&self, account: String) {
        self.object_meta().account = account
    }

    fn get_deleted_at(&self) -> String {
        self.object_meta().deleted_at.clone()
    }

    //Update the deletion timestamp for the existing object metadata object.
    fn set_deleted_at(&self, deleted_at: String) {
        self.object_meta().deleted_at = deleted_at
    }

    fn get_cluster_name(&self) -> String {
        self.object_meta().cluster_name.clone()
    }

    fn set_cluster_name(&self, current: &mut ObjectMeta, cluster_name: String) {
        current.cluster_name = cluster_name
    }

    fn get_deletion_grace_period_seconds(&self) -> u32 {
        self.object_meta().deletion_grace_period_seconds.clone()
    }

    fn set_deletion_grace_period_seconds(&self, deletion_grace_period_seconds: u32) {
        self.object_meta().deletion_grace_period_seconds = deletion_grace_period_seconds;
    }

    fn get_labels(&self) -> BTreeMap<String, String> {
        self.object_meta().labels.clone()
    }

    fn set_labels(&self, labels: BTreeMap<String, String>) {
        self.object_meta().labels = labels
    }

    fn get_annotations(&self) -> BTreeMap<String, String> {
        self.object_meta().annotations.clone()
    }

    fn set_annotations(&self, annotations: BTreeMap<String, String>) {
        self.object_meta().annotations = annotations
    }

    fn get_initializers(&self) -> Initializers {
        self.object_meta().initializers.clone()
    }

    fn set_initializers(&self, initializers: Initializers) {
        self.object_meta().initializers = initializers
    }

    fn get_finalizers(&self) -> Vec<String> {
        self.object_meta().finalizers.clone()
    }

    fn set_finalizers(&self, finalizers: Vec<String>) {
        self.object_meta().finalizers = finalizers
    }

    fn get_owner_references(&self) -> Vec<OwnerReferences> {
        self.object_meta().owner_references.clone()
    }

    fn set_owner_reference(
        &self,
        current: &mut ObjectMeta,
        kind: String,
        api_version: String,
        name: String,
        uid: String,
    ) {
        current
            .owner_references
            .push(OwnerReferences::with(kind, api_version, name, uid));
    }

    // fn set_owner_references(&self, owner_references: Vec<OwnerReferences>) {
    //     self.object_meta().owner_references = owner_references
    // }

    fn get_created_at(&self) -> String {
        self.object_meta().created_at.clone()
    }
    fn set_created_at(&self, created_at: String) {
        self.object_meta().created_at = created_at
    }   
}

///Trait for all types that could be a child with typemeta
///Parents must implement this trait to say the TypeMeta of their child.
///
///   eg:
///      Impl ChildTypeMeta for AssemblyFactory {
///                 const CHILD_KIND  "GET:/assemblys"
///       }
///
pub trait ChildTypeMeta {
    const CHILD_KIND: &'static str;

    fn children(&self) -> String {
        Self::CHILD_KIND.to_string()
    }
}

///Trait for all types that want to identity itself with typemeta
///The self objects must implement this trait to say the TypeMeta of their own.
///
///   eg:
///      Impl WhoAmI for Services {
///                 const CHILD_KIND  "GET:/services"
///      }
///
pub trait WhoAmITypeMeta {
    const MY_KIND: &'static str;

    fn who_am_i(&self) -> String {
        Self::MY_KIND.to_string()
    }
}

/// OwnerReference contains enough information to let you identify an owning
/// object. Currently, an owning object must be in the same origiin, so there
/// is no origin field.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OwnerReferences {
    /// Kind of the referent.
    pub kind: String,
    /// API version of the referent.
    pub api_version: String,
    /// Name of the referent.
    pub name: String,
    /// UID of the referent.
    pub uid: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then
    /// the owner cannot be deleted from the key-value store until this
    /// reference is removed.
    /// Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner,
    /// otherwise 422 (Unprocessable Entity) will be returned.
    pub block_owner_deletion: bool,
}

impl OwnerReferences {
    pub fn new() -> OwnerReferences {
        ::std::default::Default::default()
    }
    fn with(kind: String, api_version: String, name: String, uid: String) -> OwnerReferences {
        OwnerReferences {
            kind: kind,
            api_version: api_version,
            uid: uid,
            block_owner_deletion: false,
            name: name,
        }
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }

    pub fn get_uid(&self) -> ::std::string::String {
        self.uid.clone()
    }
    pub fn set_block_owner_deletion(&mut self, v: bool) {
        self.block_owner_deletion = v;
    }
}

/// ObjectReference contains enough information to let you inspect
/// or modify the referred object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReference {
    /// Kind of the referent.
    /// +optional
    #[serde(default)]
    kind: String,
    /// Origin of the referent.
    /// +optional
    #[serde(default)]
    origin: String,
    /// Name of the referent.
    /// +optional
    name: String,
    /// UID of the referent.
    #[serde(default)]
    uid: String,
    /// API version of the referent.
    #[serde(default)]
    api_version: String,
    /// Specific resourceVersion to which this reference is made, if any.
    #[serde(default)]
    resource_version: String,

    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a assemblyfactory, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this assemblyfactory). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default)]
    field_path: String,
}

/// Status is a return value for calls that don't return other objects.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    /// The phase of a rest resource (Assembly, Job, Node...) is a simple, high-level summary of
    /// where the rest resoruce is in its lifecycle. The phase is not intended to be a comprehensive
    /// rollup of observations of Assembly or rest resource state, nor is it intended to be a comprehensive state machine.
    /// Current condition of the rest resource or assembly.
    pub phase: String,
    ///  A human-readable description of the status of this operation.
    #[serde(default)]
    pub message: String,
    /// A machine-readable description of why this operation is in the
    /// "Failure" status. If this value is empty there
    /// is no information available. A Reason clarifies an HTTP status
    /// code but does not override it.
    //  A brief CamelCase message indicating details about why the assembly is in this state.
    /// e.g. 'Evicted'
    #[serde(default)]
    pub reason: String,
    /// Conditions is an array of current condition(types).
    /// +optional
    #[serde(default)]
    pub conditions: Vec<Condition>,
}

impl Status {
    pub fn with_conditions(
        phase: &str,
        message: &str,
        reason: &str,
        conditions: Vec<Condition>,
    ) -> Status {
        Status {
            phase: phase.to_string(),
            message: message.to_string(),
            reason: reason.to_string(),
            conditions: conditions,
        }
    }

    pub fn get_phase(&self) -> ::std::string::String {
        self.phase.clone()
    }

    /// Use this to indicate an empty Phase: Pending.
    /// For more customized usage, try with_conditions()
    pub fn pending() -> Status {
        Status {
            phase: PHASE_PENDING.to_string(),
            message: "".to_string(),
            reason: "".to_string(),
            conditions: vec![],
        }
    }
}

/// The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub status: Status,
    #[serde(default)]
    id: String,
}

impl StatusUpdate {
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }
}
/// Common condition request used across the api requests
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct Condition {
    /// Type of condition.
    pub condition_type: String,
    /// A human readable message indicating details about the transition.
    pub message: String,
    /// The reason for the condition's last transition. (camelcased reason like NotReadyNode)
    pub reason: String,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// The last time this condition was updated.
    pub last_update_time: String,
    /// Last time the condition transitioned from one status to another.
    pub last_transition_time: String,
    /// Last time the condition was checked.
    pub last_probe_time: String,
}

impl Condition {
    pub fn with_type(
        condition_type: &str,
        message: &str,
        reason: &str,
        status: &str,
        last_transition_time: &str,
        last_probe_time: &str,
    ) -> Condition {
        Condition {
            condition_type: condition_type.to_string(),
            status: status.to_string(),
            reason: reason.to_string(),
            message: message.to_string(),
            last_transition_time: last_transition_time.to_string(),
            last_update_time: last_transition_time.to_string(),
            last_probe_time: last_probe_time.to_string(),
        }
    }

    pub fn get_condition_type(&self) -> ::std::string::String {
        self.condition_type.clone()
    }
    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
    }
    pub fn get_reason(&self) -> ::std::string::String {
        self.reason.clone()
    }
    pub fn get_last_transition_time(&self) -> ::std::string::String {
        self.last_transition_time.clone()
    }
}

/// The IdGet is used by the api handlers to formulate the id or name
/// to load from the database.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IdGet {
    id: String,
    name: String,
}

/// The IdGet by default is empty.
/// Can be called with an id and name
impl IdGet {
    pub fn new() -> IdGet {
        ::std::default::Default::default()
    }
    pub fn with_id(id: String) -> IdGet {
        IdGet {
            id: id,
            name: "".to_string(),
        }
    }

    pub fn with_id_name(id: String, name: String) -> IdGet {
        IdGet { id: id, name: name }
    }

    pub fn with_account(name: String) -> IdGet {
        IdGet {
            id: "".to_string(),
            name: name,
        }
    }
    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
}

impl fmt::Display for IdGet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IdGet => ({}) ({})", self.get_id(), self.get_name())
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Initializers {
    /// Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers
    /// struct will be set to nil and the object is considered as initialized and visible to all clients.
    pub pending: Vec<Initializer>,
    /// If result is set with the Failure field, the object will be persisted to storage and then deleted,
    /// ensuring that other clients can observe the deletion.
    pub result: InitializerStatus,
}

impl Initializers {
    pub fn has_pending(&self, name: String) -> bool {
        self.pending
            .iter()
            .filter(|f| f.name == name)
            .next()
            .is_some()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Initializer {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct InitializerStatus {
    pub type_meta: TypeMeta,
    /// Status of the operation.
    /// One of: "Success" or "Failure"
    pub status: String,
    /// A human-readable description of the status of this operation.
    pub message: String,
    /// A machine-readable description of why this operation is in the "Failure" status.
    /// If this value is empty there is no      information available. A Reason clarifies an HTTP status  code but does not override it.
    pub reason: String,
    /// StatusDetails is a set of additional properties that MAY be set by the
    /// server to provide additional information about a response.
    pub details: StatusDetails,
    /// Suggested HTTP return code for this status, 0 if not set.
    pub code: u32,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StatusDetails {
    /// The name attribute of the resource associated with the status StatusReason
    pub name: String,
    /// The group attribute of the resource associated with the status StatusReason.
    pub group: String,
    /// The kind attribute of the resource associated with the status StatusReason.
    pub kind: String,
    /// The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes.
    pub causes: Vec<StatusCause>,
    /// UID of the resource
    pub uid: String,
    /// If specified, the time in seconds before the operation should be retried.
    pub retry_after_seconds: u32,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StatusCause {
    /// A cause type for the error.
    /// If this value is empty there is no information available.
    pub cause_type: String,
    /// A human-readable description of the cause of the error.
    /// This field may be presented as-is to a reader.
    pub message: String,
    /// The field of the resource that has caused this error, as named by its JSON serialization.
    /// May include dot and postfix notation for nested attributes."name"
    /// the field "name" on the current resource  "items[0].name" - the field "name" on the first array entry in "items".
    pub field: String,
}

///QueryParams parsed from URL is stored as map.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct QueryInput {
    /// Query parms sent in the URL is stored as a map by
    /// QueryVerifier.
    pub labels: BTreeMap<String, String>,
}

impl QueryInput {
    pub fn with(labels: BTreeMap<String, String>) -> QueryInput {
        QueryInput { labels: labels }
    }

    pub fn get(&self, key: &str) -> String {
        self.labels.get(key).unwrap_or(&"".to_string()).to_string()
    }
}

pub fn hours_ago(time: String) -> String {
    let now_time = DateTime::parse_from_rfc3339(&Utc::now().to_rfc3339().to_string()).unwrap();
    let time_stamp = DateTime::parse_from_rfc3339(&time.to_string()).unwrap();
    let diff = now_time.timestamp() - time_stamp.timestamp();

    let dt = chrono::Local::now() - chrono::Duration::seconds(diff);
    let ht = chrono_humanize::HumanTime::from(dt);
    ht.to_string()
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_typemeta() {
        let val = r#"{"kind": "AssemblyFactory", "api_version": "v1"}"#;
        let type_meta: TypeMeta = json_decode(val).unwrap();
        assert_eq!(type_meta.kind, "AssemblyFactory");
        assert_eq!(type_meta.api_version, "v1");
    }

    #[test]
    fn decode_typemeta_without() {
        let val = r#"{}"#;
        let type_meta: TypeMeta = json_decode(val).unwrap();
        assert_eq!(type_meta.kind, "");
        assert_eq!(type_meta.api_version, "");
    }

    #[test]
    fn decode_owner_ref() {
        let val = r#"{
            "kind": "Assembly",
             "api_version": "v1",
             "name": "levi.megam.i0",
             "uid":"8765678765567",
             "block_owner_deletion": false
          }"#;
        let own: OwnerReferences = json_decode(val).unwrap();
        assert_eq!(own.kind, "Assembly");
        assert_eq!(own.name, "levi.megam.i0");
        assert_eq!(own.uid, "8765678765567");
        assert_eq!(own.block_owner_deletion, false);
        assert_eq!(own.api_version, "v1");
    }

    #[test]
    fn decode_conditions() {
        let val = r#"{
            "condition_type": "OutOfDisk",
             "message": "nodelet has sufficient disk space available",
             "reason": "NodeletHasSufficientDisk",
             "status":"False",
             "last_update_time": "2017-09-21T06:35:16Z",
             "last_transition_time": "2017-09-21T06:35:16Z",
             "last_probe_time":"2017-09-21T06:35:16Z"
          }"#;
        let condtn: Condition = json_decode(val).unwrap();
        assert_eq!(condtn.condition_type, "OutOfDisk");
        assert_eq!(
            condtn.message,
            "nodelet has sufficient disk space available"
        );
        assert_eq!(condtn.reason, "NodeletHasSufficientDisk");
        assert_eq!(condtn.status, "False");
        assert_eq!(condtn.last_update_time, "2017-09-21T06:35:16Z");
        assert_eq!(condtn.last_transition_time, "2017-09-21T06:35:16Z");
        assert_eq!(condtn.last_probe_time, "2017-09-21T06:35:16Z");
    }

    #[test]
    fn decode_objectmeta() {
        let val = r#"
            {
                    "name":"levi.megam.io",
                    "account":"8765345676543",
                    "labels":{
                        "rioos_environment":"development",
                        "rioos_category":"machine"
                    },
                    "annotations":{
                        "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                        "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                    },
                    "owner_references":[
                    {
                        "kind":"Assembly",
                        "api_version":"v1",
                        "name":"levi.megam.io",
                        "uid":"0001010",
                        "block_owner_deletion":true
                    }
                    ],
                    "created_at":"2017-11-20T06:49:06.907347+00:00",
                    "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                    "deletion_grace_period_seconds":30,
                    "finalizers":[
                        "orphan"
                        ],
                    "cluster_name":"dc1_torono"
                }"#;

        let meta: ObjectMeta = json_decode(val).unwrap();
        assert_eq!(meta.name, "levi.megam.io");
        assert_eq!(meta.account, "8765345676543");
        assert_eq!(meta.labels.len(), 2);
        assert!(meta.labels.contains_key("rioos_environment"));
        assert!(meta.labels.contains_key("rioos_category"));
        assert_eq!(meta.annotations.len(), 2);
        assert!(
            meta.annotations
                .contains_key("rioos/ruchi.calvincare.org/pickup",)
        );
        assert_eq!(meta.created_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deleted_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deletion_grace_period_seconds, 30);
        assert_eq!(meta.cluster_name, "dc1_torono");
    }

    #[test]
    fn decode_objectmeta_default() {
        let val = r#"{}"#;

        let meta: ObjectMeta = json_decode(val).unwrap();
        assert_eq!(meta.name, "");
        assert_eq!(meta.account, "");
        assert_eq!(meta.labels.len(), 0);
        assert_eq!(meta.owner_references.len(), 0);
        assert_eq!(meta.annotations.len(), 0);
        assert_eq!(meta.created_at, "");
        assert_eq!(meta.deleted_at, "");
        assert_eq!(meta.cluster_name, "");
    }

    #[test]
    fn decode_objectmeta_without_account() {
        let val = r#"
            {
                    "name":"levi.megam.io",
                    "labels":{
                        "rioos_environment":"development",
                        "rioos_category":"machine"
                    },
                    "annotations":{
                        "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                        "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                    },
                    "owner_references":[
                    {
                        "kind":"Assembly",
                        "api_version":"v1",
                        "name":"levi.megam.io",
                        "uid":"0001010",
                        "block_owner_deletion":true
                    }
                    ],
                    "created_at":"2017-11-20T06:49:06.907347+00:00",
                    "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                    "deletion_grace_period_seconds":30,
                    "finalizers":[
                        "orphan"
                        ],
                    "cluster_name":"dc1_torono"
                }"#;

        let meta: ObjectMeta = json_decode(val).unwrap();
        assert_eq!(meta.name, "levi.megam.io");
        assert_eq!(meta.labels.len(), 2);
        assert!(meta.labels.contains_key("rioos_environment"));
        assert!(meta.labels.contains_key("rioos_category"));
        assert_eq!(meta.annotations.len(), 2);
        assert!(
            meta.annotations
                .contains_key("rioos/ruchi.calvincare.org/pickup",)
        );
        assert_eq!(meta.created_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deleted_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deletion_grace_period_seconds, 30);
        assert_eq!(meta.cluster_name, "dc1_torono");
    }

    #[test]
    fn decode_objectmeta_optional() {
        let val = r#"
            {
                    "name":"levi.megam.io",
                    "labels":{
                        "rioos_environment":"development",
                        "rioos_category":"machine"
                    },
                    "annotations":{
                        "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                        "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                    },
                    "owner_references":[
                    {
                        "kind":"Assembly",
                        "api_version":"v1",
                        "name":"levi.megam.io",
                        "uid":"0001010",
                        "block_owner_deletion":true
                    }
                    ],
                    "created_at":"2017-11-20T06:49:06.907347+00:00",
                    "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                    "deletion_grace_period_seconds":30
                }"#;

        let meta: ObjectMeta = json_decode(val).unwrap();
        assert_eq!(meta.name, "levi.megam.io");
        assert_eq!(meta.labels.len(), 2);
        assert!(meta.labels.contains_key("rioos_environment"));
        assert!(meta.labels.contains_key("rioos_category"));
        assert_eq!(meta.annotations.len(), 2);
        assert!(
            meta.annotations
                .contains_key("rioos/ruchi.calvincare.org/pickup",)
        );
        assert_eq!(meta.created_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deleted_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(meta.deletion_grace_period_seconds, 30);
        assert_eq!(meta.finalizers.len(), 0);
        assert_eq!(meta.cluster_name, "");
        assert_eq!(meta.account, "");
    }

    #[test]
    fn decode_status() {
        let val = r#"
        {
            "phase": "pending",
            "message": "",
            "reason": "",
            "conditions": [
            {
                "message": "nodelet has sufficient disk space available",
                "reason": "NodeletHasSufficientDisk",
                "status": "False",
                "last_transition_time": "2017-09-21T06:35:16Z",
                "last_probe_time": "2017-09-21T06:35:16Z",
                "condition_type": "OutOfDisk",
                "last_update_time": "2017-09-21T06:35:16Z"
            }
            ]
        }"#;
        let val: Status = json_decode(val).unwrap();
        assert_eq!(val.phase, "pending");
        assert_eq!(val.message, "");
        assert_eq!(val.reason, "");
    }

    #[test]
    fn decode_idget() {
        let val = r#"{"id": "0001", "name": "test"}"#;
        let reg: IdGet = json_decode(val).unwrap();
        assert_eq!(reg.id, "0001");
        assert_eq!(reg.name, "test");
    }

    #[test]
    fn decode_intializer() {
        let val = r#"{"pending": [{
                        "name": "loadbalancer"
                    }],
                     "result": {
                            "status":"success",
                            "message": "omitempty",
                            "type_meta":{
                                "kind":"",
                                "api_version":""
                            },
                            "reason":"",
                            "code": 400,
                            "details":{
                                "name":"name",
                                "group": "grp",
                                "kind": "",
                                "uid":"",
                                "retry_after_seconds": 30,
                                "causes": [{
                                    "cause_type": "",
                                    "message":"",
                                    "field":""
                                }]
                            }

                     }}"#;
        let initial: Initializers = json_decode(val).unwrap();
        assert_eq!(initial.pending.len(), 1);
        assert_eq!(initial.result.status, "success");
        assert_eq!(initial.result.message, "omitempty");
        assert_eq!(initial.result.reason, "");
        assert_eq!(initial.result.code, 400);
        assert_eq!(initial.result.details.name, "name");
        assert_eq!(initial.result.details.group, "grp");
        assert_eq!(initial.result.details.uid, "");
        assert_eq!(initial.result.details.kind, "");
        assert_eq!(initial.result.details.causes.len(), 1);
        assert_eq!(initial.result.details.retry_after_seconds, 30);
    }

}
