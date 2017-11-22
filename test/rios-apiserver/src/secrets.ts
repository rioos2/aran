import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

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
