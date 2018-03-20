import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('EndPoints API', function() {
    it('returns the created end points', function(done) {
      request.post('/endpoints')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"subsets":{"addresses":[{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10","mac_address":"00:0a:95:9d:68:16"}],"unready_addresses" :[{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11","mac_address":"00:0a:95:9d:68:17"}],
        "ports": [{ "name": "", "port": "","protocol":"tcp"}]},"object_meta":{"name":"endpnt1","account":"","labels":{},"annotations":{},
        "owner_references":[{"kind":"","api_version":"","name":"","uid":globalAny.assembly_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          globalAny.target_ref=res.body.object_meta.owner_references[0].uid;
          expect(res.body.type_meta.kind).to.equal(globalAny.endpoint);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns all endpoints', function(done) {
      request.get('/endpoints')
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.kind).to.equal(globalAny.endpointlist);
          expect(res.body.api_version).to.equal(globalAny.version);
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });
    it('returns endpoint by id', function(done) {
      request.get('/endpoints/'+globalAny.endpoints_id)
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.endpoints_id);
        expect(res.body.type_meta.kind).to.equal(globalAny.endpoint);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns  endpoints list by assembly', function(done) {
      request.get('/endpoints/assembly/'+globalAny.target_ref)
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.object_meta.owner_references[0].uid).to.equal(globalAny.target_ref);
          expect(res.body.type_meta.kind).to.equal(globalAny.endpoint);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns Malformed error subsets body ', function(done) {
      request.post('/endpoints')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"subsets":{"":[{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10","mac_address":"00:0a:95:9d:68:16"}],"unready_addresses" :[{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11","mac_address":"00:0a:95:9d:68:17"}],"ports": [{ "name": "", "port": "","protocol":"tcp"}]},"object_meta":{"name":"endpnt1","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"872235286753452032","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns Badrequest error for uid ', function(done) {
      request.post('/endpoints')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"subsets":{"addresses":[{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10","mac_address":"00:0a:95:9d:68:16"}],"unready_addresses" :[{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11","mac_address":"00:0a:95:9d:68:17"}],"ports": [{ "name": "", "port": "","protocol":"tcp"}]},"object_meta":{"name":"endpnt1","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns Badrequest error for subsets ', function(done) {
      request.post('/endpoints')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"subsets":{"addresses":[],"unready_addresses" :[],"ports": []},"object_meta":{"name":"endpnt1","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"872235286753452032","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns Record Not Found endpoint', function(done) {
      request.get('/endpoints/23456789')
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns Record Not Found endpoin get by assembly', function(done) {
      request.get('/assemblys/endpoints/23456789')
      .ca(globalAny.rootCA)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for create end points', function(done) {
      request.post('/endpoints')
      .ca(globalAny.rootCA)
        .send({"subsets":{"addresses":[{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10","mac_address":"00:0a:95:9d:68:16"}],"unready_addresses" :[{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11","mac_address":"00:0a:95:9d:68:17"}],"ports": [{ "name": "", "port": "","protocol":"tcp"}]},"object_meta":{"name":"endpnt1","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"872235286753452032","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for list endpoints', function(done) {
      request.get('/endpoints')
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for show  endpoint', function(done) {
      request.get('/endpoints/'+globalAny.endpoints_id)
      .ca(globalAny.rootCA)
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });
  });
