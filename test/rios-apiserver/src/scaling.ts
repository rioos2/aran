import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Scaling API', function() {
  describe('Horizontal Scaling API', function() {
    it('returns the created horizontal_scaling', function(done) {
      request.post('/horizontalscaling')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"","name":"","uid":globalAny.asm_fac_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"",
        "deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1}, "scale_type":"AUTOHS","state":"data", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,
        "metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},
        "resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
        .expect(200)
        .end(function(err, res) {
          globalAny.hscale_id =res.body.id;
          done(err);
        });
    });

      it('returns the created horizontal_scaling missing name', function(done) {
        request.post('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"ABLETOSCALE", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created horizontal_scaling missing account', function(done) {
        request.post('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":"", "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"ABLETOSCALE", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the created horizontal_scaling owner reference missing', function(done) {
        request.post('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi.megam.io","uid":"","block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"ABLETOSCALE", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created horizontal_scaling missing state', function(done) {
        request.post('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created horizontal_scaling missing scale_type', function(done) {
        request.post('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"","state":"ABLETOSCALE", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m","scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the horizontalscaling status update by id', function(done) {
        request.put('/horizontalscaling/'+globalAny.hscale_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"last_scale_time":"","current_replicas":1,"desired_replicas":2}})
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the horizontalscaling status update by  wrong id', function(done) {
        request.put('/horizontalscaling/'+globalAny.account_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"last_scale_time":"","current_replicas":1,"desired_replicas":2}})
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the horizontalscaling status update by id type mismatch ', function(done) {
        request.put('/horizontalscaling/'+globalAny.hscale_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"last_scale_time":"","current_replicas":"1","desired_replicas":2}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns all horizontalscaling', function(done) {
        request.get('/horizontalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            expect(res.body.items.length).to.equal(1);
            done(err);
          });
      });

      it('returns all horizontalscaling wrong url ', function(done) {
        request.get('/horizontalscalings')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns all horizontalscaling without header ', function(done) {
        request.get('/horizontalscaling')
        .ca(globalAny.rootCA)
          .expect(406)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the horizontalscaling  update by id', function(done) {
        request.put('/horizontalscaling/'+globalAny.hscale_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"lev.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"data", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m", "scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the horizontalscaling  update by wrong  id', function(done) {
        request.put('/horizontalscaling/'+globalAny.account_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"object_meta":{"name": "hzscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"lev.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_replicas":1, "desired_replicas":1},
          "scale_type":"AUTOHS","state":"data", "metadata":{},"spec":{"scale_up_wait_time":"5m","scale_down_wait_time":"5m","min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":{"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by": "5m","scale_down_by" :"5m"}},"resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the horizontalscaling  scale up by id', function(done) {
        request.get('/horizontalscaling/'+globalAny.hscale_id+'/scale')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            expect(res.body.id).to.equal(globalAny.asm_fac_id);
            done(err);
          });
      });

      it('returns the horizontalscaling  scale up by wrong  id', function(done) {
        request.get('/horizontalscaling/862345672345678/scale')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the horizontalscaling  scale up by  id wrong url', function(done) {
        request.get('/scale/horizontalscaling/'+globalAny.hscale_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });



  });
  });
