// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS Audits Blockchain API service
use std::collections::BTreeMap;
use std::net::SocketAddr;

use exonum;
use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::crypto::{PublicKey, SecretKey};
use exonum::events::NetworkConfiguration;
use exonum::node::{MemoryPoolConfig, NodeApiConfig, NodeConfig, Whitelist};
use toml::Value;

use rio_core::config::ConfigFile;

use error::Error;

pub struct Config {
    pub node: NodeInternalConfig,
}

///  Rio Blockchain configuration
#[serde(default)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInternalConfig {
    /// Initial config that will be written in the first block.
    pub genesis: GenesisConfig,
    /// Network listening address.
    pub listen_address: SocketAddr,
    /// Remote Network address used by this node.
    pub external_address: Option<SocketAddr>,
    /// Network configuration.
    pub network: NetworkConfiguration,
    /// Peer addresses.
    pub peers: Vec<SocketAddr>,
    /// Consensus public key.
    pub consensus_public_key: PublicKey,
    /// Consensus secret key.
    pub consensus_secret_key: SecretKey,
    /// Service public key.
    pub service_public_key: PublicKey,
    /// Service secret key.
    pub service_secret_key: SecretKey,
    /// Node's whitelist.
    pub whitelist: Whitelist,
    /// Api configuration.
    pub api: NodeApiConfig,
    /// Memory pool configuration.
    pub mempool: MemoryPoolConfig,
    /// Additional config, usable for services.
    pub services_configs: BTreeMap<String, Value>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            node: NodeInternalConfig::default(),
        }
    }
}

impl Default for NodeInternalConfig {
    fn default() -> Self {
        internal_config()
    }
}

impl ConfigFile for NodeInternalConfig {
    type Error = Error;
}

pub fn internal_config() -> NodeInternalConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:7000".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();

    NodeInternalConfig {
        listen_address: peer_address,
        peers: vec![],
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: None,
        network: Default::default(),
        whitelist: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
    }
}

