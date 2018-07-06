//15 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('License  API', function() {

  it('returns all licenses', function(done) {
    request.get('/license')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.items.length).to.equal(1);
        expect(res.body.kind).to.equal(globalAny.Licenses);
        expect(res.body.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

    it('returns the update license', function(done) {
      request.post('/license/activate')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"SoftwareKey"},"status":"trial","product":"Rio/OS","activation_code":"ertyuicvbnm456789dfghjk456789","expired_at":"30"})
        .expect(200)
        .end(function(err, res) {
          globalAny.license_id =res.body.id;
          globalAny.license_name =res.body.object_meta.name;
          expect(res.body.kind).to.equal(globalAny.License);
          expect(res.body.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('returns the create licenses missing status', function(done) {
      request.post('/license/activate')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"SoftwareKey"},"status":"","product":"Rio/OS","activation_code":"ertyuicvbnm456789dfghjk456789","expired_at":"30"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the create licenses missing status', function(done) {
      request.post('/license/activate')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"SoftwareKey"},"status":"trial","product":"Rio/OS","activation_code":"ertyuicvbnm456789dfghjk456789","expired_at":"30"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created job missing activation_code', function(done) {
      request.post('/license/activate')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"SoftwareKey"},"status":"trial","product":"Rio/OS","activation_code":"","expired_at":"30"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created job missing product', function(done) {
      request.post('/license/activate')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"SoftwareKey"},"status":"trial","product":"","activation_code":"ertyuicvbnm456789dfghjk456789","expired_at":"30"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('invalid url for all license get', function(done) {
      request.get('/licenses')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('license get by name', function(done) {
      request.get('/licenses/'+ globalAny.license_name)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          globalAny.license_name =res.body.object_meta.name;
          expect(res.body.kind).to.equal(globalAny.License);
          expect(res.body.api_version).to.equal(globalAny.version);
          
          expect(res.body);
          done(err);
        });
    });

    it('license get by invalid name', function(done) {
      request.get('/licenses/Nalperian')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

      });
