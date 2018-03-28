//10 testcases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('User Roles API', function() {
   it('returns the created roles', function(done) {
     request.post('/roles')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "ubunturole1_rios:superuser","description":"superuser of RIO/OS. God given powers.  instance"})
       .expect(200)
       .end(function(err, res) {
         expect(res.body.name).to.equal("ubunturole1_rios:superuser");
         globalAny.role_id =res.body.id;
         globalAny.role_name =res.body.name;
         done(err);
       });
   });

   it(' created roles empty name', function(done) {
     request.post('/roles')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "","description":"superuser of RIO/OS. God given powers.  instance"})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it(' created roles empty description', function(done) {
     request.post('/roles')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "ubunturole1_rios:superuser","description":""})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it(' created roles missing name parameter', function(done) {
     request.post('/roles')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"description":"superuser of RIO/OS. God given powers.  instance"})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it('returns error without header create roles', function(done) {
     request.get('/roles')
     .ca(globalAny.rootCA)
     .send({"name": "ubunturole1_rios:superuser","description":"superuser of RIO/OS. God given powers.  instance"})
      .expect(406)
       .end(function(err, res) {
         done(err);
       });
   });


   it('returns the role by id', function(done) {
     request.get('/roles/'+globalAny.role_id)
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.role_id);
         done(err);
       });
   });

   it('returns the role by  wrong id', function(done) {
     request.get('/roles/98765432123411')
      .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

   it('returns the all roles', function(done) {
     request.get('/roles')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
       expect(res.body.items.length).to.equal(4);
       expect(res.body.kind).to.equal(globalAny.rolelist);
       expect(res.body.api_version).to.equal(globalAny.version);
         done(err);
       });
   });

   it('returns the all roles invalid url', function(done) {
     request.get('/role')
      .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

   it('returns the role by name', function(done) {
     request.get('/roles/name/'+globalAny.role_name)
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
        expect(res.body.name).to.equal(globalAny.role_name);
         done(err);
       });
   });

   it('returns the role by wrong name', function(done) {
     request.get('/roles/name/rioos_user')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

 });
