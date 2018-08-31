import { expect } from 'chai';
import supertest = require('supertest');
const globalAny:any = global;

const request = supertest.agent(globalAny.apiServer);


describe('Deployment API', function() {

describe('stacksfactorys API', function() {
  it('returns error for no record found to list stacksfactorys', function(done) {
    request.get('/stacksfactorys/all')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(404)
      .end(function(err, res) {
        done()
      });
  });

  it('returns the stacksfactorys with one replicas', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},
      "replicas": 1,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id})
      .expect(200)
      .end(function(err, res) {
        globalAny.stacks_fac_id =res.body.id;
        globalAny.replicas = res.body.replicas;
        expect(res.body.type_meta.kind).to.equal(globalAny.stacksfactorys);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        expect(res.body.status.phase).to.equal(globalAny.pending);
        expect(res.body.spec.assembly_factory.length).to.equal(1);
        expect(res.body.spec.plan.id).to.equal(globalAny.plan_id);
        done(err);
      });
  });

  it('returns the stacksfactorys_status_update by id', function(done) {
    request.put('/stacksfactorys/'+globalAny.stacks_fac_id+'/status')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"status":{"phase": "ready","conditions": []}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.spec.plan.id).to.equal(globalAny.plan_id);
        expect(res.body.type_meta.kind).to.equal(globalAny.stacksfactorys);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        expect(res.body.id).to.equal(globalAny.stacks_fac_id);
        done()
      });
  });

  it('returns the bad request error for empty phase field', function(done) {
    request.put('/assemblyfactorys/'+globalAny.stacks_fac_id+'/status')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"status":{"message":"","reason":"","phase": "","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
      "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });

  it('returns the Malformed error for no field phase ', function(done) {
    request.put('/assemblyfactorys/'+globalAny.stacks_fac_id+'/status')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"status":{"message":"","reason":"","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
      "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });

  it('returns the stacksfactorys_status_update by for wrong id', function(done) {
    request.put('/stacksfactorys/2345678/status')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
      "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
      .expect(404)
      .end(function(err, res) {
        done()
      });
  });

  it('returns the Unauthorized error for stacksfactorys_status_update ', function(done) {
    request.put('/stacksfactorys/2345678/status')
    .ca(globalAny.rootCA)
      .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
      "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });


  it('returns the assemblyfactory by stacksfactory id', function(done) {
    this.timeout(4000)
    request.get('/stacksfactorys/'+globalAny.stacks_fac_id+'/describe')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.kind).to.equal(globalAny.stacksfactoryslist);
        expect(res.body.api_version).to.equal(globalAny.version);
        expect(res.body.items.length).to.equal(1);
        done()
      });
  });

  it('returns the stacksfactory by id', function(done) {
    request.get('/stacksfactorys/'+globalAny.stacks_fac_id)
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.spec.plan.id).to.equal(globalAny.plan_id);
        expect(res.body.type_meta.kind).to.equal(globalAny.stacksfactorys);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        expect(res.body.id).to.equal(globalAny.stacks_fac_id);
        done()
      });
  });

  it('returns the all stacksfactorys', function(done) {
    request.get('/stacksfactorys/all')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.kind).to.equal(globalAny.stacksfactoryslist);
        expect(res.body.api_version).to.equal(globalAny.version);
        expect(res.body.items.length).to.equal(1);
        done()
      });
  });
  it('returns the stacksfactorys by account', function(done) {
    request.get('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.kind).to.equal(globalAny.stacksfactoryslist);
        expect(res.body.api_version).to.equal(globalAny.version);
        done()
      });
  });


  it('returns Bad request error if object_meta not had name', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"","account":globalAny.account_id,"labels":{"rioos_environment":"development","rioos_category":"machine"},"annotations":{"rioos/karthika.calvincare.org/apply":"OnHeadBald","rioos/ruchi.calvincare.org/pickup":"OnHungry"},
      "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,"initializers":{"pending": [{ "name": "loadbalancer"}],"result": {"status":"success","message": "omitempty","type_meta":{"kind":"","api_version":""}, "reason":"","code": 400,"details":{ "name":"name", "group": "grp", "kind": "","uid":"","retry_after_seconds": 30,"causes": [{"cause_type": "","message":"","field":""}]} }},"finalizers":["orphan"],"cluster_name":"dc1_torono"},
      "replicas": 3,"resources": {"compute_type":"cpu","storage_type":"hdd"},"status": {"phase": "pending",
      "message": "","reason": "","conditions": [{"message": "nodelet has sufficient disk space available","reason": "NodeletHasSufficientDisk","status": "False","last_transition_time": "","last_probe_time": "","condition_type": "","last_update_time": ""}]},
      "created_at": "","secret": {"id":""},"plan":globalAny.plan_id,"metadata": {"io:rioos:orginin::name":"rioos","io:rioos:team::name":"development"},"spec":{"tolerations":[{"key": "key","operator": "Equal","value": "value","effect": "NoSchedule"}],
      "node_selector" : {},"affinity" : {"assemblyfactory_affinity": "requiredDuringSchedulingIgnoredDuringExecution"},"restart_policy": "Always"}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });


  it('returns Bad request error if object_meta not had account', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": "","cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "replicas": 1,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });


  it('returns Bad request error if no replicas', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "replicas": 0,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
    .expect(400)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Bad request error if no plan', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "replicas": 1,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": "","status": {"phase": "ready"}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Bad request error if no resources', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "replicas": 1,"resources": {"storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });


  it('returns Unauthorized error for assemblyfactory create', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
    .send({"object_meta": {"name": "levis.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "replicas": 1,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
    "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
    .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns without cluster name to create stacksfactorys ', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "","labels": {"rioos_category": "machine"}},  "replicas": 1,"resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
      .expect(400)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Unauthorized error show stacksfactorys by id', function(done) {
    request.get('/stacksfactorys/'+globalAny.stacks_fac_id)
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Unauthorized error stacksfactorys_status_update by id', function(done) {
    request.put('/stacksfactorys/'+globalAny.stacks_fac_id+'/status')
    .ca(globalAny.rootCA)
      .send({ "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":"","condition_type":" "}]}})
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Unauthorized error get  stacksfactorys by account', function(done) {
    request.get('/stacksfactorys')
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Unauthorized error list all stacksfactorys', function(done) {
    request.get('/stacksfactorys/all')
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns Unauthorized error assemblyfactorys by stacksfactorys id', function(done) {
    request.get('/stacksfactorys/'+globalAny.stacks_fac_id+'/describe')
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });


  it('returns Record not found stacksfactorys get by id', function(done) {
    request.get('/stacksfactorys/23456789')
    .ca(globalAny.rootCA)
    .set('Authorization', globalAny.bobo_bearer)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(404)
      .end(function(err, res) {
        done()
      });
  });

  it('Malformed body for no replicas field', function(done) {
    request.post('/stacksfactorys')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name": "levi.megam.io","account": globalAny.account_id,"cluster_name": "chennai","labels": {"rioos_category": "machine"}},  "resources": {"compute_type": "cpu","storage_type": "hdd","cpu": "1",
      "memory": "1 GiB","storage": "3 GiB","private_ipv4": "true"},"secret": {"id": globalAny.secrets_id},"plan": globalAny.plan_id,"status": {"phase": "ready"}})
        .expect(400)
      .end(function(err, res) {
        done()
      });
  });

});
});
