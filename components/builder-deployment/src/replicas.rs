// Copyright (c) 2017 RioCorp Inc.
use deployment_ds::DeploymentDS;
use protocol::asmsrv::{Assembly, IdGet, AssemblyFactory, Status, Condition, Properties, OpsSettings, Volume, ObjectMeta, OwnerReferences, TypeMeta};
use db::data_store::DataStoreConn;
use error::{Result, Error};


pub struct Replicas {
    conn: DataStoreConn,
    af_req: AssemblyFactory,
}

//// This is responsible for managing the replicas upto the desired count.
//// The count can be upward or downward.
///  eg: desired can be 5, and the current replicas can be 4, which mean we need to deploy 1 more.
///  eg: desired can be 5, and the current replicas can be 6, which mean we need to nuke 1.
impl Replicas {
    //create a new replica with a database connection and assembly_factory_request object
    pub fn new(conn: DataStoreConn, request: AssemblyFactory) -> Replicas {
        Replicas {
            conn: conn,
            af_req: request,
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
                let replicated = self.upto_desired()?;
                Ok(Some(response))
            }
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }


    //This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn upto_desired(&self) -> Result<()> {

        //This should be done after assembly_factory is create (match )

        let mut context = ReplicaContext::new(&self.af_req, self.desired());
        context.calculate()?;
        //deploy the assemblys
        let deployed = context.to_deploy.iter().map(|k| {
            DeploymentDS::assembly_create(&self.conn, &k)
        });

        //remove the assemblys
        let nuked = context.to_nuke.iter().map(|k| {
            DeploymentDS::assembly_create(&self.conn, &k)
        });

        //fold all the errors in deployed and nuked - refer metrics code.
        //send the assemblyfactry_response in case of success
        Ok(())
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
    fn new(request: &'a AssemblyFactory, desired_replicas: u64) -> ReplicaContext<'a> {
        let base_name = request.get_name();

        ReplicaContext {
            desired_replicas: desired_replicas,
            af_req: &*request,
            namer: ReplicaNamer::new(base_name, request.get_replicas()),
            to_deploy: vec![Assembly::new()],
            to_nuke: vec![Assembly::new()],
        }

    }

    fn calculate(&mut self) -> Result<()> {
        //create the assembly_create_reqs
        for x in self.desired_replicas..self.af_req.get_replicas() {
            let assembly_create_req = Assembly::new();
            let replica_name = self.namer.next(self.af_req.get_replicas());
            self.add_for_deployment(assembly_create_req)
        }

        //create the assembly_create_reqs
        for x in self.desired_replicas..self.af_req.get_replicas() {
            let assembly_create_req = Assembly::new();
            let replica_name = self.namer.next(self.af_req.get_replicas());
            self.add_for_removal(assembly_create_req)
        }

        Ok(())
    }

    fn add_for_deployment(&mut self, assembly_req: Assembly) {
        self.to_deploy.push(assembly_req)
    }

    fn add_for_removal(&mut self, assembly_req: Assembly) {
        self.to_nuke.push(assembly_req)
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
    fn new(name: String, upto: u64) -> ReplicaNamer {
        ReplicaNamer {
            name: name,
            upto: upto,
        }
    }

    fn fqdn_as_tuples(&self) -> (String, String) {
        if self.name.contains(".") {
            let subdot_fqdn = &self.name.split(".").collect::<Vec<_>>();

            return (subdot_fqdn[0].to_string(), subdot_fqdn[1].to_string());
        }
        (self.name, "".to_string())

    }

    ///  If the requested replica count is just one then we need to return
    ///  levi.megam.io (base_name)
    fn next(&self, count: u64) -> String {
        if self.upto > 1 {
            let fqdns = self.fqdn_as_tuples();
            return format!("{}{}.{}", fqdns.0, count, fqdns.1);
        }

        self.name
    }
}
