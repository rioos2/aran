import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Deployment API', function() {

describe('Create assembly_factorys', function() {
  it('returns the created assembly', function(done) {
    request.post('/assembly_factorys')
      .set('Authorization', globalAny.bobo_bearer)
      .send({"name": "ubuntu","uri":"/v1/assemblys/assembly_factorys","description":"ubuntuinstallation","tags": ["ubuntu"],"external_management_resource":["safsf"],"properties":{"domain":"megambox.com","cloudsetting":"/clouds/one","region":"chennai","storage_type":"ssd"},"plan":"","component_collection":{"flavor":"/url","network":"/url"},"opssettings":{"nodeselector":"","priority":" ","nodename":" ","restartpolicy":" "},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","lastTransitionTime":" ","lastProbeTime":" ","conditionType":" "}]}, "replicas":2})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.name).to.equal("ubuntu");
        globalAny.asm_fac_id =res.body.id;
        done(err);
      });
  });
  it('returns the assemblys', function(done) {
    request.get('/assembly_factorys/' + globalAny.asm_fac_id)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.asm_fac_id);
        done(err);
      });
  });
  it('returns the assembly_factorys_status_update', function(done) {
    request.put('/assembly_factorys/status/'+ globalAny.asm_fac_id)
      .set('Authorization', globalAny.bobo_bearer)
      .send({ "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","lastTransitionTime":" ","lastProbeTime":"","conditionType":" "}]}})
      .expect(200)
      .end(function(err, res) {
       expect(res.body);
        done(err);
      });
  });
});

describe('Get assembly_factorys', function() {
  it('returns the assemblys_factory', function(done) {
    request.get('/assembly_factorys')
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.length).to.equal(1);
        done(err);
      });
  });
});

  describe('Create assembly', function() {
   it('returns the created assembly', function(done) {
      request.post('/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"name": "ubuntu","uri":"/v1/assemblys","description":"ubuntuinstallation","tags": ["ubuntu"],"parent_id":"774977920044113920",
         "component_collection": " ","urls": " ", "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","lastTransitionTime":" ","lastProbeTime":" ","conditionType":" "}]},"node":"","ip":"" })
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("ubuntu");
          expect(res.body.parent_id).to.equal("774977920044113920");
          globalAny.asm = res.body;
          done(err);
        });
    });
    it('returns the assemblys', function(done) {
      request.get('/assemblys/'+ globalAny.asm.id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.asm.id);
          done(err);
        });
    });

    it('returns the assemblys_status_update', function(done) {
      request.put('/assemblys/status/'+ globalAny.asm.id)
        .set('Authorization', globalAny.bobo_bearer)
        .send({ "status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","lastTransitionTime":" ","lastProbeTime":"","conditionType":" "}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
          done(err);
        });
    });
  });

  describe('Get assemblys', function() {
    it('returns the assemblys', function(done) {
      request.get('/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.length).to.equal(1);
          done(err);
        });
    });
  });

});
