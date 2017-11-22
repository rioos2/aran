import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Scaling API', function() {
  describe('Horizontal Scaling API', function() {
    it('returns the created horizontal_scaling', function(done) {
      request.post('/origins/'+globalAny.origin_id+'/horizontalscaling')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"name":"example1","origin": "rioosapi","description":"Horizontal auto scale","tags":["horizontal scale","loadbalancer"],"scale_type":"AUTOHS","representation_skew":"ACTIVE","state":"data","metadata":[], "spec":{"scale_target_ref":"852041862205153280","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by":"5m","scale_up_wait_time":"5m","scale_down_by":"5m","scale_down_wait_time":"5m"}},"resource":{"name": "memory","min_target_value":"2","max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_up_wait_time":"5m","scale_down_by":"5m","scale_down_wait_time":"5m"}}}]},"status":{"last_scale_time":"","current_replicas":1,"desired_replicas":1},"type_meta":{"kind":"Assemblyfactory","api_version":"v1"},"object_meta":{"name":"xyz","namespace":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"},"owner_references": [{"kind":"Node","api_version":"v1","name":"ddd", "uid":"","block_owner_deletion":true}]}})
        .expect(200)
        .end(function(err, res) {
          globalAny.hscale_id =res.body.id;
          done(err);
        });
    });

    it('returns the horizontalscaling status update by id', function(done) {
      request.put('/horizontalscaling/'+globalAny.hscale_id+'/status')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"last_scale_time":"","current_replicas":1,"desired_replicas":1}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    
    it('returns all horizontalscaling', function(done) {
      request.get('/horizontalscaling')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });
    it('returns the horizontal scaling by id', function(done) {
      request.get('/horizontalscaling/assemblyfactorys/852041862205153280')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });


  });
  });
