import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Deployment API', function() {

describe('Assembly_factory API', function() {
  it('returns the assembly_factorys', function(done) {
    request.post('/assemblyfactorys')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"name":"ubuntu","uri":"/v1/assemblys/assembly_factorys","description":"ubuntuinstallation","tags":["ubuntu"],"replicas":5,"properties":{"domain":"megambox.com","cloudsetting":"/clouds/one","region":"chennai","storage_type":"ssd"},"type_meta":{"kind":"Assemblyfactory","api_version":"v1"},"object_meta":{"name":"xyz","origin":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"},"owner_references":[{"kind":"Node","api_version":"v1","name":"ddd","uid":"","block_owner_deletion":true}]},"plan":"/v3/plan/apache","plan_data":null,"external_management_resource":["safsf"],"component_collection":{"flavor":"/url","network":"/url"},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":" ","condition_type":" "}]},"opssettings":{"nodeselector":"","priority":" ","nodename":" ","restartpolicy":" "}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.name).to.equal("ubuntu");
        globalAny.asm_fac_id =res.body.id;
        done(err);
      });
  });
  it('returns the assembly_factory by id', function(done) {
    request.get('/assembly_factorys/' + globalAny.asm_fac_id)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.asm_fac_id);
        done(err);
      });
  });
  it('returns the assembly_factorys_status_update by id', function(done) {
    request.put('/assembly_factorys/status/'+ globalAny.asm_fac_id)
      .set('Authorization', globalAny.bobo_bearer)
      .send({ "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":"","condition_type":" "}]}})
      .expect(200)
      .end(function(err, res) {
       expect(res.body);
        done(err);
      });
  });
  it('returns the all assemblys_factory', function(done) {
    request.get('/assembly_factorys')
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.results.length).to.equal(1);
        done(err);
      });
  });
});

  describe('Assembly API', function() {
   it('returns the created assembly', function(done) {
      request.post('/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"name": "ubuntu","uri":"/v1/assemblys","description":"ubuntuinstallation","tags": ["ubuntu"],"parent_id":"780970728630525952",
"type_meta":{"kind":"Assembly","api_version":"v1"}, "object_meta":{"name":"xyz","origin":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"},"owner_references": [{"kind":"Node","api_version":"v1","name":"ddd", "uid":"","block_owner_deletion":true}]},"urls": " ", "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ",
"last_transition_time":" ","last_probe_time":" ","condition_type":" "}]},"node":"","ip":"" })
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("ubuntu");
          expect(res.body.spec.id).to.equal(globalAny.asm_fac_id);
          globalAny.asm = res.body;
          done(err);
        });
    });
    it('returns the assembly by id', function(done) {
      request.get('/assemblys/'+ globalAny.asm.id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.asm.id);
          done(err);
        });
    });

    it('returns the assemblys_status_update by id', function(done) {
      request.put('/assemblys/status/'+ globalAny.asm.id)
        .set('Authorization', globalAny.bobo_bearer)
        .send({ "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":"","condition_type":" "}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
          done(err);
        });
    });
    it('returns all assemblys', function(done) {
      request.get('/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
           expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
  });
});
