import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Authorization API', function() {
  describe('Create role for user', function() {
    it('returns the created roles', function(done) {
      request.post('/roles')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"name": "ubunturole/rios:superuser","description":"uperuser of RIO/OS. God given powers.  instance"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("ubunturole/rios:superuser");
          globalAny.role_id =res.body.id;
          done(err);
        });
    });
    it('returns the all roles', function(done) {
      request.get('/roles')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
  });

  describe('Create permission for user', function() {
    it('returns the created permission', function(done) {
      request.post('/permissions')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"role_id":globalAny.role_id,"name": "rioos.assembly.get","description":"Read only access to all the users  VMs, Containers"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          done(err);
        });
    });

    it('returns role based permission', function(done) {
      request.get('/permissions'+ globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          done(err);
        });
    });
  });

  it('returns the all permission', function(done) {
    request.get('/permissions')
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
      expect(res.body.results.length).to.equal(1);
        done(err);
      });
  });

  });
