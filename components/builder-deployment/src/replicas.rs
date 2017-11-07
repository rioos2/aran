// Copyright (c) 2017 RioCorp Inc.
use deployment_ds::{DeploymentDS, ASSEMBLY};
use protocol::asmsrv::{Assembly, IdGet, AssemblyFactory, Status, Condition, Properties, OpsSettings, Volume, ObjectMeta, OwnerReferences, TypeMeta};
use protocol::DEFAULT_API_VERSION;
use db::data_store::DataStoreConn;
use error::{Result, Error};
use std::collections::BTreeMap;

const CONDITION_TYPE: &'static [&'static str] = &["AssemblyStorageReady", "AssemblyNetworkReady"];

pub struct Replicas<'a> {
    conn: &'a DataStoreConn,
    af_req: &'a AssemblyFactory,
}

//// This is responsible for managing the replicas upto the desired count.
//// The count can be upward or downward.
///  eg: desired can be 5, and the current replicas can be 4, which mean we need to deploy 1 more.
///  eg: desired can be 5, and the current replicas can be 6, which mean we need to nuke 1.
impl<'a> Replicas<'a> {
    //create a new replica with a database connection and assembly_factory_request object
    pub fn new(conn: &'a DataStoreConn, request: &'a AssemblyFactory) -> Replicas<'a> {
        Replicas {
            conn: &*conn,
            af_req: &*request,
        }
    }

    //returns the desired replicas as asked by the user.
    fn desired(&self) -> u64 {
        self.af_req.get_replicas()
    }


    //This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn new_desired(&self) -> Result<Option<AssemblyFactory>> {

        //This should be done after assembly_factory is create (match )
        match DeploymentDS::assembly_factory_create(&self.conn, &self.af_req) {
            Ok(Some(response)) => {
                let replicated = self.upto_desired(&response.get_id())?;
                Ok(Some(response))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }


    //This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn upto_desired(&self, id: &str) -> Result<Option<Vec<Assembly>>> {

        //This should be done after assembly_factory is create (match )

        let mut context = ReplicaContext::new(
            &self.af_req,
            self.desired(),
            vec![Assembly::new()],
            vec![Assembly::new()],
        );
        context.calculate(id);
        //deploy the assemblys
        let deployed = context
            .to_deploy
            .iter()
            .map(|k| if k.get_name().len() > 0 {
                DeploymentDS::assembly_create(&self.conn, &k)
            } else {
                Ok(None)
            })
            .collect::<Vec<_>>();

        //remove the assemblys
        // let nuked = context.to_nuke.iter().map(|k| {
        //     DeploymentDS::assembly_create(&self.conn, &k)
        // });

        //fold all the errors in deployed and nuked - refer metrics code.
        //send the assemblyfactry_response in case of success
        Ok(None)
    }
}


//The replica context to deploy or nuke(remove)
struct ReplicaContext<'a> {
    desired_replicas: u64,
    af_req: &'a AssemblyFactory,
    namer: ReplicaNamer,
    to_deploy: Vec<Assembly>,
    to_nuke: Vec<Assembly>,
}

impl<'a> ReplicaContext<'a> {
    fn new(request: &'a AssemblyFactory, desired_replicas: u64, assembly_req: Vec<Assembly>, asse: Vec<Assembly>) -> ReplicaContext<'a> {
        let base_name = request.get_name();

        ReplicaContext {
            desired_replicas: desired_replicas,
            af_req: &*request,
            namer: ReplicaNamer::new(&base_name, request.get_replicas()),
            to_deploy: assembly_req,
            to_nuke: asse,
        }

    }

    fn calculate(&mut self, id: &str) {
        //create the assembly_create_reqs
        for x in 0..self.desired_replicas {
            let mut assembly_create_req = Assembly::new();
            let replica_name = self.namer.next(x + 1);
            assembly_create_req.set_name(replica_name.to_string());
            assembly_create_req.set_uri("/v1/assemblys".to_string());
            assembly_create_req.set_description(self.af_req.get_description());
            assembly_create_req.set_tags(self.af_req.get_tags());
            assembly_create_req.set_parent_id(id.to_string());
            assembly_create_req.set_origin(self.af_req.get_origin());
            let mut status = Status::new();
            status.set_phase("intializing".to_string());
            status.set_message("new instance initiating".to_string());
            let mut condition_collection = Vec::new();
            for conn in CONDITION_TYPE {
                let mut condition = Condition::new();
                condition.set_condition_type(conn.to_string());
                condition.set_status("False".to_string());
                condition_collection.push(condition);
            }
            status.set_conditions(condition_collection);

            assembly_create_req.set_status(status);
            self.add_for_deployment(assembly_create_req);
        }

        //create the assembly_create_reqs
        for x in 0..self.desired_replicas {
            let assembly_create_req = Assembly::new();
            let replica_name = self.namer.next(x + 1);
            self.add_for_removal(assembly_create_req);
        }
    }

    fn add_for_deployment(&mut self, assembly_req: Assembly) {
        self.to_deploy.push(assembly_req);
    }

    fn add_for_removal(&mut self, assembly_req: Assembly) {
        self.to_nuke.push(assembly_req);
    }
}

/// Replica namer is used to name the assembly object
/// Based on the count of replicas requested the namer will decide on the name.
/// The generate naming convention for 2 replicas with base_name: levi.megam.io
/// levi01.megam.io, levi02.megam.io
///
/// The generate naming convention for 1 replicas with base_name: levi.megam.io
/// levi.megam.io

struct ReplicaNamer {
    name: String,
    upto: u64,
}


impl ReplicaNamer {
    fn new(name: &str, upto: u64) -> ReplicaNamer {
        ReplicaNamer {
            name: name.to_string(),
            upto: upto,
        }
    }

    fn fqdn_as_tuples(&self) -> (&str, &str) {
        if self.name.contains(".") {
            let subdot_fqdn = &self.name.split(".").collect::<Vec<_>>();

            return (subdot_fqdn[0], subdot_fqdn[1]);
        }
        (&self.name, "")

    }

    ///  If the requested replica count is just one then we need to return
    ///  levi.megam.io (base_name)
    fn next(&self, count: u64) -> String {
        if self.upto > 1 {
            let fqdns = self.fqdn_as_tuples();
            return format!("{}{}.{}", fqdns.0, count, fqdns.1);
        }

        self.name.clone()
    }
}
