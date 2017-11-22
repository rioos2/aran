import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

  describe('Services API', function() {
    it('returns the created services', function(done) {
      request.post('/origins/'+globalAny.origin_id+'/services')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": globalAny.origin_id,"uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :globalAny.asm_fac_id },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
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
  
    it('update services', function(done) {
      request.put('/services/'+globalAny.services_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"id": globalAny.services_id,"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": globalAny.origin_id,"uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :globalAny.asm_fac_id },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.services_id =res.body.id;
          done(err);
        });
    });

  });
