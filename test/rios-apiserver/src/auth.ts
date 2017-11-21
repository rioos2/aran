import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Authorization API', function() {

describe('User authenticate API', function() {
  it('returns the created user account', function(done) {
    request.post('/accounts')
      .send({"name": "megam","email":"info@riocorp1.io","first_name":"vino","last_name": "v","phone":"9994048897","api_key": "1234567890","password": "vino123","states":"safa","approval":"zfdgdg","suspend":"true","registration_ip_address": "","roles":["role/rios:superuser"]})
      .expect(200)
      .end(function(err, res) {
        globalAny.acc_id =res.body.id;
        globalAny.name = res.body.name;
        globalAny.email = res.body.email;

        globalAny.bobo_bearer = "Bearer ydukl6BhNeJi5V6pT5";
        done(err);
      });
  });
  it('returns the account by id', function(done) {
    request.get('/accounts/' + globalAny.acc_id)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.acc_id);
        done(err);
      });
  });

  it('returns the created origin', function(done) {
    request.post('/origins')
      .send({"type_meta":{"kind":"Origin","api_version":"v1"}, "object_meta":{"name":"megam","origin":"rioos","uid":globalAny.acc_id,"created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"}}})
      .expect(200)
      .end(function(err, res) {
      globalAny.origin_id=res.body.object_meta.origin;
        done(err);
      });
  });

  it('returns the origin by name', function(done) {
    request.get('/origins/' + globalAny.origin_id)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.object_meta.origin).to.equal(globalAny.origin_id);
        done(err);
      });
  });

  it('returns the all origin', function(done) {
    request.get('/origins')
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });


  });
  });
