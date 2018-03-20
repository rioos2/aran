//15 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Job  API', function() {

    it('returns the created job', function(done) {
      request.post('/jobs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"loadbalancer","account":globalAny.account_id,"labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"877634565345","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"spec": {"node_id":globalAny.node_id,"group":"assembly","action": "deploy" }})
        .expect(200)
        .end(function(err, res) {
          globalAny.job_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });
    it('update job status', function(done) {
      request.put('/jobs/'+globalAny.job_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"ready","message":"","reason":"","conditions":[{"condition_type":"","message":"","reason":"","status":"ready","last_update_time":"","last_transition_time":"","last_probe_time":""}]}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the created job node id type mismatch', function(done) {
      request.post('/jobs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"loadbalancer","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"877634565345","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"spec": {"node_id":123,"group":"assembly","action": "deploy" }})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns the created job missing name', function(done) {
      request.post('/jobs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"877634565345","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"spec": {"node_id": globalAny.node_id,"group":"assembly","action": "deploy" }})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all jobs', function(done) {
      request.get('/jobs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('invalid url for all jobs get', function(done) {
      request.get('/job')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the jobs by node id', function(done) {
      request.get('/jobs/node?node_id='+globalAny.node_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.type_meta.kind).to.equal(globalAny.joblist);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });
    it('returns the jobs by wrong node id', function(done) {
      request.get('/jobs/node?node_id='+globalAny.job_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the jobs by wrong url', function(done) {
      request.get('/jobs/node&node_id='+globalAny.node_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the jobs by wrong node type', function(done) {
      request.get('/jobs/node?node_id=9876yuh')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
