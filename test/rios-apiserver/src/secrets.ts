import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('secrets  API', function() {

  it('returns the created secrets', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/secrets')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "root","data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},"type_meta":{"kind":"Secret","api_version":"v1"}, "object_meta":{"name":"xyz","origin":globalAny.origin_id,"uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
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
        done(err);
      });
  });



  });
