// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, Status, TypeMeta};

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    #[serde(default)]
    meta_data: BTreeMap<String, String>,
    plans: Vec<PlanProperties>,
    #[serde(default)]
    created_at: String,
    category: String,
    version: String,
    icon: String,
    description: String,
    status: Status,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PlanProperties {
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
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
    stateful_volumes: Vec<StatefulVolume>,
    #[serde(default)]
    envs: BTreeMap<String, Envs>,
    /*`Lifecycle` describes actions that the management system should take in response to lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the machine/container blocks until the action is complete, unless the machine/container process fails, in which case the handler is aborted.
    There are two hooks that are exposed to Machines/Containers:
    PostStart
    This hook executes immediately after a machine/container is created. In case of containers, however, there is no guarantee that the hook will execute before the container ENTRYPOINT. No parameters are passed to the handler.
    PreStop
    This hook is called immediately before a machine/container is terminated. It is blocking, meaning it is synchronous, so it must complete before the call to delete the machine/container can be sent. No parameters are passed to the handler.*/
    #[serde(default)]
    lifecycle: LifeCycle,
    #[serde(default)]
    status: Status, //`status` : <<old status definition>> Indicates if the plan can be used are not. Default no status is available. Will be turned on when the rio.marketplace syncer gets active.
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

impl MetaFields for PlanProperties {
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

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_plan(&mut self, v: Vec<PlanProperties>) {
        self.plans = v;
    }

    pub fn get_plan(&self) -> Vec<PlanProperties> {
        self.plans.clone()
    }

    pub fn get_category(&self) -> ::std::string::String {
        self.category.clone()
    }

    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = v;
    }

    pub fn get_icon(&self) -> ::std::string::String {
        self.icon.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    pub fn get_version(&self) -> ::std::string::String {
        self.version.clone()
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

    pub fn set_meta_data(&mut self, v: BTreeMap<String, String>) {
        self.meta_data = v;
    }

    pub fn get_meta_data(&self) -> &BTreeMap<String, String> {
        &self.meta_data
    }
}

impl PlanProperties {
    pub fn new() -> PlanProperties {
        ::std::default::Default::default()
    }

    pub fn set_characteristics(&mut self, v: BTreeMap<String, String>) {
        self.characteristics = v;
    }

    pub fn get_characteristics(&self) -> &BTreeMap<String, String> {
        &self.characteristics
    }

    pub fn get_version(&self) -> ::std::string::String {
        self.version.clone()
    }

    pub fn get_category(&self) -> ::std::string::String {
        self.category.clone()
    }
    pub fn get_stateful_volumes(&self) -> &Vec<StatefulVolume> {
        &self.stateful_volumes
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Port {
    container_port: i32, //container port
    host_ip: String,     //ip address for the host
    host_port: i32,      //port of the host
    protocol: String,    //plan protocol type like tcp or udp
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
    value: String,    //The default value as in the blueprint plan
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
    #[serde(default)]
    pre_stop: Command,
    #[serde(default)]
    post_start: Command,
    #[serde(default)]
    probe: Probe,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Probe {
    tcp_socket: TcpSocket, // TCPSocket specifies an action involving a TCP port implement a realistic TCP lifecycle hook
    exec: Vec<String>, //One and only one of the following should be specified. Exec specifies the action to take.  optional
    http_get: HttpGet, // HTTPGet specifies the http request to perform.
    http_headers: BTreeMap<String, String>, //HTTPHeader describes a custom header to be used in HTTP probes
    env: BTreeMap<String, String>, // EnvVar represents an environment variable present in a Container.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TcpSocket {
    port: String, // Port to connect to.
    host: String, //Host name to connect to, defaults to the pod IP.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HttpGet {
    path: String,   //Path to access on the HTTP server.
    port: String,   //Name or number of the port to access on the container.
    host: String, // Host name to connect to, defaults to the pod IP. probably want to set "Host" in httpHeaders instead.
    scheme: String, //Scheme to use for connecting to the host, defaults to HTTP.
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Volumes {
    host_path: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct VolumeMounts {
    mount_path: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StatefulVolume {
    pub name: String,
    volumes: Volumes,
    volume_mounts: VolumeMounts,
    #[serde(default)]
    settingmap: SettingMap,
}
impl StatefulVolume {
    pub fn get_settingmap(&self) -> &SettingMap {
        &self.settingmap
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SettingMap {
    #[serde(default)]
    uri: String,
    #[serde(default)]
    uid: String,
    #[serde(default)]
    rioos_binder: Vec<String>,
    map_type: String,
}

impl SettingMap {
    pub fn set_uri(&mut self, v: String) {
        self.uri = v;
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_plan_factory() {
        let val = r#"{"object_meta":{
             "name":"ubuntu",
             "account":""
             },
             "plans":[{
                 "object_meta":{
                     "name":"ubuntu",
                     "account":"",
                     "owner_references":[{
                         "kind":"Package",
                         "api_version":"v1",
                         "name":"ubuntu",
                         "uid":"109876543212345678",
                         "block_owner_deletion":false
                         }]
                    },
                 "category": "machine",
                 "version": "16.04",
                 "characteristics" :{
                     "rioos_sh_image_extension": "img",
                     "rioos_sh_market_image_extension":  "tar.gz"
                     },
                 "icon" : "ubuntu.png",
                 "description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
                 "status":{
                     "phase":"SyncPending"
                     },
                 "metadata": {"origin": "rioos_system"},
                 "lifecycle":{
                     "probe": {
                         "env": {},
                         "exec": [],
                         "http_get": {
                             "host": "",
                             "path": "",
                             "port": "",
                             "scheme": ""
                             },
                         "tcp_socket": {"host": "", "port": ""},
                         "http_headers": {}
                         },
                     "pre_stop": {
                         "command": []},
                     "post_start": {"command": []}
                     }
                }],
             "category": "machine",
             "version": "16.04",
             "icon": "ubuntu.png",
             "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
             "status":{"phase":"SyncPending"}
         }"#;
        let plan: Plan = json_decode(val).unwrap();
        assert_eq!(plan.category, "machine");
        assert_eq!(plan.version, "16.04");
        assert_eq!(plan.icon, "ubuntu.png");
    }
    #[test]
    fn decode_probe() {
        let probe_val = r#"{
            "tcp_socket" :
                {
                "port": "8080",
                "host": "console.rioos.xyz"
                },
            "exec": ["cat","/tmp/health"],
            "http_get":
                {
                    "path": "/healthz",
                    "port": "8080",
                    "host": "console.rioos.xyz",
                    "scheme": "http"
                },
            "http_headers": {
                "X-Custom-Header": "Awesome"
                },
            "env": {}
            }"#;
        let probe: Probe = json_decode(probe_val).unwrap();
        assert!(probe.http_headers.contains_key("X-Custom-Header"));
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
