// Copyright (c) 2017 RioCorp Inc.

//! Configuration for a Rio/OS API service

use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use rio_net::config::{PasswordCfg, ShieldCfg, PasswordAuth, ShieldAuth, RouterAddr, RouterCfg};
use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    //
    pub ui: UiCfg,
    //
    pub github: PasswordCfg,
    //RIO Shield
    pub shield: ShieldCfg,
    //
    // Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            routers: vec![RouterAddr::default()],
            ui: UiCfg::default(),
            github: PasswordCfg::default(),
            shield: ShieldCfg::default(),
            events_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl PasswordAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}

impl ShieldAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}


impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

/// Public listening net address for HTTP requests
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,
    pub port: u16,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 9636,
        }
    }
}

impl ToSocketAddrs for HttpCfg {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        match self.listen {
            IpAddr::V4(ref a) => (*a, self.port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, self.port).to_socket_addrs(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub root: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [http]
        listen = "0:0:0:0:0:0:0:1"
        port = 9636

        [ui]
        root = "/some/path"

        [[targets]]
        platform = "windows"
        architecture = "x86_64"

        [[routers]]
        host = "172.18.0.2"
        port = 9632
        heartbeat = 9001

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 9636);
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9632");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.http.port, 9000);
    }
}

/*

const (
	// KubernetesDir is the directory kubernetes owns for storing various configuration files
	KubernetesDir = "/etc/kubernetes"

	ManifestsSubDirName = "manifests"

	CACertAndKeyBaseName = "ca"
	CACertName           = "ca.crt"
	CAKeyName            = "ca.key"

	APIServerCertAndKeyBaseName = "apiserver"
	APIServerCertName           = "apiserver.crt"
	APIServerKeyName            = "apiserver.key"

	APIServerKubeletClientCertAndKeyBaseName = "apiserver-kubelet-client"
	APIServerKubeletClientCertName           = "apiserver-kubelet-client.crt"
	APIServerKubeletClientKeyName            = "apiserver-kubelet-client.key"

	ServiceAccountKeyBaseName    = "sa"
	ServiceAccountPublicKeyName  = "sa.pub"
	ServiceAccountPrivateKeyName = "sa.key"

	FrontProxyCACertAndKeyBaseName = "front-proxy-ca"
	FrontProxyCACertName           = "front-proxy-ca.crt"
	FrontProxyCAKeyName            = "front-proxy-ca.key"

	FrontProxyClientCertAndKeyBaseName = "front-proxy-client"
	FrontProxyClientCertName           = "front-proxy-client.crt"
	FrontProxyClientKeyName            = "front-proxy-client.key"

	AdminKubeConfigFileName             = "admin.conf"
	KubeletKubeConfigFileName           = "kubelet.conf"
	ControllerManagerKubeConfigFileName = "controller-manager.conf"
	SchedulerKubeConfigFileName         = "scheduler.conf"

	// Some well-known users and groups in the core Kubernetes authorization system

	ControllerManagerUser   = "system:kube-controller-manager"
	SchedulerUser           = "system:kube-scheduler"
	MastersGroup            = "system:masters"
	NodesGroup              = "system:nodes"
	NodesClusterRoleBinding = "system:node"

	// Constants for what we name our ServiceAccounts with limited access to the cluster in case of RBAC
	KubeDNSServiceAccountName   = "kube-dns"
	KubeProxyServiceAccountName = "kube-proxy"

	// APICallRetryInterval defines how long kubeadm should wait before retrying a failed API operation
	APICallRetryInterval = 500 * time.Millisecond
	// DiscoveryRetryInterval specifies how long kubeadm should wait before retrying to connect to the master when doing discovery
	DiscoveryRetryInterval = 5 * time.Second

	// Minimum amount of nodes the Service subnet should allow.
	// We need at least ten, because the DNS service is always at the tenth cluster clusterIP
	MinimumAddressesInServiceSubnet = 10

	// DefaultTokenDuration specifies the default amount of time that a bootstrap token will be valid
	// Default behaviour is "never expire" == 0
	DefaultTokenDuration = 0

	// LabelNodeRoleMaster specifies that a node is a master
	// It's copied over to kubeadm until it's merged in core: https://github.com/kubernetes/kubernetes/pull/39112
	LabelNodeRoleMaster = "node-role.kubernetes.io/master"

	// MinExternalEtcdVersion indicates minimum external etcd version which kubeadm supports
	MinExternalEtcdVersion = "3.0.14"
)
*/
