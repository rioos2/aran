// Copyright 2018 The Rio Advancement Inc

use error::{Error, Result};

use protocol::api::base::Status;
use protocol::api::base::{ChildTypeMeta, MetaFields};
use protocol::api::deploy::{Assembly, AssemblyFactory, AWAIT_PHASE_PENDING, NEW_REPLICA_INITALIZING_MSG, NEW_STAND_STILL_MSG, PHASE_STAND_STILL};
use protocol::api::schema::type_meta_url; //To access object_meta() and children() in AssemblyFactory, Assembly

use models::{assembly, assemblyfactory};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;

use super::super::APPLICABLE_TO_STAND_STILL;

pub type AssembledMap = Result<(AssemblyFactory, Vec<(String, String)>)>;

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
    pub fn new(conn: &'a DataStoreConn, current: u32, desired: u32, request: &'a AssemblyFactory) -> Self {
        Replicas {
            current: current,
            desired: desired,
            conn: &*conn,
            response: &*request,
        }
    }

    ///returns the desired replicas as asked by the user.
    fn desired(&self) -> u32 {
        self.desired
    }

    ///returns the desired replicas as asked by the user.
    fn current(&self) -> u32 {
        self.current
    }

    ///This is reponsible for managing the replicas in an assembly factory upto the desired.
    pub fn new_desired(&self) -> AssembledMap {
        match assemblyfactory::DataStore::new(&self.conn).create(&self.response) {
            Ok(Some(assemblyfactory)) => {
                let replicated = self.upto(&assemblyfactory);

                let assembly: Vec<(String, String)> = replicated
                    .into_iter()
                    .map(|x| {
                        let y = x.unwrap().unwrap();
                        (y.clone().get_id(), y.clone().object_meta().name)
                    })
                    .collect::<Vec<_>>();

                Ok((assemblyfactory, assembly))
            }
            Err(err) => Err(err),
            Ok(None) => Err(Error::Db(RecordsNotFound)),
        }
    }

    pub fn upto_desired(&self) -> AssembledMap {
        let replicated = self.upto(self.response);

        let assembly: Vec<(String, String)> = replicated
            .into_iter()
            .map(|x| {
                let y = x.unwrap().unwrap();
                (y.clone().get_id(), y.clone().object_meta().name)
            })
            .collect::<Vec<_>>();
        Ok((self.response.clone(), assembly))
    }

    ///This is reponsible for managing the replicas in an assembly factory upto the desired.
    fn upto(&self, assemblyfactory: &'a AssemblyFactory) -> Vec<Result<Option<Assembly>>> {
        let mut context = ReplicaContext::new(assemblyfactory, self.current(), self.desired());
        context.calculate();

        context
            .deploys
            .iter()
            .map(|k| assembly::DataStore::new(&self.conn).create(&k))
            .collect::<Vec<_>>()

        /*TO-DO: we need this code.
        This code removes the assemblys
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


        ///fold all the errors in deployed and nuked - refer metrics code.
        let deploy_failure = &deployed.iter().filter(|f| (*f).is_err()).count();
        let  nuke_failure = &nuked.iter().filter(|f| (*f).is_err()).count();
        */
    }
}

#[derive(Debug)]
struct ReplicaContext<'a> {
    current: u32,
    desired: u32,
    parent: &'a AssemblyFactory,
    namer: ReplicaNamer,
    deploys: Vec<Assembly>,
    nukes: Vec<Assembly>,
}

impl<'a> ReplicaContext<'a> {
    fn new(response: &'a AssemblyFactory, current_replicas: u32, desired_replicas: u32) -> ReplicaContext<'a> {
        let base_name = &response.object_meta().name;

        ReplicaContext {
            current: current_replicas,
            desired: desired_replicas,
            parent: &*response,
            namer: ReplicaNamer::new(&base_name, desired_replicas),
            deploys: vec![],
            nukes: vec![],
        }
    }

    /// This calculates the scaleup or scale down needed for the asseblys
    /// current = 0 desired = 4, then create 1, 2,3,4.
    //  current = 5, desired = 6, then nuke 1
    //  set the phase as "Pending"  if not `blockchain_template`
    //  set the phase as "StandStill"  for `blockchain_template`
    fn calculate(&mut self) {
        for x in self.current..self.desired {
            let phase_msg_tuple = Self::initialize_phase_for(self.parent.get_spec().get_plan().map_or("".to_string(), |p| p.get_category()));

            let mut assembly = self.build_assembly(&x, &self.parent.get_id());

            assembly.set_status(Status::with_conditions(
                &phase_msg_tuple.0,
                &format!("{} {}", &phase_msg_tuple.1, self.namer.next(x + 1)),
                "",
                vec![],
            ));

            self.add_for_deployment(assembly);
        }

        for x in self.desired..self.current {
            let assembly = self.build_assembly(&x, &self.parent.get_id());
            self.add_for_removal(assembly);
        }
    }

    ///Build the assembly by setting up a object meta (name, account id of the parent,
    ///and its type meta from the parent)
    fn build_assembly(&mut self, x: &u32, id: &str) -> Assembly {
        let mut assembly = Assembly::new();
        let ref mut om = assembly.mut_meta(assembly.object_meta(), self.namer.next(x + 1), self.parent.get_account());
        //set the parents datacenter/location or clustername
        assembly.set_cluster_name(om, self.parent.get_cluster_name());
        //send the parents typemeta.
        assembly.set_owner_reference(
            om,
            self.parent.type_meta().kind,
            self.parent.type_meta().api_version,
            self.parent.object_meta().name,
            id.to_string(),
        );
        assembly.set_meta(type_meta_url(self.parent.children()), om.clone());
        assembly
    }

    fn add_for_deployment(&mut self, assembly_req: Assembly) {
        self.deploys.push(assembly_req);
    }

    fn add_for_removal(&mut self, assembly_req: Assembly) {
        self.nukes.push(assembly_req);
    }

    /// Initialize the phase and msg based on category
    /// We will stand still if its a blockchain_template
    fn initialize_phase_for(category: String) -> (String, String) {
        if APPLICABLE_TO_STAND_STILL.contains(&category.as_str()) {
            return (PHASE_STAND_STILL.to_string(), NEW_STAND_STILL_MSG.to_string());
        }
        (AWAIT_PHASE_PENDING.to_string(), NEW_REPLICA_INITALIZING_MSG.to_string())
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
        if self.upto >= 1 {
            let fqdns = self.fqdn_as_tuples();
            return format!("{}{}.{}.{}", fqdns.0, count, fqdns.1, fqdns.2);
        }
        self.name.clone()
    }
}
