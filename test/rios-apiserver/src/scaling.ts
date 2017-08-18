import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/v1');
const globalAny:any = global;

describe('Scaling API', function() {
  describe('Horizontal Scaling API', function() {
    it('returns the created horizontal_scaling', function(done) {
      request.post('/horizontal_scaling')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"name":"example1","description":"Horizontal auto scale","tags":["horizontal scale","loadbalancer"],"scale_type":"AUTOHS","representation_skew":"ACTIVE","state":"data","metadata":[], "spec":{"scale_target_ref":"ASM001","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by":"5m","scale_up_wait_time":"5m","scale_down_by":"5m","scale_down_wait_time":"5m"}},"resource":{"name": "memory","min_target_value":"2","max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_up_wait_time":"5m","scale_down_by":"5m","scale_down_wait_time":"5m"}}}]},"status":{"last_scale_time":"","current_replicas":1,"desired_replicas":1}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.name).to.equal("example1");
          globalAny.hs_id =res.body.id;
          done(err);
        });
    });

    it('returns the horizontal_scaling_status_update', function(done) {
      request.put('/horizontal_scaling/status/'+ globalAny.hs_id)
        .set('Authorization', globalAny.bobo_bearer)
        .send({"status":{"last_scale_time":"sdgd","current_replicas":3,"desired_replicas":3}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
          done(err);
        });
    });
    it('returns all horizontal_scaling', function(done) {
      request.get('/horizontal_scaling')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
            expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
  });
  });
