import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

  describe('Services API', function() {
    it('returns the created services', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"service1","account":"123456789","labels":{},"annotations":{},"owner_references":[{
              "kind":"","api_version":"","name":"","uid":"1234567892345678","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,
              "finalizers":[],"cluster_name":""},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"",
              "last_probe_time":"","condition_type":"","last_update_time":""}]},"spec":{"service_type":"LoadBalancer",
              "loadbalancer_ip":"192.168.1.11","names":{"private_name":"levis-01.megam.io"},"external_names":{"public_name":"levis-01.megam.io"}}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.services_id =res.body.id;
          done(err);
        });
    });

    it('returns  services', function(done) {
      request.get('/services/'+globalAny.services_id)
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.services_id);
          done(err);
        });
    });

    it('returns Malformed for spec', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"service1","account":"123456789","labels":{},"annotations":{},"owner_references":[{
              "kind":"","api_version":"","name":"","uid":"123456789","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,
              "finalizers":[],"cluster_name":""},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"",
              "last_probe_time":"","condition_type":"","last_update_time":""}]},"spec":{"se_type":"LoadBalancer",
              "loadbalancer_ip":"192.168.1.11","names":{"private_name":"levis-01.megam.io"},"external_names":{"public_name":"levis-01.megam.io"}}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns BadRequest owner_references uid is empty', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"service1","account":"123456789","labels":{},"annotations":{},"owner_references":[{
              "kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,
              "finalizers":[],"cluster_name":""},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"",
              "last_probe_time":"","condition_type":"","last_update_time":""}]},"spec":{"service_type":"LoadBalancer",
              "loadbalancer_ip":"192.168.1.11","names":{"private_name":"levis-01.megam.io"},"external_names":{"public_name":"levis-01.megam.io"}}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns BadRequest for objectmeta name', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":"123456789","labels":{},"annotations":{},"owner_references":[{
              "kind":"","api_version":"","name":"","uid":"12345678","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,
              "finalizers":[],"cluster_name":""},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"",
              "last_probe_time":"","condition_type":"","last_update_time":""}]},"spec":{"service_type":"LoadBalancer",
              "loadbalancer_ip":"192.168.1.11","names":{"private_name":"levis-01.megam.io"},"external_names":{"public_name":"levis-01.megam.io"}}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns BadRequest for objectmeta account', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"afsgsh","account":"","labels":{},"annotations":{},"owner_references":[{
              "kind":"","api_version":"","name":"","uid":"123456789","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0,
              "finalizers":[],"cluster_name":""},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"","reason":"","status":"ready","last_transition_time":"",
              "last_probe_time":"","condition_type":"","last_update_time":""}]},"spec":{"service_type":"LoadBalancer",
              "loadbalancer_ip":"192.168.1.11","names":{"private_name":"levis-01.megam.io"},"external_names":{"public_name":"levis-01.megam.io"}}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update services', function(done) {
      request.put('/services/'+globalAny.services_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"spec":{"service_type":"ExternalName","loadbalancer_ip":"","names":{"938035042985189376":"levi.megam.io"},"external_names":{}},"status":{"phase":"Pending","message":"","reason":"","conditions":[]},"metadata":{"rioos_sh_scheduled_node":"936819970396921856"},
        "object_meta":{"name":"levi.megam.io","account":"938033755627462656","created_at":"2018-03-15T04:38:02.063334177+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory","api_version":"v1",
        "name":"levi.megam.io","uid":"938035042624479232","block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"",
        "details":{"name":"","group":"","kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all services', function(done) {
      request.get('/services')
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(2);
          done(err);
        });
    });


    it('returns Record not found for service', function(done) {
      request.get('/services/12345678')
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for list services', function(done) {
      request.get('/services')
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns unauthorized error for create services', function(done) {
      request.post('/services')
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for show services', function(done) {
      request.get('/services/1234567')
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });
  });
