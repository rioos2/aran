//15 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Build  API', function() {

    it('returns the created build', function(done) {
      request.post('/builds')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-build","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-sample-build1", "uid":globalAny.bc_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"}, "status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "","completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}},
         "spec": {"triggerd_by_causes": [{"message": "","webhook_causes": [{"hook_type":"git","revision": {"git": {"commit": "78rftghjvbnm","message": "readme update"}},"secret": "876543212345678909"}],"image_build_cause": {"image_id": ""}}],"source":
         {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master"},"binary" : {"as_file": ""},"docker_file":"","images": [{ "from": {"kind":"","origin":"","name":"","uid":"","api_version":"",
         "resource_version":"","field_path":""}, "pull_secret": "","paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],"source_secret": globalAny.secrets_id },"strategy": {"build_type":"Docker", "docker_strategy":{"force_pull": true,"from":{"kind": "ImageMark",
         "name": "debian:latest","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""},"docker_filepath": "http://somehost.com/scripts_directory","env":[{"name":"HTTP_PROXY","value": "http://myproxy.net:5187/"}]}},"output": {"to":
         {"kind": "ImageMark","name": "node-build-1:136c86c0","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""}},"post_commit": { "script": "bundle exec rake test" },"node_selector": {} }})
         .expect(200)
        .end(function(err, res) {
          globalAny.build_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it(' created build empty name', function(done) {
      request.post('/builds')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-sample-build1", "uid":globalAny.bc_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"}, "status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "","completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}},
         "spec": {"triggerd_by_causes": [{"message": "","webhook_causes": [{"hook_type":"git","revision": {"git": {"commit": "78rftghjvbnm","message": "readme update"}},"secret": "876543212345678909"}],"image_build_cause": {"image_id": ""}}],"source":
         {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master"},"binary" : {"as_file": ""},"docker_file":"","images": [{ "from": {"kind":"","origin":"","name":"","uid":"","api_version":"",
         "resource_version":"","field_path":""}, "pull_secret": "","paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],"source_secret": globalAny.secrets_id },"strategy": {"build_type":"Docker", "docker_strategy":{"force_pull": true,"from":{"kind": "ImageMark",
         "name": "debian:latest","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""},"docker_filepath": "http://somehost.com/scripts_directory","env":[{"name":"HTTP_PROXY","value": "http://myproxy.net:5187/"}]}},"output": {"to":
         {"kind": "ImageMark","name": "node-build-1:136c86c0","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""}},"post_commit": { "script": "bundle exec rake test" },"node_selector": {} }})
      .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created build missing name parameter', function(done) {
      request.post('/builds')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-sample-build1", "uid":globalAny.bc_id,
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"}, "status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "","completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}},
         "spec": {"triggerd_by_causes": [{"message": "","webhook_causes": [{"hook_type":"git","revision": {"git": {"commit": "78rftghjvbnm","message": "readme update"}},"secret": "876543212345678909"}],"image_build_cause": {"image_id": ""}}],"source":
         {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master"},"binary" : {"as_file": ""},"docker_file":"","images": [{ "from": {"kind":"","origin":"","name":"","uid":"","api_version":"",
         "resource_version":"","field_path":""}, "pull_secret": "","paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],"source_secret": globalAny.secrets_id },"strategy": {"build_type":"Docker", "docker_strategy":{"force_pull": true,"from":{"kind": "ImageMark",
         "name": "debian:latest","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""},"docker_filepath": "http://somehost.com/scripts_directory","env":[{"name":"HTTP_PROXY","value": "http://myproxy.net:5187/"}]}},"output": {"to":
         {"kind": "ImageMark","name": "node-build-1:136c86c0","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""}},"post_commit": { "script": "bundle exec rake test" },"node_selector": {} }})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns error without header create build', function(done) {
      request.get('/builds')
      .ca(globalAny.rootCA)
      .send({"object_meta":{"name":"ruby-build","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-sample-build1", "uid":globalAny.bc_id,
       "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"}, "status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "","completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}},
       "spec": {"triggerd_by_causes": [{"message": "","webhook_causes": [{"hook_type":"git","revision": {"git": {"commit": "78rftghjvbnm","message": "readme update"}},"secret": "876543212345678909"}],"image_build_cause": {"image_id": ""}}],"source":
       {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master"},"binary" : {"as_file": ""},"docker_file":"","images": [{ "from": {"kind":"","origin":"","name":"","uid":"","api_version":"",
       "resource_version":"","field_path":""}, "pull_secret": "","paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],"source_secret": globalAny.secrets_id },"strategy": {"build_type":"Docker", "docker_strategy":{"force_pull": true,"from":{"kind": "ImageMark",
       "name": "debian:latest","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""},"docker_filepath": "http://somehost.com/scripts_directory","env":[{"name":"HTTP_PROXY","value": "http://myproxy.net:5187/"}]}},"output": {"to":
       {"kind": "ImageMark","name": "node-build-1:136c86c0","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""}},"post_commit": { "script": "bundle exec rake test" },"node_selector": {} }})
        .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the created build missing build config id', function(done) {
      request.post('/builds')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-build","account":globalAny.account_id,"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig", "api_version":"v1", "name":"ruby-sample-build1", "uid":"",
         "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"}, "status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "","completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}},
         "spec": {"triggerd_by_causes": [{"message": "","webhook_causes": [{"hook_type":"git","revision": {"git": {"commit": "78rftghjvbnm","message": "readme update"}},"secret": "876543212345678909"}],"image_build_cause": {"image_id": ""}}],"source":
         {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master"},"binary" : {"as_file": ""},"docker_file":"","images": [{ "from": {"kind":"","origin":"","name":"","uid":"","api_version":"",
         "resource_version":"","field_path":""}, "pull_secret": "","paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],"source_secret": globalAny.secrets_id },"strategy": {"build_type":"Docker", "docker_strategy":{"force_pull": true,"from":{"kind": "ImageMark",
         "name": "debian:latest","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""},"docker_filepath": "http://somehost.com/scripts_directory","env":[{"name":"HTTP_PROXY","value": "http://myproxy.net:5187/"}]}},"output": {"to":
         {"kind": "ImageMark","name": "node-build-1:136c86c0","uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""}},"post_commit": { "script": "bundle exec rake test" },"node_selector": {} }})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all build', function(done) {
      request.get('/builds')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('invalid url for all build get', function(done) {
      request.get('/build')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build by build config id', function(done) {
      request.get('/builds/buildconfig/'+globalAny.bc_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the build by wrong buildconfig id', function(done) {
      request.get('/builds/buildconfig/'+globalAny.job_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build by buildconfig wrong url', function(done) {
      request.get('/buildconfig/'+globalAny.bc_id+'/builds')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build by id', function(done) {
      request.get('/builds/' + globalAny.build_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.build_id);
          done(err);
        });
    });

    it('returns the build by wrong  id', function(done) {
      request.get('/builds/8907654345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the builds by wrong id type', function(done) {
      request.get('/builds/890765uikj4345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update build', function(done) {
      request.put('/builds/'+globalAny.build_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-build","account":"931719409490206720","created_at":"2018-03-08T12:22:12.702400390+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig","api_version":"v1","name":"ruby-sample-build1","uid":"921422565900042240",
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"spec":{"triggerd_by_causes":[{"message":"","webhook_causes":[{"hook_type":"git","revision":
        {"git":{"commit":"78rftghjvbnm","message":"readme update"}},"secret":"876543212345678909"}],"image_build_cause":{"image_id":""}}],"source":{"binary":{"as_file":""},"docker_file":"","git":{"uri":"https://github.com/rioadvancement/news-composer-network","reference":"master"},"source_secret":"",
        "images":[{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","paths":[{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]}]},"strategy":
        {"build_type":"Docker","source_strategy":{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[],"scripts":"","incremental":"",
        "force_pull":false,"runtime_image":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}},"docker_strategy":{"from":{"kind":"ImageMark","origin":"","name":"debian:latest","uid":"",
        "api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[{"name":"HTTP_PROXY","value":"http://myproxy.net:5187/"}],"force_pull":true,"docker_filepath":"http://somehost.com/scripts_directory","image_optimization_policy":""}},"output":{"to":{"kind":"ImageMark",
        "origin":"", "name":"node-build-1:136c86c0","uid":"","api_version":"","resource_version":"","field_path":""}},"post_commit":{"script":"bundle exec rake test"},"node_selector":{}},"status":{"phase":"New","cancelled":false,"reason":"","message":"",
        "start_timestamp":"","completion_timestamp":"","duration":"","output_docker_image_reference":"","output":{"to":""}}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update build status', function(done) {
      request.put('/builds/'+globalAny.build_id+'/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "", "completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update build status wrong id', function(done) {
      request.put('/builds/876543456543245678/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase": "New","cancelled": false,"reason":"","message":"","start_timestamp": "", "completion_timestamp":"","duration":"","output_docker_image_reference": "","output":{"to":""}}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the update build wrong id', function(done) {
      request.put('/builds/890987654324567')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-build","account":"931719409490206720","created_at":"2018-03-08T12:22:12.702400390+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"BuildConfig","api_version":"v1","name":"ruby-sample-build1","uid":"921422565900042240",
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"spec":{"triggerd_by_causes":[{"message":"","webhook_causes":[{"hook_type":"git","revision":
        {"git":{"commit":"78rftghjvbnm","message":"readme update"}},"secret":"876543212345678909"}],"image_build_cause":{"image_id":""}}],"source":{"binary":{"as_file":""},"docker_file":"","git":{"uri":"https://github.com/rioadvancement/news-composer-network","reference":"master"},"source_secret":"",
        "images":[{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","paths":[{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]}]},"strategy":
        {"build_type":"Docker","source_strategy":{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[],"scripts":"","incremental":"",
        "force_pull":false,"runtime_image":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}},"docker_strategy":{"from":{"kind":"ImageMark","origin":"","name":"debian:latest","uid":"",
        "api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[{"name":"HTTP_PROXY","value":"http://myproxy.net:5187/"}],"force_pull":true,"docker_filepath":"http://somehost.com/scripts_directory","image_optimization_policy":""}},"output":{"to":{"kind":"ImageMark",
        "origin":"", "name":"node-build-1:136c86c0","uid":"","api_version":"","resource_version":"","field_path":""}},"post_commit":{"script":"bundle exec rake test"},"node_selector":{}},"status":{"phase":"New","cancelled":false,"reason":"","message":"",
        "start_timestamp":"","completion_timestamp":"","duration":"","output_docker_image_reference":"","output":{"to":""}}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
