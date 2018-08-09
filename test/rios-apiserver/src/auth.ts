// account test case total -9
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
//------marketplace creation using marketplaceServer
// const request = supertest.agent(globalAny.marketplaceServer);

//------ apiserver api creation using apiServer
const request = supertest.agent(globalAny.apiServer);

describe('Authorization API', function() {

describe('User authenticate API', function() {
  it('returns the created user account', function(done) {
    this.timeout(4000)
    request.post('/accounts')
      //---------------using api server using server-ca.cert.pem
    .ca(globalAny.rootCA)
        //---------------using marketplace server using client-appstores.cert.pem
      // .ca(globalAny.rootMarketplaceCA)
      .send({"email":"info@riocorp.io","teams":["RIOOS:SUPERUSER"],"first_name":"vino","last_name": "v","phone":"9994048897","company_name": "megam","password": "team4riocorp","registration_ip_address": "192.168.1.10","object_meta":{"name":"info@riocorp.io","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
      .expect(200)
      .end(function(err, res) {
        globalAny.account_id =res.body.id;
        globalAny.email = res.body.email;
        globalAny.token = res.body.token;
        globalAny.bobo_bearer = "Bearer " + globalAny.token;
        expect(res.body.teams[0]).to.equal("RIOOS:SUPERUSER");
        expect(res.body.type_meta.kind).to.equal(globalAny.account);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        expect(res.body.object_meta.name).to.equal(globalAny.email);
        done()
      });
  });
  it('returns the account by id', function(done) {
    request.get('/accounts/' + globalAny.account_id)
    .ca(globalAny.rootCA)
    .set('Authorization', globalAny.bobo_bearer)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
       expect(res.body.id).to.equal(globalAny.account_id);
       expect(res.body.type_meta.kind).to.equal(globalAny.account);
       expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done()
      });
  });

  it('returns the account by email', function(done) {
    request.get('/accounts/name/' + globalAny.email)
    .ca(globalAny.rootCA)
    .set('Authorization', globalAny.bobo_bearer)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {

       expect(res.body.email).to.equal(globalAny.email);
       expect(res.body.type_meta.kind).to.equal(globalAny.account);
       expect(res.body.type_meta.api_version).to.equal(globalAny.version);
       done()
      });
  });

  it('returns error without header account_get_by_id', function(done) {
    request.get('/accounts/' + globalAny.account_id)
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns error without header for account_get_by_name', function(done) {
    request.get('/accounts/name/' + globalAny.email)
    .ca(globalAny.rootCA)
      .expect(401)
      .end(function(err, res) {
        done()
      });
  });

  it('returns the Malformed body error', function(done) {
    request.post('/accounts')
    .ca(globalAny.rootCA)
      .send({"email":"info@riocorp.io","":"vino","last_name": "v","phone":"9994048897","company_name": "megam","password": "team4riocorp","registration_ip_address": "192.168.1.10",
      "object_meta":{"name":"info@riocorp.io","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],
      "created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
      .expect(400)
      .end(function(err, res) {
          expect(res.body);
        done()
      });
  });

  it('returns the BadRequest error', function(done) {
    request.post('/accounts')
    .ca(globalAny.rootCA)
      .send({"email":"","first_name":"vino","last_name": "v","phone":"9994048897","company_name": "megam","password": "team4riocorp","registration_ip_address": "192.168.1.10",
      "object_meta":{"name":"info@riocorp.io","account":"","labels":{},"annotations":{},
      "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
      .expect(400)
      .end(function(err, res) {
          expect(err);
        done()
      });
  });

  it('returns url Not Found error', function(done) {
    request.post('/accoun')
    .ca(globalAny.rootCA)
      .send({"email":"info@riocorp.io","first_name":"vino","last_name": "v",
      "phone":"9994048897","company_name": "megam","password": "team4riocorp","registration_ip_address": "192.168.1.10",
      "object_meta":{"name":"info@riocorp.io","account":"","labels":{},"annotations":{},
      "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
      .expect(404)
      .end(function(err, res) {
          expect(err);
        done()
      });
  });
  //
  // it('returns Record Not Found error', function(done) {
  //   request.get('/accounts/123456789' )
  //   .ca(fs.readFileSync('/home/vinov/rioos/home/config/server-ca.cert.pem'))
  //   .set('Authorization', globalAny.bobo_bearer)
  //   .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  //     .expect(404)
  //     .end(function(err, res) {
  //         expect(err);
  //       done();
  //     });
  // });

  // it('returns login user ', function(done) {
  //   request.post('/authenticate')
  //     .ca(fs.readFileSync('/home/vinov/rioos/home/config/server-ca.cert.pem'))
  //     .send({"email": globalAny.email, "password":"team4riocorp"})
  //     .expect(200)
  //     .end(function(err, res) {
  //       expect(res.body.id).to.equal(globalAny.account_id);
  //       done();
  //     });
  // });
  //
  // it('returns unauthorised error ', function(done) {
  //   request.post('/authenticate')
  //   .ca(fs.readFileSync('/home/vinov/rioos/home/config/server-ca.cert.pem'))
  //     .send({"email": globalAny.email, "password":"team4rioc"})
  //     .expect(401)
  //     .end(function(err, res) {
  //       done();
  //     });
  // });

  });
  });
