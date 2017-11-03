//A trait responsible for calculating the replicas to be deployed or nuked
trait ReplicaCalculator {
    fn calculate() ReplicaContext
}

//The replica context to deploy or nuke(remove)
struct ReplicaContext {
    to_deploy BTreeMap<String, AssemblyCreateReq>
    to_nuke   BTreeMap<String, AssemblyCreateReq>
}

impl Replicas implements ReplicaCalculator {

    fn new(&assembly_factory, with_desired u32) {
        let base_name = af.name;


        Replicas {
            desire_replicas: with_desired,
            af: assembly_factory,
            namer: ReplicaNamer::new(&base_name)
        }

    }

    fn calculate() {
        if self.desired_replicas < self.af.replicas {
            name.next()
        }

    }

    fn add_for_deployment(&self, ) {
        to_deploys.add(self)
    }

    fn add_for_removal(&self)  {

    }

    pub fn deploy(&self) {

    }

    pub fn remove(&self) {

    }

}
