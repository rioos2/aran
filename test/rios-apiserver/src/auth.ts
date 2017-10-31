import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Authorization API', function() {

describe('User authenticate API', function() {
  it('returns the created user account', function(done) {
    request.post('/accounts')
      .send({"name": "mega","email":"infomega","first_name":"vino1","last_name": "v","phone":"9994048897","api_key": "1234567890","password": "vino123","states":"safa","approval":"zfdgdg","suspend":"true","registration_ip_address": "","roles":["role/rios:superuser"]})
      .expect(200)
      .end(function(err, res) {
        globalAny.acc_id =res.body.id;
        globalAny.email = res.body.email;
        globalAny.bobo_bearer = "Bearer ydukl6BhNeJi5V6pT5";
        done(err);
      });
  });
  it('returns the account by id', function(done) {
    request.get('/accounts/' + globalAny.acc_id)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.role_id);
        done(err);
      });
  });

  it('returns the account by id', function(done) {
    request.get('/accounts/' + globalAny.role_id)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.role_id);
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


  });


  describe('User Roles API', function() {
    it('returns the created roles', function(done) {
      request.post('/roles')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"name": "ubunturole1/rios:superuser","description":"uperuser of RIO/OS. God given powers.  instance"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("ubunturole1/rios:superuser");
          globalAny.role_id =res.body.id;
          done(err);
        });
    });

    it('returns the role by id', function(done) {
      request.get('/roles/' + globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.role_id);
          done(err);
        });
    });

    it('returns the all roles', function(done) {
      request.get('/roles')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
      //  expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
  });

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
      request.get('/permissions/' + globalAny.perm_id)
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
      //  expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });

  });


  });
