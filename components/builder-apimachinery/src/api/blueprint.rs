// Copyright 2018 The Rio Advancement Inc

use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    category: String, //`Category` represents a plans  relationship to Rio/OS. Valid relationships are  machine, container, application and blockchain.
    version: String, //`version` represents the version of this plan software. example : Ubuntu 14.04
    #[serde(default)]
    characteristics: BTreeMap<String, String>, //`Characteristics` The  additional metadata of the plan.  example `extension: iso`, This says this plan is available as an iso.
    icon: String, //icon: An identifier to represent this plan pictorially. example ubuntu.png.
    description: String, //plan description
    #[serde(default)]
    ports: Vec<Port>, //`ports` The default port numbers available for this plan. example, wordpress app is available in port 80
    //`envs` The required, editable environment variables for a plan. Example: In this example, "RUBY_HOME": {"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true","value":"/home/rails/app","editable":"true"}.
    //RAILS_APP_HOME default home is /var/lib/rioos/railsapp which is a required and editable field.
    #[serde(default)]
    envs: BTreeMap<String, Envs>,
    /*`Lifecycle` describes actions that the management system should take in response to lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the machine/container blocks until the action is complete, unless the machine/container process fails, in which case the handler is aborted.
    There are two hooks that are exposed to Machines/Containers:
    PostStart
    This hook executes immediately after a machine/container is created. In case of containers, however, there is no guarantee that the hook will execute before the container ENTRYPOINT. No parameters are passed to the handler.
    PreStop
    This hook is called immediately before a machine/container is terminated. It is blocking, meaning it is synchronous, so it must complete before the call to delete the machine/container can be sent. No parameters are passed to the handler.*/
    #[serde(default)]
    lifecycle: BTreeMap<String, LifeCycle>,
    #[serde(default)]
    status: Status, //`status` : <<old status definition>> Indicates if the plan can be used are not. Default no status is available. Will be turned on when the rio.marketplace syncer gets active.
    #[serde(default)]
    created_at: String,
}

impl MetaFields for Plan {
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

impl Plan {
    pub fn new() -> Plan {
        ::std::default::Default::default()
    }

    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Plan {
        Plan {
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

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = v;
    }
    pub fn get_icon(&self) -> ::std::string::String {
        self.icon.clone()
    }
    pub fn set_characteristics(&mut self, v: BTreeMap<String, String>) {
        self.characteristics = v;
    }

    pub fn get_characteristics(&self) -> &BTreeMap<String, String> {
        &self.characteristics
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }
    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_category(&mut self, v: ::std::string::String) {
        self.category = v;
    }

    pub fn get_category(&self) -> ::std::string::String {
        self.category.clone()
    }

    pub fn set_ports(&mut self, v: Vec<Port>) {
        self.ports = v;
    }

    pub fn get_ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn set_envs(&mut self, v: BTreeMap<String, Envs>) {
        self.envs = v;
    }

    pub fn get_envs(&self) -> &BTreeMap<String, Envs> {
        &self.envs
    }

    pub fn set_lifecycle(&mut self, v: BTreeMap<String, LifeCycle>) {
        self.lifecycle = v;
    }

    pub fn get_lifecycle(&self) -> &BTreeMap<String, LifeCycle> {
        &self.lifecycle
    }

    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    pub fn get_version(&self) -> ::std::string::String {
        self.version.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Port {
    container_port: i32, //container port
    host_ip: String, //ip address for the host
    host_port: i32, //port of the host
    protocol: String, //plan protocol type like tcp or udp
}

impl Port {
    pub fn new(container_port: i32, host_ip: &str, host_port: i32, protocol: &str) -> Port {
        Port {
            container_port: container_port,
            host_ip: host_ip.to_string(),
            host_port: host_port,
            protocol: protocol.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Envs {
    required: String, // env required or not: Must be there for this launch
    value: String, //The default value as in the blueprint plan
    editable: String, //Can this  field be edited by the user.
}

impl Envs {
    pub fn new(required: &str, value: &str, editable: &str) -> Envs {
        Envs {
            required: required.to_string(),
            value: value.to_string(),
            editable: editable.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct LifeCycle {
    exec: Command,
}
impl LifeCycle {
    pub fn new() -> LifeCycle {
        ::std::default::Default::default()
    }

    pub fn set_exec(&mut self, v: Command) {
        self.exec = v;
    }

    pub fn get_exec(&self) -> &Command {
        &self.exec
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Command {
    command: Vec<String>,
}

impl Command {
    pub fn new() -> Command {
        ::std::default::Default::default()
    }

    pub fn set_command(&mut self, v: Vec<String>) {
        self.command = v;
    }

    pub fn get_command(&self) -> &Vec<String> {
        &self.command
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_plan_factory() {
        let val = r#"{
            "type_meta":{"kind": "PlanFactory", "api_version": "V1"},
            "object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"finalizers":[],"cluster_name":""},
            "category": "application",
            "version": "5.2.0",
                "characteristics" :{"image_pullpolicy": "always","git":"source url"},
                "icon" : "rails.png",
                "description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance",
                "ports": [{
                    "container_port": 80,
                    "host_ip":"192.168.1.10",
                    "host_port": 8001,
                    "protocol":"TCP/UDP"
                }],
                "envs":{
                    "RUBY_HOME":
                    {
                        "required":"true",
                        "value":"/usr/lib/ruby/2.4.9",
                        "editable":"false"
                    },
                    "RAILS_APP_HOME":{
                        "required":"true",
                        "value":"/home/rails/app",
                        "editable":"true"
                    }
                },
                "lifecycle": {
                    "postStart":{
                        "exec":{
                            "command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]
                        }
                    },
                    "preStop": {
                        "exec": {
                            "command": ["/usr/sbin/nginx","-s","quit"]
                        }
                    }
                },
            "status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}
    }"#;
        let plan: Plan = json_decode(val).unwrap();
        assert_eq!(plan.category, "application");
        assert_eq!(plan.version, "5.2.0");
    }

    #[test]
    fn decode_ports() {
        let val = r#"{
            "container_port": 8002,
            "host_ip":"192.168.1.10",
            "host_port": 8001,
            "protocol":"TCP/UDP"
        }"#;
        let ports: Port = json_decode(val).unwrap();
        assert_eq!(ports.container_port, 8002);
        assert_eq!(ports.host_ip, "192.168.1.10");
        assert_eq!(ports.host_port, 8001);
        assert_eq!(ports.protocol, "TCP/UDP");
    }

    #[test]
    fn decode_envs() {
        let val = r#"{
            "required": "true",
            "value": "/usr/lib/ruby/2.4.9",
            "editable": "false"
        }"#;
        let envs: Envs = json_decode(val).unwrap();
        assert_eq!(envs.required, "true");
        assert_eq!(envs.value, "/usr/lib/ruby/2.4.9");
        assert_eq!(envs.editable, "false");
    }

}
