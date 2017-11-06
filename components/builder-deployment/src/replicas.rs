// // Copyright (c) 2017 RioCorp Inc.
//
// pub struct Replicas {
//     conn: Connection,
//     af_req: AssemblyFacCreateReq,
// }
//
// //// This is responsible for managing the replicas upto the desired count.
// //// The count can be upward or downward.
// ///  eg: desired can be 5, and the current replicas can be 4, which mean we need to deploy 1 more.
// ///  eg: desired can be 5, and the current replicas can be 6, which mean we need to nuke 1.
// impl Replicas  {
//
//     //create a new replica with a database connection and assembly_factory_request object
//     pub fn new(conn: &Connection, request: &AssemblyFacCreateReq) {
//         Replicas {
//             conn: conn,
//             af_req: request
//         }
//
//     }
//
//     //returns the desired replicas as asked by the user.
//     fn desired(&self) u32 {
//         self.af_req.replicas
//     }
//
//
//     //This is reponsible for managing the replicas in an assembly factory upto the desired.
//     pub fn new_desired()  Result<AssemblyFactoryResponse> {
//
//         //This should be done after assembly_factory is create (match )
//         match DeploymentDS::assembly_factory_create(conn, self.af_req) {
//             Ok((response)) => {
//                 let replicated = upto_desired()?;
//                 Ok(response)
//             },
//             Err(err) => Err(err),
//         }
//     }
//
//
//       //This is reponsible for managing the replicas in an assembly factory upto the desired.
//     pub fn upto_desired() Result<AssemblyCreateReq> {
//
//         //This should be done after assembly_factory is create (match )
//
//         let context = ReplicaContext ::new(&self.af_req, &self.desired()).calculate()?;
//
//         //deploy the assemblys
//         let deployed = context.to_deploy.map(|k| DeploymentDS::assembly_create(conn, &k))
//
//         //remove the assemblys
//         let nuked = context.to_nuke.map(|k| DeploymentDS::assembly_create(conn, &k))
//
//         //fold all the errors in deployed and nuked - refer metrics code.
//         //send the assemblyfactry_response in case of success
//
//     }
// }
//
//
// //The replica context to deploy or nuke(remove)
// struct ReplicaContext {
//     desired_replicas: u32,
//     to_deploy: Vec<&AssemblyCreateReq>,
//     to_nuke:   Vec<&AssemblyCreateReq>
// }
//
// impl ReplicaContext implements ReplicaCalculator {
//
//     fn new(request: &AssemblyFactoryRequest, desired_replicas u32) {
//         let base_name = af_req.name;
//
//         ReplicaContext {
//             desired_replicas: desired_replicas,
//             af_req: request,
//             namer: ReplicaNamer::new(&base_name, request.replicas)
//         }
//
//     }
//
//     fn calculate() -> Result<()> {
//         //create the assembly_create_reqs
//         for x in self.desired_replicas..self.af_req.replicas {
//          let assembly_create_req = AssemblyCreateReq::new();
//          let replica_name = name.next();
//          self.add_for_deployment(assembly_create_req)
//         }
//
//         //create the assembly_create_reqs
//         for x in self.af_req.replicas..self.af.req.replicas {
//          let assembly_create_req = AssemblyCreateReq::new();
//          let replica_name = name.next();
//          self.add_for_removal(assembly_create_req)
//         }
//
//         Ok(())
//     }
//
//     fn add_for_deployment(&self, key: &str, assembly_req: &AssemblyCreateReq) {
//         to_deploy.add(key,assembly_req)
//     }
//
//     fn add_for_removal(&self,key: &str, assembly_req: &AssemblyCreateReq)  {
//         to_nuke.add(key, assembly_create_req)
//     }
//
// }
//
// /// Replica namer is used to name the assembly object
// /// Based on the count of replicas requested the namer will decide on the name.
// /// The generate naming convention for 2 replicas with base_name: levi.megam.io
// /// levi01.megam.io, levi02.megam.io
// ///
// /// The generate naming convention for 1 replicas with base_name: levi.megam.io
// /// levi.megam.io
//
// struct ReplicaNamer {
//     name &str
// }
//
//
// impl ReplicaNamer {
//
//     fn new(name: &str, upto: u32) {
//         &ReplicaNamer {
//             name: name,
//             upto: u32,
//         }
//     }
//
//     fn fqdn_as_tuples() (&str, &str) {
//         if &self.name.contains(".")  {
//             let subdot_fqdn = &self.name.split(".");
//
//             return (subdot_fqdn[0], subdot_fqdn[1])
//         }
//         (&self.name,"")
//
//     }
//
//     ///  If the requested replica count is just one then we need to return
//     ///  levi.megam.io (base_name)
//     fn next(&self, count: u32) {
//         if self.upto > 1  {
//             let fqdns = self.fqdn_as_tuples();
//             return format!("{}{}.{}", fqdns.0, count, fqdns.1);
//         }
//
//         self.name
//     }
//
// }
