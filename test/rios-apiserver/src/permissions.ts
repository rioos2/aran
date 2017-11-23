import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('User Permission API', function() {
    it('returns the created permission', function(done) {
      request.post('/permissions')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":globalAny.role_id,"name": "rioos.assembly.get","description":"Read only access to all the users  VMs, Containers"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          globalAny.perm_id =res.body.id;
          done(err);
        });
    });

    it('returns role based permission', function(done) {
      request.get('/permissions/roles/'+ globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          done(err);
        });
    });

    it('returns the permission by id', function(done) {
      request.get('/permissions/'+ globalAny.perm_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.perm_id);
          done(err);
        });
    });

    it('returns the specfic permission for the specfic role', function(done) {
      request.get('/permissions/' + globalAny.perm_id + '/roles/' + globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.perm_id);
         expect(res.body.role_id).to.equal( globalAny.role_id);
          done(err);
        });
    });

    it('returns the all permission', function(done) {
      request.get('/permissions')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
      expect(res.body.items.length).to.equal(5);
          done(err);
        });
    });

  });
