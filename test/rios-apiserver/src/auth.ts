import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Authorization API', function() {
  describe('User Roles API', function() {
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

    it('returns the role by id', function(done) {
      request.get('/roles/' + globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.role_id);
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

  describe('User Permission API', function() {
    it('returns the created permission', function(done) {
      request.post('/permissions')
        .set('Authorization', globalAny.bobo_bearer)
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
        .expect(200)
        .end(function(err, res) {
          expect(res.body.role_id).to.equal(globalAny.role_id);
          done(err);
        });
    });

    it('returns the permission by id', function(done) {
      request.get('/permissions/' + globalAny.perm_id)
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.perm_id);
          done(err);
        });
    });

    it('returns the specfic permission for the specfic role', function(done) {
      request.get('/permissions/' + globalAny.perm_id + '/roles/' + globalAny.role_id)
        .set('Authorization', globalAny.bobo_bearer)
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
        .expect(200)
        .end(function(err, res) {
        expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });

  });

  describe('Ldap config creation API', function()    {
    it('returns the created ldap config', function(done) {
      request.post('/ldap/config')
        .send({"host": "ldap://ldap.example.com","port": "ldap port : eg: 636 as integer","enforce_starttls": true, "use_ldaps":true,"lookup_dn": "cn=admin,dc=megam,dc=org","lookup_password": "chennai28v","user_search": {"search_base": "dc=megam,dc=org","search_filter_template": "cn={username}" },"group_search": {"search_base": "string","search_filter_template": "string","member_attributes": ["string"]},"ca_certs": "if the client needs to access LDAP  using TLS please add it the root chaing Certificate authority(CA) certificate","client_cert": "if the client needs to access LDAP  using TLS then add the certificate"})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    });

  });
