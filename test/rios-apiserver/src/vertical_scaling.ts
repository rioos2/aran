import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Scaling API', function() {
  describe('Vertical Scaling API', function() {
    it('returns the created vertical_scaling', function(done) {
      request.post('/verticalscaling')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "hzscaling", "account":"098765432", "labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"","name":"","uid":globalAny.asm_fac_id,
        "block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{ "cpu":"2","ram":"1000"},
        "desired_resource":{ "cpu":"3","ram":"2000"}},"scale_type":"AUTOHS","state":"data", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{},"max_resource":{},
        "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
        .expect(200)
        .end(function(err, res) {
          globalAny.vscale_id =res.body.id;
          done(err);
        });
    });

    it('returns  metrics limit error', function(done) {
      this.timeout(4000)
      request.get('/verticalscaling/scale/'+globalAny.vscale_id )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(500)
        .end(function(err, res) {
          done(err);
        });
    });

    // it('returns  metrics limit error', function(done) {
    //   request.get('/verticalscaling/scale'+globalAny.vscale_id )
    //     .set('Authorization', globalAny.bobo_bearer)
    //     .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
    //     .expect(200)
    //     .end(function(err, res) {
    //       done(err);
    //     });
    // });

      it('returns the created verticalscaling missing name', function(done) {
        request.post('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
            .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created verticalscaling missing account', function(done) {
        request.post('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":"", "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
            .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the created verticalscaling owner reference missing', function(done) {
        request.post('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":"",
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created verticalscaling missing state', function(done) {
        request.post('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the created vertical_scaling missing scale_type', function(done) {
        request.post('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
            .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the verticalscaling status update by id', function(done) {
        request.put('/verticalscaling/'+globalAny.vscale_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status": {"last_scale_time": "", "current_resource": {"cpu":"2", "ram":"1000 GiB"}, "desired_resource":{ "cpu":"3", "ram":"2000 GiB" }}})
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the verticalscaling status update by  wrong id', function(done) {
        request.put('/verticalscaling/'+globalAny.account_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status": {"last_scale_time": "", "current_resource": {"cpu":"2", "ram":"1000 GiB"}, "desired_resource":{ "cpu":"3", "ram":"2000 GiB" }}})
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the verticalscaling status update by id type mismatch ', function(done) {
        request.put('/verticalscaling/'+globalAny.vscale_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status": {"last_scale_time": 5, "current_resource": {"cpu":"2", "ram":"1000 GiB"}, "desired_resource":{ "cpu":"3", "ram":"2000 GiB" }}})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns all verticalscaling', function(done) {
        request.get('/verticalscaling')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            expect(res.body.items.length).to.equal(1);
            done(err);
          });
      });

      it('returns all verticalscaling wrong url ', function(done) {
        request.get('/verticalscalings')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns all verticalscaling without header ', function(done) {
        request.get('/verticalscaling')
        .ca(globalAny.rootCA)
          .expect(406)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the verticalscaling  update by id', function(done) {
        request.put('/verticalscaling/'+globalAny.vscale_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
            .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the verticalscaling  update by wrong  id', function(done) {
        request.put('/verticalscaling/'+globalAny.account_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"update_policy":{"mode":"auto"},"object_meta":{"name": "vscaling", "account":globalAny.account_id, "labels":{},"annotations":{}, "owner_references":[{"kind":"Assembly","api_version":"v1","name":"lev1.megam.io","uid":globalAny.assembly_id,
          "block_owner_deletion":true}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}, "status": {"last_scale_time": "", "current_resource":{"cpu":"2","ram":"1000 GiB"},
          "desired_resource":{"cpu":"3","ram":"2000 GiB"}}, "scale_type":"AUTOVS","state":"active", "metadata":{},"spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_resource":{"cpu":"2","ram":"1000 MiB"},"max_resource":{"cpu":"4",  "ram":"4000 MiB"},
          "metrics":[{"metric_type": "Resource","resource":{"name": "memory", "min_target_value":"2", "max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}})
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the assembly by id and check spec data', function(done) {
        this.timeout(4000)
        request.get('/assemblys/'+ globalAny.assembly_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
           expect(res.body.id).to.equal(globalAny.assembly_id);
           expect(res.body.spec.assembly_factory.id).to.equal(globalAny.asm_fac_id);
           expect(res.body.spec.assembly_factory.spec.plan.id).to.equal(globalAny.plan_id);
           expect(res.body.spec.endpoints.id).to.equal(globalAny.endpoints_id);
           expect(res.body.spec.volumes[0].id).to.equal(globalAny.vol_id);
           expect(res.body.type_meta.kind).to.equal(globalAny.assemblys);
           expect(res.body.type_meta.api_version).to.equal(globalAny.version);
            done(err);
          });
      });


  });
  });
