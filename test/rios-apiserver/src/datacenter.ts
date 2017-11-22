import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Datacenter  API', function() {
    it('returns the created datacenter', function(done) {
      request.post('/datacenters')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"name":"chennai","nodes":["844747261714907136"],"storage": globalAny.stp_id,"networks": ["844751056645668864"],"enabled": true,"advanced_settings":{"storage":"true"},"flag":"ch.png","currency":"rs","status":{"phase":"ready","message": "","reason": "","conditions": [{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":" ","condition_type":" "}]} })
        .expect(200)
        .end(function(err, res) {
         expect(res.body.name).to.equal("chennai");
         globalAny.datacenter_id =res.body.id;
        done(err);
        });
    });
    it('returns all datacenters', function(done) {
      request.get('/datacenters')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
        expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('returns the datacenters by id', function(done) {
      request.get('/datacenters/' + globalAny.datacenter_id)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.datacenter_id);
          done(err);
        });
    });


  });
