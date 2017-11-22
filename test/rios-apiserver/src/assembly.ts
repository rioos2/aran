import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Deployment API', function() {

  describe('Assembly API', function() {

    it('returns the assembly by id', function(done) {
      request.get('/assemblys/'+ globalAny.asm_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.asm_id);
          done(err);
        });
    });

    it('returns the assemblys status update by id', function(done) {
      request.put('/assemblys/'+globalAny.asm_id+'/status')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"Initializing","message":"Initializing replica  ubuntu1.","reason":"","conditions":[{"message":"","reason":"","status":"False","last_transition_time":"","last_probe_time":"","condition_type":"AssemblyStorageReady"},{"message":"","reason":"","status":"False","last_transition_time":"","last_probe_time":"","condition_type":"AssemblyNetworkPending"}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
         expect(res.body.id).to.equal(globalAny.asm_id);
          done(err);
        });
    });
    it('returns all assemblys', function(done) {
      request.get('/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.items.length).to.equal(2);
          done(err);
        });
    });

    it('returns the assemblys by origin', function(done) {
      request.get('/origins/'+globalAny.origin_id+'/assemblys')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.items.length).to.equal(2);
          done(err);
        });
    });



  });
});
