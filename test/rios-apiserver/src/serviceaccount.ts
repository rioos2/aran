import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Service account API', function() {

  it('returns the created serviceaccounts', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/serviceaccounts/testaccount')
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secrets":[{"kind":"Secret","name":"<name of the secret key>","origin":globalAny.origin_id,"uid":"the id of secret"}] ,"type_meta":{"kind":"ServiceAccount","api_version":"v1"}, "object_meta":{"name":"testaccount","origin":globalAny.origin_id,"uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        globalAny.servacc_id =res.body.id;
        done(err);
      });
  });


  it('returns  serviceaccounts list by orgin', function(done) {
    request.get('/origins/'+globalAny.origin_id+'/serviceaccounts/testaccount')
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the all serviceaccounts', function(done) {
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