/// Convert into exonum NodeConfig  from the Rioos Blockchain Config
impl Into<NodeConfig> for NodeInternalConfig {
    fn into(self) -> NodeConfig {
        NodeConfig {
            listen_address: self.listen_address,
            peers: self.peers,
            service_public_key: self.service_public_key,
            service_secret_key: self.service_secret_key,
            consensus_public_key: self.consensus_public_key,
            consensus_secret_key: self.consensus_secret_key,
            genesis: self.genesis,
            external_address: self.external_address,
            network: self.network,
            whitelist: self.whitelist,
            api: self.api,
            mempool: self.mempool,
            services_configs: self.services_configs,
        }
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
######################################################################################
## Rio/OS Audit Blockchain Configuration
##
## References  (exonum)
## - Glossary      : https://exonum.com/doc/glossary/
## - Configuration : https://exonum.com/doc/architecture/configuration/
## - Network       : https://exonum.com/doc/advanced/network
######################################################################################
# Local parameters may differ for each node where local parameters may differ for each node
######################################################################################
##
## Every node needs public and private keys. Keys are unique to every node and are used to
## identify it withing the network. consensous pair is interacting with other nodes while 
## reaching consensus: 
## consensus_public_key:  
## consensus_secret_key:
## 
## Acceptable values:
##   - Node’s public key (hex) for use with consensus messages
##   - Node’s private key (hex) for signing consensus messages

consensus_public_key = "16ef83ca4b231404daec6d07b24beb84d89c25944285d2e32a2dcf8f0f3eda72"
consensus_secret_key = "2a751e6595af66f7644bd33cf7710b6226cf8d0de4b3d18bc8fc2d80f19325a716ef83ca4b231404daec6d07b24beb84d89c25944285d2e32a2dcf8f0f3eda72"

##
## Every node needs public and private keys. Keys are unique to every node and are used to
## identify it withing the network. service pair is interacting with other nodes on service 
## needs: 
## service_public_key:  
## service_secret_key:
## 
## Acceptable values:
##   - Node’s public key (hex) for use with consensus messages
##   - Node’s private key (hex) for signing consensus messages

service_public_key = "523ead8ea8457de570e165a512dd5d1b6688cb5757c3d744e03d1173f3e3e237"
service_secret_key = "18544ebbf3ceeeebca847fe6b4e6ce88f83fc92b6b0e24d5466f3cd08aea37bb523ead8ea8457de570e165a512dd5d1b6688cb5757c3d744e03d1173f3e3e237"

##
## The node address broadcasted to other peers using Connect messages
## external_address:  
## 
## Acceptable values:
##   - an IP/port pair, e.g. 127.0.0.1:6333

external_address = "127.0.0.1:6333"

##
## The listen address of the current node is an IP address and TCP port that the other nodes 
## can communicate
## listen_address:  
## 
## Acceptable values:
##   - an IP/port pair, e.g. 127.0.0.1:6333

listen_address = "0.0.0.0:6333"

##
## A node in the blockchain network that replicates all transactions in the blockchain and 
## thus has a local copy of the entire blockchain state. There are 2 categories of full nodes 
## in Exonum: validators and auditors. 
## The peers is a list of full node addresses
## peers:  
## 
## Acceptable values:
##   - an array of IP/port pair, e.g. ["127.0.0.1:6333"]

peers = ["127.0.0.1:6333", "127.0.0.1:6333"]

######################################################################################
# API configuration parameters
######################################################################################
[api]

##
## Enables API endpoints for the blockchain explorer at the public API address
## enable_blockchain_explorer:  
## 
## Acceptable values:
##   - true or false

enable_blockchain_explorer = true

##
## Timeout interval (ms) to update info about connected peers
## state_update_timeout:  
## 
## Acceptable values:
##   - an integer

state_update_timeout = 10000

##
## Listen address for public API endpoints
##
## Default: 0.0.0.0:7000
## 
## Acceptable values:
##   - an IP/port pair, e.g. 0.0.0.0:7000

public_api_address = "0.0.0.0:7000"

##
## Listen address for private API endpoints
## 
## Default: 127.0.0.1:7000
##
## Acceptable values:
##   - an IP/port pair, e.g. 127.0.0.1:7001

## private_api_address = 

######################################################################################
# Genesis configuration parameters. The configuration used to create a genesis block.
######################################################################################
[genesis]

##
## List of validators’ public keys as hex strings. Each list element consists of two parts:
## consensus_key:
## Validator’s public key (hex) for use with consensus messages
##  
## service_key:
## Validator’s public key (hex) for use with service transactions
## 
## Acceptable values:
##   - Validators’s public key (hex) for use with consensus messages
##   - Validator’s public key (hex) for service transactions

[[genesis.validator_keys]]
consensus_key = "16ef83ca4b231404daec6d07b24beb84d89c25944285d2e32a2dcf8f0f3eda72"
service_key = "523ead8ea8457de570e165a512dd5d1b6688cb5757c3d744e03d1173f3e3e237"
[[genesis.validator_keys]]
consensus_key = "924625eb77b9ad21e76713e7ada715945fbf0a926698832e121484c797fcc58e"
service_key = "7413a596e4fa0953cf22b120bd1ee0ba233bd1c619f10b21e6854b6b3cc9a6e9"

# Consensus algorithm parameters
[genesis.consensus]

##
## Timeout interval (ms) to peers exchange info
## Node regularly sends PeersRequest to a random known node with the timeout peers_timeout defined in the global configuration. 
## In response, the addressee sends its list of known peers. Thus, it is enough to connect to one node at the start and 
## after some time it will be possible to collect Connect messages from the entire network.
## peers_timeout:  
## 
## Acceptable values:
##   - an integer

peers_timeout = 10000

##
## Timeout interval (ms) to proposal upon beginning of a new height
## Propose message is a set of transactions proposed by the round leader for inclusion into the next block.
## Instead of whole transactions, Propose messages include only transactions hashes. A validator that received 
## a Propose message can request missing transactions from its peers.
## proposal_timeout:  
## 
## Acceptable values:
##   - an integer

propose_timeout = 500

##
## Timeout interval (ms) between rounds
## round_timeout:  
## 
## Acceptable values:
##   - an integer

round_timeout = 3000

##
## Time interval (ms) for sending a Status message
## Status is an information message about the current height. It is sent with a periodicity written 
## in the status_timeout global configuration parameter.
## status_timeout:  
## 
## Acceptable values:
##   - an integer

status_timeout = 5000

##
## Maximum number of transactions per block
## tx_block_limit
## 
## Acceptable values:
##   - an integer

txs_block_limit = 1000

######################################################################################
# Message processing parameters
######################################################################################
[mempool]

##
## Maximum number of events in events queue
## events_pool_capacity
## 
## Acceptable values:
##   - an integer

events_pool_capacity = 400000

##
## Maximum number of transactions in the pool of unconfirmed transactions
## tx_pool_capacity
## 
## Acceptable values:
##   - an integer

tx_pool_capacity = 100000

######################################################################################
# Local connection parameters
######################################################################################
[network]

##
## Maximum number of incoming connections
## max_incoming_connections
## 
## Acceptable values:
##   - an integer

max_incoming_connections = 128

##
## Maximum number of outgoing connections
## max_outgoing_connections
## 
## Acceptable values:
##   - an integer

max_outgoing_connections = 128

##
## Activation of the NODELAY algorithm from the TCP stack (see RFC2126)
## 
## Acceptable values:
##   - true (or) false

tcp_nodelay = false

#
## Timeout interval (ms) before the first reconnect attempt
## tcp_reconnect_timeout:  
## 
## Acceptable values:
##   - an integer

tcp_reconnect_timeout = 500

#
## Maximum timeout interval (ms) for reconnect attempt
## tcp_reconnect_timeout_max:  
## 
## Acceptable values:
##   - an integer

tcp_reconnect_timeout_max = 600000

######################################################################################
# Service-specific parameters under the keys corresponding to service_names of 
# the blockchain services.
######################################################################################
[services_configs]

######################################################################################
# Network whitelisting parameters
######################################################################################
[whitelist]

##
## Enables whitelisting
## If the whitelist is turned on, then upon receiving the Connect message, the node checks 
## the presence of the public key from the message in the node’s whitelist. If the public key 
## is not included in the whitelist, connection is not accepted.
## 
## Acceptable values:
##   - true (or) false

whitelist_enabled = false

##
## List of consensus public keys for trusted peers
## If the whitelist is turned on, then upon receiving the Connect message, the node checks 
## the presence of the public key from the message in the node’s whitelist. If the public key 
## is not included in the whitelist, connection is not accepted.
## 
## Acceptable values:
##   - an array of trusted peers public key(hex)  e.g. ["99ace6c721db293b0ed5b487e6d6111f22a8c55d2a1b7606b6fa6e6c29671aa1"]

whitelisted_peers = []"#;

        let config = NodeInternalConfig::from_raw(&content).unwrap();
        
        assert_eq!(&format!("{}", config.listen_address), "0.0.0.0:633aa3");        
    }

}
*/
