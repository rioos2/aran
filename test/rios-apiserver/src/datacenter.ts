import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Datacenter  API', function() {
    it('returns the created datacenter', function(done) {
      request.post('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[globalAny.network_id],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk", "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body.object_meta.name).to.equal("chennai");
         globalAny.datacenter_id =res.body.id;
        done(err);
        });
    });

    it('returns the created datacenter stoarge empty', function(done) {
      request.post('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[globalAny.network_id],"enabled": true,"storage": "","advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk", "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
        done(err);
        });
    });

    it('returns the created datacenter networks missing', function(done) {
      request.post('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk", "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
        done(err);
        });
    });

    it('returns the created datacenter node missing', function(done) {
      request.post('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [],"networks":[globalAny.network_id],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk", "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
        done(err);
        });
    });

    it('returns the created datacenter name missing', function(done) {
      request.post('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[globalAny.network_id],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk", "last_update_time":""}]}, "object_meta":{"name":"","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
        done(err);
        });
    });

    it('returns all datacenters', function(done) {
      request.get('/datacenters')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('returns all datacenters for invalid url', function(done) {
      request.get('/datacenter')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns all datacenters for without header', function(done) {
      request.get('/datacenters')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the datacenters by id', function(done) {
      request.get('/datacenters/' + globalAny.datacenter_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.datacenter_id);
          done(err);
        });
    });

    it('returns the datacenters by wrong id type', function(done) {
      request.get('/datacenters/890765uikj4345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update datacenter', function(done) {
      request.put('/datacenters/'+ globalAny.datacenter_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[globalAny.network_id],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending",
        "conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk",
        "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"",
        "deletion_grace_period_seconds":0,"finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
        done(err);
        });
    });
    it('update datacenter invalid id', function(done) {
      request.put('/datacenters/8765432345678')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"nodes": [globalAny.node_id],"networks":[globalAny.network_id],"enabled": true,"storage": globalAny.st_id,"advanced_settings":{},"flag":"ch.png","currency": "rs","status":{"message":"","reason":"","phase": "pending",
        "conditions": [{"message":"nodelet has sufficient disk space available", "reason":"NodeletHasSufficientDisk", "status":"False","last_transition_time":"2017-09-21T06:35:16Z", "last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk",
        "last_update_time":""}]}, "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"",
        "deletion_grace_period_seconds":0,"finalizers":[],"cluster_name":""}})
        .expect(404)
        .end(function(err, res) {
        done(err);
        });
    });


  });
