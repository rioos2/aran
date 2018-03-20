import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Network  API', function() {

    it('returns the created network', function(done) {
      request.post('/networks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          globalAny.network_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it('returns the created network without name', function(done) {
      request.post('/networks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created network name empty', function(done) {
      request.post('/networks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"name":"","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all networks', function(done) {
      request.get('/networks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });
    it('returns all networks invalid url', function(done) {
      request.get('/network')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns all networks without header', function(done) {
      request.get('/networks')
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the network by id', function(done) {
      request.get('/networks/'+globalAny.network_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.network_id);
          done(err);
        });
    });

    it('returns the wrong network by id', function(done) {
      request.get('/networks/87655434578765')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update networks', function(done) {
      request.put('/networks/'+globalAny.network_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","used_bits":[1],"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('update storage connectors wrong connector id', function(done) {
      request.put('/networks/876890655433')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","used_bits":[1],"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(404)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('update networks missing name', function(done) {
      request.put('/networks/'+globalAny.network_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type": "private_ipv4","subnet_ip": "192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","used_bits":[1],"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},"bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},     "object_meta":{"name":"","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
  });
