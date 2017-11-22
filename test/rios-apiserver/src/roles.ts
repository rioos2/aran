import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

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





  });
