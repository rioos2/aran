import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Service account API', function() {

it('returns the created user account', function(done) {
  request.post('/accounts')
    .send({"name": "mega","email":"inf","first_name":"vino","last_name": "v","phone":"9994048897","api_key": "1234567890","password": "vino123","states":"safa","approval":"zfdgdg","suspend":"true","registration_ip_address": "","roles":["role/rios:superuser"]})
    .expect(200)
    .end(function(err, res) {
      globalAny.acc_id =res.body.id;
      globalAny.email = res.body.email;
      globalAny.bobo_bearer = 'Bearer '+res.body.token;
      done(err);
    });
});
it('returns the created origin', function(done) {
  request.post('/origins')
    .send({"type_meta":{"kind":"Origin","api_version":"v1"}, "object_meta":{"name":"megam","origin":"riooss","uid":globalAny.acc_id,"created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
    .expect(200)
    .end(function(err, res) {
      globalAny.origin_id=res.body.object_meta.origin;
      done(err);
    });
});
it('returns the created assembly', function(done) {
   request.post('/assemblys')
     .set('Authorization', globalAny.bobo_bearer)
     .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
     .send({"name": "ubuntu", "origin": globalAny.origin_id,"uri": "/v1/assemblys","instance_id": "dawn-cloud-a3d34d.megambox.gom", "description": "ubuntuinstallation", "parent_id": "811199221985189888", "tags": ["ubuntu"],"node": "","ips": {"private_ipv4":["198.168.0.15","192.168.0.25"]},"volumes": [{"id":"","target":"",  "volume_type":"" }], "urls": {"vnc_console":"http://10.0.0.1:8969/"},   "status": {"phase": "pending", "message": "","reason": "","conditions": [{"message": "","reason": "","status": " ","last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
     .expect(200)
     .end(function(err, res) {
       expect(res.body.name).to.equal("ubuntu");
       globalAny.asm = res.body;
       globalAny.asm_id = res.body.id;
       done(err);
     });
 });




  describe('EndPoints API', function() {
    it('returns the created end points', function(done) {
      request.post('/endpoints')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {	"kind": "Endpoint","api_version": "v1" }, "target_ref": globalAny.asm_id, "object_meta": {"name": "xyz",	"origin":globalAny.origin_id,"uid":"","created_at": "","cluster_name": "","labels": { 	"group": "development",	"key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}}, "subsets": {"addresses": [{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10"}],"not_ready_addresses": [{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11"}],"ports": [{ "name": "", "port": "","protocol":"tcp/udp/http"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          globalAny.target_ref=res.body.target_ref;

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
      request.get('/origins/'+globalAny.origin_id+'/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  endpoints list by assembly', function(done) {
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
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": globalAny.origin_id,"uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :globalAny.asm_id },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
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
      request.get('/origins/'+globalAny.origin_id+'/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by assembly', function(done) {
      request.get('/assemblys/'+globalAny.asm_id+'/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the created services', function(done) {
      request.post('/services')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": "","uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :"835982843296366592" },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created end points', function(done) {
      request.post('/endpoints')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {	"kind": "Endpoint","api_version": "v1" }, "target_ref": globalAny.asm_id, "object_meta": {"name": "xyz",	"origin":"","uid":"","created_at": "","cluster_name": "","labels": { 	"group": "development",	"key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}}, "subsets": {"addresses": [{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10"}],"not_ready_addresses": [{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11"}],"ports": [{ "name": "", "port": "","protocol":"tcp/udp/http"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          globalAny.target_ref=res.body.target_ref;

          done(err);
        });
    });

    it('returns the created services', function(done) {
      request.post('/services')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_meta": {"name": "xyz","origin": globalAny.origin_id,"uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :"" },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns the created end points', function(done) {
      request.post('/endpoints')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {	"kind": "Endpoint","api_version": "v1" }, "target_ref": "", "object_meta": {"name": "xyz",	"origin":globalAny.origin_id,"uid":"","created_at": "","cluster_name": "","labels": { 	"group": "development",	"key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}}, "subsets": {"addresses": [{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10"}],"not_ready_addresses": [{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11"}],"ports": [{ "name": "", "port": "","protocol":"tcp/udp/http"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          globalAny.target_ref=res.body.target_ref;

          done(err);
        });
    });
    it('returns the created services', function(done) {
      request.post('/services')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Service","api_version": "v1"  },"object_me": {"name": "xyz","origin": globalAny.origin_id,"uid": "","created_at": "","cluster_name": "","labels": {  "group": "development",  "key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"spec": {"selector": { "rioos_assembly_factory_id" :"835982843296366592" },"service_type": "LoadBalancer/ExternalName","loadbalancer_ip": "","names": {"private_name":"levis-01.megam.io"},"external_names": {"public_name":"levis-01.megam.io"} },"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "", "reason": "", "status": " ", "last_transition_time": " ", "last_probe_time": "","condition_type": " "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });



    it('returns  endpoints list by assembly', function(done) {
      request.get('/assemblys/835982843296366591/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  endpoints list by assembly', function(done) {
      request.get('/assemblys/83598284329636659o/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by assembly', function(done) {
      request.get('/assemblys/835982843296366591/services')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by assembly', function(done) {
      request.get('/assemblys/83598284329636659o/services')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  endpoints list by origin', function(done) {
      request.get('/origins/835982843296366591/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  endpoints list by origin', function(done) {
      request.get('/origins/83598284329636659o/endpoints')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by origins', function(done) {
      request.get('/origins/835982843296366591/services')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns  services list by origins', function(done) {
      request.get('/origins/83598284329636659o/services')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns  services', function(done) {
      request.get('/services/835982843296366591')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns  services', function(done) {
      request.get('/services/83598284329636659e')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns  endpoints', function(done) {
      request.get('/endpoints/835982843296366591')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns  endpoints', function(done) {
      request.get('/endpoints/83598284329636659e')
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

  });
  describe('secrets  API', function() {

  it('returns the created secrets', function(done) {
    request.post('/secrets')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"type_meta":{"kind":"ServiceAccount","api_version":"v1"},"object_meta":{"name":"xyz","origin":globalAny.origin_id,"uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        globalAny.secrets_id =res.body.id;
        done(err);
      });
  });
  it('returns all secrets', function(done) {
    request.get('/secrets')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {

        expect(res.body);
        done(err);
      });
  });
  it('returns  secrets', function(done) {
    request.get('/secrets/'+globalAny.secrets_id)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
      expect(res.body.id).to.equal(globalAny.secrets_id);
        done(err);
      });
  });

  it('returns  secrets list by orgin', function(done) {
    request.get('/origins/'+globalAny.origin_id+'/secrets')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });
  it('returns  secrets', function(done) {
    request.get('/secrets/835982843296366591')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });
  it('returns  secrets', function(done) {
    request.get('/secrets/83598284329636659e')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });
  it('returns the created secrets', function(done) {
    request.post('/secrets')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"type_meta":{"kind":"ServiceAccount","api_version":"v1"},"object_meta":{"name":"xyz","origin":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });

  it('returns  secrets list by origins', function(done) {
    request.get('/origins/835982843296366591/secrets')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  secrets list by origins', function(done) {
    request.get('/origins/83598284329636659o/secrets')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  });
  describe('Service account API', function() {

  it('returns the created serviceaccounts', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/serviceaccounts/812345678909')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"type_meta":{"kind":"ServiceAccount","api_version":"v1"},"object_meta":{"name":"xyz","origin":globalAny.origin_id,"uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        globalAny.secrets_id =res.body.id;
        done(err);
      });
  });


  it('returns  secrets list by orgin', function(done) {
    request.get('/origins/'+globalAny.origin_id+'/serviceaccounts')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the created secrets', function(done) {
    request.get('/serviceaccounts')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });

  });


  });
