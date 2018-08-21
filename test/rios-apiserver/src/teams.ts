//10 testcases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('User Teams API', function() {
   it('returns the created teams', function(done) {
     request.post('/teams')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "RIOOS:TESTER","description":"tester of RIO/OS.","object_meta": {"account":globalAny.account_id},"metadata": {"origin":"rioos_system"}})
       .expect(200)
       .end(function(err, res) {
         expect(res.body.name).to.equal("RIOOS:TESTER");
         globalAny.team_id =res.body.id;
         globalAny.team_name =res.body.name;
         done(err);
       });
   });

   it(' created teams empty name', function(done) {
     request.post('/teams')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "","description":"superuser of RIO/OS. God given powers.  instance","object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it(' created teams empty description', function(done) {
     request.post('/teams')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"name": "ubuntuteam1_rios:superuser","description":"","object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it(' created teams missing name parameter', function(done) {
     request.post('/teams')
     .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .send({"description":"superuser of RIO/OS. God given powers.  instance","object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}})
       .expect(400)
       .end(function(err, res) {
         expect(res.body);
         done(err);
       });
   });

   it('returns error without header create teams', function(done) {
     request.get('/teams')
     .ca(globalAny.rootCA)
     .send({"name": "ubuntuteam1_rios:superuser","description":"superuser of RIO/OS. God given powers.  instance","object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}})
      .expect(406)
       .end(function(err, res) {
         done(err);
       });
   });


   it('returns the team by id', function(done) {
     request.get('/teams/'+globalAny.team_id)
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.team_id);
         done(err);
       });
   });

   it('returns the team by  wrong id', function(done) {
     request.get('/teams/98765432123411')
      .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

   it('returns the all teams', function(done) {
     request.get('/teams')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
       expect(res.body.items.length).to.equal(4);
       expect(res.body.kind).to.equal(globalAny.teamlist);
       expect(res.body.api_version).to.equal(globalAny.version);
         done(err);
       });
   });

   it('returns the all teams invalid url', function(done) {
     request.get('/team')
      .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

   it('returns the team by name', function(done) {
     request.get('/teams/name/'+globalAny.team_name)
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(200)
       .end(function(err, res) {
        expect(res.body.name).to.equal(globalAny.team_name);
         done(err);
       });
   });

   it('returns the team by wrong name', function(done) {
     request.get('/teams/name/rioos_user')
       .ca(globalAny.rootCA)
       .set('Authorization', globalAny.bobo_bearer)
       .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
       .expect(404)
       .end(function(err, res) {
         done(err);
       });
   });

 });
