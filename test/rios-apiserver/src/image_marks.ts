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
        .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id, "block_owner_deletion":false}]},"lookup_policy":false, "generation":0, "image":{"name":
        "ruby@123", "size":156800, "virtual_size": 168400, "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest","docker_image_layers":[{"layer_type":"","layers":[]}]}})
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
        .send({"object_meta":{"name":"","account":globalAny.account_id,"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id, "block_owner_deletion":false}]},"lookup_policy":false, "generation":0, "image":{"name":
        "ruby@123", "size":156800, "virtual_size": 168400, "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest","docker_image_layers":[{"layer_type":"","layers":[]}]}})
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
        .send({"object_meta":{"account":globalAny.account_id,"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":globalAny.build_id, "block_owner_deletion":false}]},"lookup_policy":false, "generation":0, "image":{"name":
        "ruby@123", "size":156800, "virtual_size": 168400, "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest","docker_image_layers":[{"layer_type":"","layers":[]}]}})
            .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns error without header create imagemarks', function(done) {
      request.get('/imagemarks')
      .ca(globalAny.rootCA)
      .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid": globalAny.build_id, "block_owner_deletion": false}]},"lookup_policy":false, "generation":0, "image":{"name":
      "ruby@123", "size":156800, "virtual_size": 168400, "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest","docker_image_layers":[{"layer_type":"","layers":[]}]}})
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the created imagemarks missing build id', function(done) {
      request.post('/imagemarks')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby@371829c","account":globalAny.account_id,"owner_references":[{"kind":"Build", "api_version":"v1", "name":"ruby-image", "uid":"", "block_owner_deletion":false}]},"lookup_policy":false, "generation":0, "image":{"name":
        "ruby@123", "size":156800, "virtual_size": 168400, "docker_image_reference":"registry.rioos.xyz/test.megam.io/ruby:latest","docker_image_layers":[{"layer_type":"","layers":[]}]}})
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
        .send({"object_meta":{"name":"ruby@371829c","account":"946050327142998016","created_at":"2018-03-26T13:25:48.913321833+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build","api_version":"v1","name":"ruby-i","uid":"921422565900042240",
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"tag":{"name":"","annotations":{},"from":{"kind":"","origin":"","name":"","uid":"","api_version":"",
        "resource_version":"","field_path":""},"reference":false,"generation":0,"import_policy":{"insecure":false,"scheduled":false},"reference_policy":""},"generation":0,"conditions":[],"lookup_policy":false,"image":{"type_meta":{"kind":"","api_version":""},
        "name":"ruby@123","size":0,"virtual_size":0,"docker_image_reference":"ruby","docker_image_layers":[{"layer_type":"","layers":[]},{"layer_type":"","layers":[]}]}})
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
        .send({"object_meta":{"name":"ruby@371829c","account":"946050327142998016","created_at":"2018-03-26T13:25:48.913321833+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Build","api_version":"v1","name":"ruby-i","uid":"921422565900042240",
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"tag":{"name":"","annotations":{},"from":{"kind":"","origin":"","name":"","uid":"","api_version":"",
        "resource_version":"","field_path":""},"reference":false,"generation":0,"import_policy":{"insecure":false,"scheduled":false},"reference_policy":""},"generation":0,"conditions":[],"lookup_policy":false,"image":{"type_meta":{"kind":"","api_version":""},
        "name":"ruby@123","size":0,"virtual_size":0,"docker_image_reference":"ruby","docker_image_layers":[{"layer_type":"","layers":[]},{"layer_type":"","layers":[]}]}})
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
