use super::StacksFactoryOutput;
use assembler::Assembler;
use assembler::ServicesConfig;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::{Error, Result};
use models::stacksfactory;
use protocol::api::base::ChildTypeMeta;
use protocol::api::base::MetaFields;
use protocol::api::deploy::{AssemblyFactory, StacksFactory};
use protocol::api::schema::type_meta_url;

/// Builds a deployer for each type.
pub struct DeployerFactory<'a>(DeployerFactoryInner<'a>);

enum DeployerFactoryInner<'a> {
    /// Builds a new connector for each name by applying all configurations with a
    /// matching prefix. This is considered "static" because the set of configurations may
    /// not be updated dynamically.
    DynamicStacks(StacksDeployer<'a>),
}

impl<'a> DeployerFactory<'a> {
    pub fn new(conn: &'a DataStoreConn, config: &ServicesConfig) -> DeployerFactory<'a> {
        let f = StacksDeployer {
            conn: conn,
            service_config: (*config).clone(),
        };
        DeployerFactory(DeployerFactoryInner::DynamicStacks(f))
    }

    pub fn mk_stacker(&self, stacks: &StacksFactory) -> Result<StacksFactory> {
        match self.0 {
            DeployerFactoryInner::DynamicStacks(ref c) => c.mk_deploy(stacks),
            /*
            When you have more types open this up.
            _ => Err(Error::StacksFactoryInvalidType(
                "Can't invoke stacks deployer. Invoked in an incorrect way.".to_string(),
            )),*/
        }
    }
}

struct StacksDeployer<'a> {
    conn: &'a DataStoreConn,
    service_config: ServicesConfig,
}

impl<'a> StacksDeployer<'a> {
    /// Builds a new stacksfactory by applying all configurations.
    fn mk_stacks_factory(&self, stacks: &StacksFactory) -> StacksFactoryOutput {
        stacksfactory::DataStore::new(&self.conn).create(&stacks)
    }

    fn mk_deploy(&self, factory: &StacksFactory) -> Result<StacksFactory> {
        match self.mk_stacks_factory(&factory) {
            Ok(Some(stacks)) => {
                let assembly_factory: AssemblyFactory = self.build_assembly_factory(&stacks);
                //Build Array of Assemblers
                //TO-DO: Kishore fix it.
                let _built = stacks.get_spec().get_plan().map(|_stack| {
                    Assembler::new(&self.conn, &self.service_config).assemble(&assembly_factory)
                });
                Err(Error::StacksFactoryInvalidType(
                    "Can't invoke stacks deployer. Invoked in an incorrect way.".to_string(),
                ))
            }
            Err(err) => Err(err),
            Ok(None) => Err(Error::Db(RecordsNotFound)),
        }
    }

    ///Build the assembly by setting up a object meta (name, account id of the parent,
    ///and its type meta from the parent)
    fn build_assembly_factory(&self, parent: &StacksFactory) -> AssemblyFactory {
        let mut assembly_factory = AssemblyFactory::new();
        let ref mut om = assembly_factory.mut_meta(
            assembly_factory.object_meta(),
            parent.get_name(),
            parent.get_account(),
        );

        //set the parents datacenter/location or clustername
        assembly_factory.set_cluster_name(om, parent.get_cluster_name());

        //send the parents typemeta.
        assembly_factory.set_owner_reference(
            om,
            parent.type_meta().kind,
            parent.type_meta().api_version,
            parent.object_meta().name,
            parent.get_id().to_string(),
        );
        assembly_factory.set_meta(type_meta_url(parent.children()), om.clone());
        assembly_factory
    }
}
