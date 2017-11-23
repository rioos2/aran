// Copyright (c) 2017 RioCorp Inc.

use assembly_ds::AssemblyDS;
use assemblyfactory_ds::AssemblyFactoryDS;
use protocol::asmsrv::{Assembly, AssemblyFactory, Status, Condition, TypeMeta, INITIAL_CONDITIONS, NEW_REPLICA_INITALIZING, ASSEMBLYS_URI, INITIALIZING};
use db::data_store::DataStoreConn;
use error::Result;

const ASSEMBLY: &'static str = "Assembly";

pub struct Replicas<'a> {
    current: u32,
    desired: u32,
    conn: &'a DataStoreConn,
    response: &'a AssemblyFactory,
}

//// This is responsible for managing the replicas upto the desired count.
//// The count can be upward or downward.
///  eg: desired can be 5, and the current can be 4, which means we need to deploy 1 more.
///  eg: desired can be 5, and the current can be 6, which means we need to nuke 1.
impl<'a> Replicas<'a> {
    pub fn new(conn: &'a DataStoreConn, current: u32, desired: u32, request: &'a AssemblyFactory) -> Replicas<'a> {
        Replicas {
            current: current,
            desired: desired,
            conn: &*conn,
            response: &*request,
        }
    }

    //returns the desired replicas as asked by the user.
    fn desired(&self) -> u32 {
        self.desired
    }

    //returns the desired replicas as asked by the user.
    fn current(&self) -> u32 {
        self.current
    }


    //This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn new_desired(&self) -> Result<(AssemblyFactory, Vec<(String, String)>)> {
        match AssemblyFactoryDS::create(&self.conn, &self.response) {
            Ok(response) => {
                let replicated = self.upto_desired(&response.get_id());
                let assembly: Vec<(String, String)> = replicated
                    .into_iter()
                    .map(|x| {
                        let y = x.unwrap().unwrap();
                        (y.clone().get_id(), y.clone().get_name())
                    })
                    .collect::<Vec<_>>();
                Ok((response, assembly))
            }
            Err(err) => Err(err),
        }
    }


    //This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn upto_desired(&self, id: &str) -> Vec<Result<Option<Assembly>>> {
        let mut context = ReplicaContext::new(&self.response, self.current(), self.desired());
        context.calculate(id);
        //deploy the assemblys
        context
            .deploys
            .iter()
            .map(|k| AssemblyDS::create(&self.conn, &k))
            .collect::<Vec<_>>()


        /*remove the assemblys
        let nuked = context
            .nukes
            .iter()
            .map(|k| if k.get_name().len() > 0 {
            	let jobs_create_req = k;
                JobsDS::create(&self.conn, &k)
            } else {
                Ok(None)
            })
            .collect::<Vec<_>>();


        //fold all the errors in deployed and nuked - refer metrics code.
        let deploy_failure = &deployed.iter().filter(|f| (*f).is_err()).count();
        let  nuke_failure = &nuked.iter().filter(|f| (*f).is_err()).count();
	    */
    }
}
#[derive(Debug)]
struct ReplicaContext<'a> {
    current: u32,
    desired: u32,
    response: &'a AssemblyFactory,
    namer: ReplicaNamer,
    deploys: Vec<Assembly>,
    nukes: Vec<Assembly>,
}

impl<'a> ReplicaContext<'a> {
    fn new(response: &'a AssemblyFactory, current_replicas: u32, desired_replicas: u32) -> ReplicaContext<'a> {
        let base_name = response.get_name();

        ReplicaContext {
            current: current_replicas,
            desired: desired_replicas,
            response: &*response,
            namer: ReplicaNamer::new(&base_name, response.get_replicas()),
            deploys: vec![],
            nukes: vec![],
        }

    }

    /// This calculates the scaleup or scale down needed for the asseblys
    /// current = 0 desired = 4, then create 1, 2,3,4.
    //  current = 5, desired = 6, then nuke 1
    fn calculate(&mut self, id: &str) {
        for x in self.current..self.desired {
            let mut assembly_create_req = Assembly::new();
            assembly_create_req.set_name(self.namer.next(x + 1));
            assembly_create_req.set_uri(ASSEMBLYS_URI.to_string());
            assembly_create_req.set_description(self.response.get_description());
            assembly_create_req.set_tags(self.response.get_tags());
            assembly_create_req.set_parent_id(id.to_string());
            assembly_create_req.set_origin(self.response.get_origin());

            assembly_create_req.set_status(Status::with_conditions(
                INITIALIZING,
                &format!(
                    "{} {}",
                    NEW_REPLICA_INITALIZING,
                    self.namer.next(x + 1)
                ),
                "",
                INITIAL_CONDITIONS
                    .iter()
                    .map(|x| Condition::with_type("", "", "False", "", "", x))
                    .collect::<Vec<_>>(),
            ));

            assembly_create_req.set_type_meta(TypeMeta::new(ASSEMBLY));
            self.add_for_deployment(assembly_create_req);
        }

        //create the assembly_create_reqs
        for x in self.desired..self.current {
            let assembly_create_req = Assembly::new();
            let _replica_name = self.namer.next(x + 1);
            self.add_for_removal(assembly_create_req);
        }
    }

    fn add_for_deployment(&mut self, assembly_req: Assembly) {
        self.deploys.push(assembly_req);
    }

    fn add_for_removal(&mut self, assembly_req: Assembly) {
        self.nukes.push(assembly_req);
    }
}

/// Replica namer is used to name the assembly object
/// Based on the count of replicas requested the namer will decide on the name.
/// The generate naming convention for 2 replicas with base_name: levi.megam.io
/// levi01.megam.io, levi02.megam.io
///
/// The generate naming convention for 1 replicas with base_name: levi.megam.io
/// levi.megam.io
#[derive(Debug, PartialEq, Clone, Default)]
struct ReplicaNamer {
    name: String,
    upto: u32,
}


impl ReplicaNamer {
    fn new(name: &str, upto: u32) -> ReplicaNamer {
        ReplicaNamer {
            name: name.to_string(),
            upto: upto,
        }
    }

    fn fqdn_as_tuples(&self) -> (&str, &str, &str) {
        if self.name.contains(".") {
            let subdot_fqdn = &self.name.split(".").collect::<Vec<_>>();

            return (subdot_fqdn[0], subdot_fqdn[1], subdot_fqdn[2]);
        }
        (&self.name, "", "")

    }

    ///  If the requested replica count is just one then we need to return
    ///  levi.megam.io (base_name)
    fn next(&self, count: u32) -> String {
        if self.upto > 1 {
            let fqdns = self.fqdn_as_tuples();
            return format!("{}{}.{}.{}", fqdns.0, count, fqdns.1, fqdns.2);
        }

        self.name.clone()
    }
}
