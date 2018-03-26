//12 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Image Reference API', function() {

    it('returns the created image reference', function(done) {
      request.post('/imagereferences')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-image","account":globalAny.account_id,"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-build", "uid":globalAny.bc_id, "block_owner_deletion":false}]}, "spec":{"lookup_policy":false,
        "map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample","tags":{"docker": {"items":[{"created": "2016-01-29T13:40:11Z","docker_image_reference":
        "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation": 1}]}}}})
        .expect(200)
        .end(function(err, res) {
          globalAny.image_ref =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it(' created image reference empty name', function(done) {
      request.post('/imagereferences')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":globalAny.account_id,"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-build", "uid":globalAny.bc_id, "block_owner_deletion":false}]}, "spec":{"lookup_policy":false,
        "map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample","tags":{"docker": {"items":[{"created": "2016-01-29T13:40:11Z","docker_image_reference":
        "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation": 1}]}}}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created image reference missing name parameter', function(done) {
      request.post('/imagereferences')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"account":globalAny.account_id,"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-build", "uid":globalAny.bc_id, "block_owner_deletion":false}]}, "spec":{"lookup_policy":false,
        "map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample","tags":{"docker": {"items":[{"created": "2016-01-29T13:40:11Z","docker_image_reference":
        "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation": 1}]}}}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns error without header create image reference', function(done) {
      request.get('/imagereferences')
      .ca(globalAny.rootCA)
      .send({"object_meta":{"name":"ruby-image","account":globalAny.account_id,"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-build", "uid":globalAny.bc_id, "block_owner_deletion":false}]}, "spec":{"lookup_policy":false,
      "map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample","tags":{"docker": {"items":[{"created": "2016-01-29T13:40:11Z","docker_image_reference":
      "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation": 1}]}}}})
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the created image reference missing build config id', function(done) {
      request.post('/imagereferences')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-image","account":globalAny.account_id,"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-build", "uid":"", "block_owner_deletion":false}]}, "spec":{"lookup_policy":false,
        "map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample","tags":{"docker": {"items":[{"created": "2016-01-29T13:40:11Z","docker_image_reference":
        "172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image": "sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation": 1}]}}}})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all image reference', function(done) {
      request.get('/imagereferences')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('invalid url for all image reference get', function(done) {
      request.get('/imagereference')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the image reference by id', function(done) {
      request.get('/imagereferences/' + globalAny.image_ref)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.image_ref);
          done(err);
        });
    });

    it('returns the image reference by wrong  id', function(done) {
      request.get('/imagereferences/8907654345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the imagereferences by wrong id type', function(done) {
      request.get('/imagereferences/890765uikj4345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update image reference', function(done) {
      request.put('/imagereferences/'+globalAny.image_ref)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-image","account":globalAny.account_id,"created_at":"2018-03-26T12:35:44.526901029+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig","api_version":"v1","name":"ruby-build","uid":globalAny.bc_id,
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"spec":{"lookup_policy":false,"map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample",
        "public_docker_image_repository":"","tags":{"docker":{"items":[{"created":"2016-01-29T13:40:11Z","docker_image_reference":"172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image":"sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation":1}],"conditions":[]}}}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update image reference wrong id', function(done) {
      request.put('/imagereferences/890987654324567')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-image","account":globalAny.account_id,"created_at":"2018-03-26T12:35:44.526901029+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig","api_version":"v1","name":"ruby-build","uid":globalAny.bc_id,
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"spec":{"lookup_policy":false,"map_marks":{"ruby@371829c":"932309487992184832"}},"status":{"docker_image_repository":"172.30.56.218:5000/test/origin-ruby-sample",
        "public_docker_image_repository":"","tags":{"docker":{"items":[{"created":"2016-01-29T13:40:11Z","docker_image_reference":"172.30.56.218:5000/test/origin-ruby-sample@sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","image":"sha256:47463d94eb5c049b2d23b03a9530bf944f8f967a0fe79147dd6b9135bf7dd13d","generation":1}],"conditions":[]}}}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the imagereferences by build config id', function(done) {
      request.get('/imagereferences/build_configs/921422565900042240')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the imagereferences by wrong build config id', function(done) {
      request.get('/imagereferences/build_configs/92142256042240')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
