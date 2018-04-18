//16 testcases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('User Permission API', function() {
    it('returns the created permission', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":globalAny.role_id,"name": "rioos.job.get","description":"Read only access to all the users  VMs, Containers"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          globalAny.perm_id =res.body.id;
          done(err);
        });
    });

    it('created permission name empty ', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":globalAny.role_id,"name": "","description":"Read only access to all the users  VMs, Containers"})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('created permission without name', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":globalAny.role_id,"description":"Read only access to all the users  VMs, Containers"})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('created permission empty role id', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":"","name": "rioos.job.get","description":"Read only access to all the users  VMs, Containers"})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('created permission empty description', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"role_id":globalAny.role_id,"name": "rioos.job.get","description":""})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('created permission without header', function(done) {
      request.post('/permissions')
      .ca(globalAny.rootCA)
        .send({"role_id":globalAny.role_id,"name": "rioos.job.get","description":"Read only access to all the users  VMs, Containers"})
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns role based permission', function(done) {
      request.get('/permissions/roles/'+ globalAny.role_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          expect(res.body.kind).to.equal(globalAny.permissionlist);
          expect(res.body.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('role based permission for wrong role id', function(done) {
      request.get('/permissions/roles/987987987987987')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the permission by id', function(done) {
      request.get('/permissions/'+ globalAny.perm_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.perm_id);
          done(err);
        });
    });

    it('permission show by wrong id', function(done) {
      request.get('/permissions/987654567898765')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the specfic permission for the specfic role', function(done) {
      request.get('/permissions/' + globalAny.perm_id + '/roles/' + globalAny.role_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.perm_id);
         expect(res.body.role_id).to.equal( globalAny.role_id);
          done(err);
        });
    });

    it('specfic permission for the specfic role by wrong role id', function(done) {
      request.get('/permissions/' + globalAny.perm_id + '/roles/89898765432123')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('specfic permission for the specfic role by wrong permission id', function(done) {
      request.get('/permissions/98765432345678/roles/' + globalAny.role_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the all permission', function(done) {
      request.get('/permissions')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(7);
          done(err);
        });
    });

    it('returns the all the  permission for the specfic user', function(done) {
      request.get('/permissions/email/' + globalAny.email)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the all the  permission for the user in wrong user id', function(done) {
      request.get('/permissions/email/info@rio.io')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
