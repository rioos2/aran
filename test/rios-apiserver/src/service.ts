import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Service account API', function() {
  describe('EndPoints API', function() {
    it('returns the created end points', function(done) {
      request.post('/endpoints')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {	"kind": "Endpoint","api_version": "v1" }, "target_ref": "835982843296366592", "object_meta": {"name": "xyz",	"origin": "rioo","uid":"","created_at": "","cluster_name": "","labels": { 	"group": "development",	"key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}}, "subsets": {"addresses": [{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10"}],"not_ready_addresses": [{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11"}],"ports": [{ "name": "", "port": "","protocol":"tcp/udp/http"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          globalAny.target_ref=res.body.target_ref;
          globalAny.origin=res.body.object_meta.origin;
          done(err);
        });
    });
    it('returns all endpoints', function(done) {
      request.get('/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {

          expect(res.body);
          done(err);
        });
    });
    it('returns  endpoint', function(done) {
      request.get('/endpoints/'+globalAny.endpoints_id)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.endpoints_id);
          done(err);
        });
    });
    it('returns  endpoints list by orgin', function(done) {
      request.get('/origins/'+globalAny.origin+'/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  endpoints list by orgin', function(done) {
      request.get('/assemblys/'+globalAny.target_ref+'/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

  });
  describe('Services API', function() {
    it('returns the created services', function(done) {
      request.post('/services')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": "rioo","uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :"835982843296366592" },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","external_name": ""},"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.services_id =res.body.id;
          done(err);
        });
    });
    it('returns all services', function(done) {
      request.get('/services')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {

          expect(res.body);
          done(err);
        });
    });
    it('returns  services', function(done) {
      request.get('/services/'+globalAny.services_id)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.services_id);
          done(err);
        });
    });
    it('returns  services list by orgin', function(done) {
      request.get('/origins/rioo/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by orgin', function(done) {
      request.get('/assemblys/835982843296366592/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

  });

  });
