import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Deployment API', function() {

  describe('Assembly API', function() {

    it('returns the assembly by account', function(done) {
      this.timeout(4000)
      request.get('/assemblys')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.kind).to.equal(globalAny.assemblylist);
          expect(res.body.api_version).to.equal(globalAny.version);
          globalAny.assembly_id =res.body.items[0].id;
          done(err);
        });
    });

    it('returns the assembly by id', function(done) {
      this.timeout(4000)
      request.get('/assemblys/'+ globalAny.assembly_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.assembly_id);
         expect(res.body.spec.assembly_factory.spec.plan.id).to.equal(globalAny.plan_id);
         expect(res.body.type_meta.kind).to.equal(globalAny.assemblys);
         expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns the assembly_status_update by id', function(done) {
      this.timeout(4000)
      request.put('/assemblys/'+globalAny.assembly_id+'/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
        "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
         expect(res.body.id).to.equal(globalAny.assembly_id);
          done(err);
        });
    });


    it('returns the bad request error for empty phase field', function(done) {
      request.put('/assemblys/'+globalAny.assembly_id+'/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"","reason":"","phase": "","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
        "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the Malformed error for no field phase ', function(done) {
      request.put('/assemblys/'+globalAny.assembly_id+'/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"","reason":"","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
        "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the assembly_status_update by for wrong id', function(done) {
      request.put('/assemblys/2345678/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
        "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the Unauthorized error for assembly_status_update ', function(done) {
      request.put('/assemblys/2345678/status')
      .ca(globalAny.rootCA)
        .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk","status": "False",
        "last_transition_time": "2017-09-21T06:35:16Z", "last_probe_time": "2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns all assemblys', function(done) {
      this.timeout(4000)
      request.get('/assemblys/all')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.kind).to.equal(globalAny.assemblylist);
          expect(res.body.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns Unauthorized error for get account based assembly', function(done) {
      request.get('/assemblys')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns Unauthorized error for all assembly', function(done) {
      request.get('/assemblys/all')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns Record not found assembly get by id', function(done) {
      request.get('/assemblys/23456789')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
});
