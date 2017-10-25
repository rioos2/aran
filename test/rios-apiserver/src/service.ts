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
        .send({"type_meta": {	"kind": "Endpoint","api_version": "v1" }, "target_ref": "835359020318466048", "object_meta": {"name": "xyz",	"origin": "rioo","uid":"","created_at": "","cluster_name": "","labels": { 	"group": "development",	"key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}}, "subsets": {"addresses": [{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10"}],"not_ready_addresses": [{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11"}],"ports": [{ "name": "", "port": "","protocol":"tcp/udp/http"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.endpoints_id =res.body.id;
          done(err);
        });
    });
    it('returns all endpoints', function(done) {
      request.get('/endpoints')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
    it('returns  endpoint', function(done) {
      request.get('/endpoints/'+globalAny.endpoints_id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.endpoints_id);
          done(err);
        });
    });

  });
  });
