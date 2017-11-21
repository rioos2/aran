import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Job  API', function() {

    it('returns the created network', function(done) {
      request.post('/networks')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"network_type":"ip4","name":"private","subnet_ip":"192.168.1.0/24","netmask": "255.255.255.0","gateway": "192.168.1.1","bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},"created_at": "","status": {"phase":"pending","message":"","reason":"","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":" ","condition_type":" "}]}})
        .expect(200)
        .end(function(err, res) {
          globalAny.network_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it('returns all networks', function(done) {
      request.get('/networks')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });


  });
