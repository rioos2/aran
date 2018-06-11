/*

// TO-DO: Kishore fix for blockchain factory

use super::connection::secure;
use super::connection::socket::{self, Socket};
use super::Path;
use futures::{Future, Poll};
use std::sync::Arc;
use std::{io, net, time};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_timer::Timer;

/// Builds a deployer for each type.
pub struct DeployerFactory(DeployerFactoryInner);

enum DeployerFactoryInner {
    /// Uses a single connector for all names.
    DynamicAssemble(ASPrefixDeployerFactory),
    /// Builds a new connector for each name by applying all configurations with a
    /// matching prefix. This is considered "static" because the set of configurations may
    /// not be updated dynamically.
    DynamicBlockchainNetwork(ASPrefixDeployerFactory),
}

impl DeployerFactory {
    pub fn new_assemble(conn: DataStoreConn, config: ServicesConfig) -> DeployerFactory {
        let f = ASPrefixDeployerFactory(conn, config);
        DeployerFactory(DeployerFactoryInner::DynamicAssemble(f))
    }

    pub fn new_blockchain_network(conn: DataStoreConn, config: ServicesConfig) -> DeployerFactory {
        let f = BNPrefixDeployerFactory(conn, config);
        DeployerFactory(DeployerFactoryInner::DynamicBlockchainNetwork(f))
    }

    pub fn mk_assembler_deployable(
        &self,
        factory: AssemblyFactory,
    ) -> config::Result<AssemblyFactory> {
        match self.0 {
            DeployerFactoryInner::DynamicAssemble(ref c) => f.mk_assembler(factory),
            _ => Err("Can't invoke assember deployer. Invoked in an incorrect way."),
        }
    }

    pub fn mk_blockchainer_deployable(
        &self,
        factory: BlockchainFactory,
    ) -> config::Result<BlockchainFactory> {
        match self.0 {
            DeployerFactoryInner::DynamicBlockchainNetwork(ref c) => f.mk_blockchainer(factory),
            _ => Err("Can't invoke blockchainer deployer. Invoked in an incorrect way."),
        }
    }
}

struct ASPrefixDeployerFactory(DataStoreConn, ServicesConfig);
impl ASPrefixDeployerFactory {
    /// Builds a new connector by applying all configurations with a matching prefix.
    fn mk_parent(&self, factory: &AssemblyFactory) -> config::Result<AsemblyFactory> {
        assemblyfactory::DataStore::new(&self.conn).create(&factory)
    }

    fn mk_assembler(&self, factory: &AssemblyFactory) -> config::Result<AsemblyFactory> {
        match self.mk_assembly_factory(&factory) {
            Ok(Some(assemblyfactory)) => {
                let deployer_factory: DeployableFactory = assemblyfactory.into();

                Assembler::new(&self.conn, &deployer_factory).assemble()
            }
            Err(err) => Err(err),
            Ok(None) => Err(Error::Db(RecordsNotFound)),
        }
        //call mk_assembly_factory
        //convert it into DeployableFactory (trait or struct)
    }
}

struct BNPrefixDeployerFactory(DataStoreConn, ServicesConfig);
impl BNPrefixDeployerFactory {
    /// Builds a new connector by applying all configurations with a matching prefix.
    fn mk_parent(&self, factory: &AssemblyFactory) -> config::Result<Connector> {
        //use the connection and create a blockchainfactory
    }

    fn mk_blockchainer(
        &self,
        conn: &DataStoreConn,
        factory: &AssemblyFactory,
    ) -> Result<BlockchainFactory> {
        match self.mk_assembly_factory(&factory) {
            Ok(Some(assemblyfactory)) => {
                let deployer_factory: DeployableFactory = assemblyfactory.into();
                //Build Array of Assemblers

                Assembler::new(&self.conn, &deployer_factory.clone()).assemble()
            }
            Err(err) => Err(err),
            Ok(None) => Err(Error::Db(RecordsNotFound)),
        }
    }
}*/
