import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('secrets  API', function() {

  it('returns the created secrets', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "opaque","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "object_meta": {"name":"ca","account":globalAny.account_id}})
      .expect(200)
      .end(function(err, res) {
        globalAny.secrets_id = res.body.id;
        expect(res.body);
        done(err);
      });
  });
  it('returns all secrets', function(done) {
    request.get('/secrets')
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.items.length).to.equal(3);
        done(err);
      });
  });
  it('returns  secrets', function(done) {
    request.get('/secrets/'+globalAny.secrets_id)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.secrets_id);
        done(err);
      });
  });

  it('returns all secrets account based', function(done) {
    request.get('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.items.length).to.equal(1);
        done(err);
      });
  });

  it('returns Bad Request error for secret_type is empty', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"ca","account":globalAny.account_id}})
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  Bad Request error for ObjectMeta name is empty', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "opaque","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"","account":globalAny.account_id}})
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });


  it('returns internal error if no secret type match', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "root","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"sdfg","account":globalAny.account_id}})
      .expect(500)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns malformed error  if no secret type field', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"": "root","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"sdfg","account":globalAny.account_id}})
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns malformed error  if no data field', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"secret_type": "root","metadata":{"origin":"rioos_system"},"": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"sdfg","account":globalAny.account_id}})
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns 404 error for secret get by account', function(done) {
    request.get('/accounts/1234567/secrets')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns 404 for show secrets', function(done) {
    request.get('/secrets/12345678')
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the unauthorized error created secrets', function(done) {
    request.post('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .send({"secret_type": "opaque","metadata":{"origin":"rioos_system"},"data": {"username": "USERNAME","password": "PASSWORD","rsa_key": "PRIVATEKEY","rsa_pub": "PUBLICKEY","tls_key": "PRIVATEKEY", "tls_pub": "PUBLICKEY","<anykey>": "<any value>"},
      "type_meta":{"kind":"Secret","api_version":"v1"},
      "object_meta": {"name":"ca","account":globalAny.account_id}})
      .expect(401)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  the unauthorized error secrets', function(done) {
    request.get('/secrets/'+globalAny.secrets_id)
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the unauthorized error all secrets account based', function(done) {
    request.get('/accounts/'+globalAny.account_id+'/secrets')
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done(err);
      });
  });

});
