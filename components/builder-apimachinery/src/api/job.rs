// Copyright 2018 The Rio Advancement Inc

use api::base::{TypeMeta, ObjectMeta, Status, MetaFields, WhoAmITypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Jobs {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta, //  owner_refered by <assembly_id> (or) <backup_id> (or) <service_id>
    spec: SpecData,
    status: Status,
    #[serde(default)]
    created_at: String,
}

impl Jobs {
    pub fn new() -> Jobs {
        ::std::default::Default::default()
    }

    //Create a new jobs with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Jobs {
        Jobs {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_spec(&mut self, v: SpecData) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &SpecData {
        &self.spec
    }
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }
    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl WhoAmITypeMeta for Jobs {
    const MY_KIND: &'static str = "POST:jobs";
}

impl MetaFields for Jobs {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SpecData {
    node_id: String, //applicable node for this job. this job will be scheduler allocate in the specific node id. incase node id is invalid no one can receive this job.
    group: String,   //`group` represents a grouping of jobs that applies to. The supported groupings are `assembly, service, backup,snapshot`
    /*action represents the requested action to be performed by the job.
    The supported actions for group assembly are `deploy,start, stop,reboot,forcestop,delete,remove, releasecompute`,
    service are `deploy,start, stop,reboot,forcestop,delete,remove, releasecompute`,
    backup are `create, delete,
    shapshot'for 'create, delete revert'*/
    action: String,
}

impl SpecData {
    pub fn with(n: &str, g: &str, a: &str) -> SpecData {
        SpecData {
            node_id: n.to_string(),
            group: g.to_string(),
            action: a.to_string(),
            ..Default::default()
        }
    }
    pub fn get_node_id(&self) -> ::std::string::String {
        self.node_id.clone()
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_job() {
        let val = r#"{
            "type_meta":{"kind": "Job", "api_version": "V1"},
            "object_meta":{"name":"loadbalancer","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"877634565345","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"finalizers":[],"cluster_name":""},
            "status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},
            "spec": {"node_id": "8762345665434","group":"assembly","action": "deploy" }
    }"#;
        let job: Jobs = json_decode(val).unwrap();
        assert_eq!(job.id, "");
        assert_eq!(job.created_at, "");
    }

    #[test]
    fn decode_job_with_id_and_created_at() {
        let val = r#"{
            "id": "8765431234567",
            "type_meta":{"kind": "Job", "api_version": "V1"},
            "object_meta":{"name":"loadbalancer","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"877634565345","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"finalizers":[],"cluster_name":""},
            "status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},
            "spec": {"node_id": "8762345665434","group":"assembly","action": "deploy" },
            "created_at" :"2017-11-29T12:59:06.809333+00:00"
    }"#;
        let job: Jobs = json_decode(val).unwrap();
        assert_eq!(job.id, "8765431234567");
        assert_eq!(job.created_at, "2017-11-29T12:59:06.809333+00:00");
    }

    fn decode_spec_data() {
        let val = r#"{
            "node_id": "8762345665434",
            "group":"assembly",
            "action": "deploy"
    }"#;
        let spec: SpecData = json_decode(val).unwrap();
        assert_eq!(spec.node_id, "8762345665434");
        assert_eq!(spec.group, "assembly");
        assert_eq!(spec.action, "deploy");
    }

}
