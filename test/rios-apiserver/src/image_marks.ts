//12 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Image Marks  API', function() {
    it('returns the created image marks', function(done) {
      request.post('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},"lookup_policy":false, "generation":0, "image":{"name": "","type_meta":{"kind":"","api_version":""}, "docker_image_reference":"ruby", "docker_image_meta_data_version":"1.0", "docker_image_manifest":"manifest",
          "docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config", "docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},{"name":"c22013c84729","layer_size":194,"media_type": ""}],"docker_image_metadata":
          { "docker_image_id":"2f095dcd37dc","parent":"8c7059377eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66","created":"November 11, 2016 at 03.40 PM","size":2,"type_meta":{"kind":"DockerImage","api_version":"1.0"},"config":{"image":"lizrice/childimage","labels":
          {"org.label.description": "this is experiemental image that i use to test container images and labels","org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage"}}}}})
        .expect(200)
        .end(function(err, res) {
          globalAny.img_marks =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it(' created imagemarks empty name', function(done) {
      request.post('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},"lookup_policy":false, "generation":0, "image":{"name": "","type_meta":{"kind":"","api_version":""}, "docker_image_reference":"ruby", "docker_image_meta_data_version":"1.0", "docker_image_manifest":"manifest",
          "docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config", "docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},{"name":"c22013c84729","layer_size":194,"media_type": ""}],"docker_image_metadata":
          { "docker_image_id":"2f095dcd37dc","parent":"8c7059377eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66","created":"November 11, 2016 at 03.40 PM","size":2,"type_meta":{"kind":"DockerImage","api_version":"1.0"},"config":{"image":"lizrice/childimage","labels":
          {"org.label.description": "this is experiemental image that i use to test container images and labels","org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage"}}}}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created image marks missing name parameter', function(done) {
      request.post('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},"lookup_policy":false, "generation":0, "image":{"name": "","type_meta":{"kind":"","api_version":""}, "docker_image_reference":"ruby", "docker_image_meta_data_version":"1.0", "docker_image_manifest":"manifest",
          "docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config", "docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},{"name":"c22013c84729","layer_size":194,"media_type": ""}],"docker_image_metadata":
          { "docker_image_id":"2f095dcd37dc","parent":"8c7059377eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66","created":"November 11, 2016 at 03.40 PM","size":2,"type_meta":{"kind":"DockerImage","api_version":"1.0"},"config":{"image":"lizrice/childimage","labels":
          {"org.label.description": "this is experiemental image that i use to test container images and labels","org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage"}}}}})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns error without header create imagemarks', function(done) {
      request.get('/imagemarks')
      .ca(globalAny.rootCA)
      .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id,
       "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},"lookup_policy":false, "generation":0, "image":{"name": "","type_meta":{"kind":"","api_version":""}, "docker_image_reference":"ruby", "docker_image_meta_data_version":"1.0", "docker_image_manifest":"manifest",
        "docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config", "docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},{"name":"c22013c84729","layer_size":194,"media_type": ""}],"docker_image_metadata":
        { "docker_image_id":"2f095dcd37dc","parent":"8c7059377eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66","created":"November 11, 2016 at 03.40 PM","size":2,"type_meta":{"kind":"DockerImage","api_version":"1.0"},"config":{"image":"lizrice/childimage","labels":
        {"org.label.description": "this is experiemental image that i use to test container images and labels","org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage"}}}}})
      .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the created imagemarks missing image reference id', function(done) {
      request.post('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":"",
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},"lookup_policy":false, "generation":0, "image":{"name": "","type_meta":{"kind":"","api_version":""}, "docker_image_reference":"ruby", "docker_image_meta_data_version":"1.0", "docker_image_manifest":"manifest",
          "docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config", "docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},{"name":"c22013c84729","layer_size":194,"media_type": ""}],"docker_image_metadata":
          { "docker_image_id":"2f095dcd37dc","parent":"8c7059377eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66","created":"November 11, 2016 at 03.40 PM","size":2,"type_meta":{"kind":"DockerImage","api_version":"1.0"},"config":{"image":"lizrice/childimage","labels":
          {"org.label.description": "this is experiemental image that i use to test container images and labels","org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage"}}}}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all imagemarks', function(done) {
      request.get('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('invalid url for all imagemarks get', function(done) {
      request.get('/imagemark')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the imagemarks by id', function(done) {
      request.get('/imagemarks/' + globalAny.img_marks)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.img_marks);
          done(err);
        });
    });

    it('returns the imagemarks by wrong  id', function(done) {
      request.get('/imagemarks/8907654345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the imagemarks by wrong id type', function(done) {
      request.get('/imagemarks/890765uikj4345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update image marks', function(done) {
      request.put('/imagemarks/'+globalAny.img_marks)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby@371829c","account":"888178251065729024","created_at":"2018-03-07T07:02:22.213896507+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":
        [{"kind":"Build","api_version":"v1","name":"ruby-i","uid":"921422565900042240","block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"",
        "details":{"name":"","group":"","kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"tag":{"name":"","annotations":{},
        "from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"reference":false,"generation":0,"import_policy":{"insecure":false,"scheduled":false},"reference_policy":""},"generation":0,
        "conditions":[],"lookup_policy":false,"image":{"type_meta":{"kind":"","api_version":""},"name":"","docker_image_reference":"ruby","docker_image_metadata":{"docker_image_id":"2f095dcd37dc","parent":"8c705988880000007eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66",
        "created":"November 11, 2016 at 03.40 PM","size":2,"config":{"image":"lizrice/childimage","labels":{"org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage","org.label.description":
        "this is experiemental image that i use to test container images and labels"}},"type_meta":{"kind":"DockerImage","api_version":"1.0"}},"docker_image_meta_data_version":"1.0","docker_image_manifest":"manifest","docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},
        {"name":"c22013c84729","layer_size":194,"media_type":""}],"docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config"}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update image marks wrong id', function(done) {
      request.put('/imagemarks/890987654324567')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby@371829c","account":"888178251065729024","created_at":"2018-03-07T07:02:22.213896507+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":
        [{"kind":"Build","api_version":"v1","name":"ruby-i","uid":"921422565900042240","block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"",
        "details":{"name":"","group":"","kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"tag":{"name":"","annotations":{},
        "from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"reference":false,"generation":0,"import_policy":{"insecure":false,"scheduled":false},"reference_policy":""},"generation":0,
        "conditions":[],"lookup_policy":false,"image":{"type_meta":{"kind":"","api_version":""},"name":"","docker_image_reference":"ruby","docker_image_metadata":{"docker_image_id":"2f095dcd37dc","parent":"8c705988880000007eaf86bc913e915f064c073ff45552e8921ceeb1a3b7cbf9215ecb66",
        "created":"November 11, 2016 at 03.40 PM","size":2,"config":{"image":"lizrice/childimage","labels":{"org.label-schema.license":"Apache2.0","org.label-schema.name":"childimage","org.label.description":
        "this is experiemental image that i use to test container images and labels"}},"type_meta":{"kind":"DockerImage","api_version":"1.0"}},"docker_image_meta_data_version":"1.0","docker_image_manifest":"manifest","docker_image_layers":[{"name":"d74508fb6632","layer_size":1,"media_type":""},
        {"name":"c22013c84729","layer_size":194,"media_type":""}],"docker_image_manifest_media_type":"application/vnd.docker.distribution.manifest.v1+json","docker_image_config":"my-config"}})
      .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns list of the imagemarks by build id', function(done) {
      request.get('/imagemarks/builds/921422565900042240')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('returns list of the imagemarks by  wrong build id', function(done) {
      request.get('/imagemarks/builds/92142250042240')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


  });
