import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

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
