//15 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Build Config  API', function() {

    it('returns the created build config', function(done) {
      request.post('/buildconfigs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-sample-build", "account":globalAny.account_id, "created_at":"","deleted_at":"", "deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory", "api_version":"v1", "name":"levi.megam.io",
        "uid":globalAny.asm_fac_id, "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},  "meta_data": { "name": "ruby-sample-build"},"status":{"phase":"pending"},"spec": { "run_policy": "Serial", "build_trigger_policys": [ {"trigger_type": "gittrigger", "webhooks":  [
          {"hook_type": "GitHub",  "secret": "secret101"}],"image_change": {"last_triggered_image_id": "1001" }} ],"source": {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master" }, "binary" : {"as_file": ""},"docker_file":"",
          "images": [ { "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}, "pull_secret": "", "paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],
          "source_secret": globalAny.secrets_id}, "strategy":{ "build_type":"Source","source_strategy": {"env":[{"name":"DISABLE_ASSET_COMPILATION","value": "true"}],"from":{"kind": "ImageMark","name": "builder-image:latest","uid":"","api_version":"","resource_version":"",
          "field_path":"","origin":""}, "scripts": "http://somehost.com/scripts_directory" } },"output": { "to": { "kind": "ImageMark","name": "node-build-1:136c86c0" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
          "post_commit": {"script": "bundle exec rake test"},"node_selector": {"key":"value"},"last_version": 10,"successful_builds_history_limit": 10,"failed_builds_history_limit": 1}})
        .expect(200)
        .end(function(err, res) {
          globalAny.bc_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it(' created build config empty name', function(done) {
      request.post('/buildconfigs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"", "account":globalAny.account_id, "created_at":"","deleted_at":"", "deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory", "api_version":"v1", "name":"levi.megam.io",
        "uid":globalAny.asm_fac_id, "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},  "meta_data": { "name": "ruby-sample-build"},"status":{"phase":"pending"},"spec": { "run_policy": "Serial", "build_trigger_policys": [ {"trigger_type": "gittrigger", "webhooks":  [
          {"hook_type": "GitHub",  "secret": "secret101"}],"image_change": {"last_triggered_image_id": "1001" }} ],"source": {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master" }, "binary" : {"as_file": ""},"docker_file":"",
          "images": [ { "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}, "pull_secret": "", "paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],
          "source_secret": globalAny.secrets_id}, "strategy":{ "build_type":"Source","source_strategy": {"env":[{"name":"DISABLE_ASSET_COMPILATION","value": "true"}],"from":{"kind": "ImageMark","name": "builder-image:latest","uid":"","api_version":"","resource_version":"",
          "field_path":"","origin":""}, "scripts": "http://somehost.com/scripts_directory" } },"output": { "to": { "kind": "ImageMark","name": "node-build-1:136c86c0" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
          "post_commit": {"script": "bundle exec rake test"},"node_selector": {"key":"value"},"last_version": 10,"successful_builds_history_limit": 10,"failed_builds_history_limit": 1}})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created build config missing name parameter', function(done) {
      request.post('/buildconfigs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"account":globalAny.account_id, "created_at":"","deleted_at":"", "deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory", "api_version":"v1", "name":"levi.megam.io",
        "uid":globalAny.asm_fac_id, "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},  "meta_data": { "name": "ruby-sample-build"},"status":{"phase":"pending"},"spec": { "run_policy": "Serial", "build_trigger_policys": [ {"trigger_type": "gittrigger", "webhooks":  [
          {"hook_type": "GitHub",  "secret": "secret101"}],"image_change": {"last_triggered_image_id": "1001" }} ],"source": {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master" }, "binary" : {"as_file": ""},"docker_file":"",
          "images": [ { "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}, "pull_secret": "", "paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],
          "source_secret": globalAny.secrets_id}, "strategy":{ "build_type":"Source","source_strategy": {"env":[{"name":"DISABLE_ASSET_COMPILATION","value": "true"}],"from":{"kind": "ImageMark","name": "builder-image:latest","uid":"","api_version":"","resource_version":"",
          "field_path":"","origin":""}, "scripts": "http://somehost.com/scripts_directory" } },"output": { "to": { "kind": "ImageMark","name": "node-build-1:136c86c0" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
          "post_commit": {"script": "bundle exec rake test"},"node_selector": {"key":"value"},"last_version": 10,"successful_builds_history_limit": 10,"failed_builds_history_limit": 1}})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns error without header create build config', function(done) {
      request.get('/buildconfigs')
      .ca(globalAny.rootCA)
      .send({"object_meta":{"name":"ruby-sample-build", "account":globalAny.account_id, "created_at":"","deleted_at":"", "deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory", "api_version":"v1", "name":"levi.megam.io",
      "uid":globalAny.asm_fac_id, "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},  "meta_data": { "name": "ruby-sample-build"},"status":{"phase":"pending"},"spec": { "run_policy": "Serial", "build_trigger_policys": [ {"trigger_type": "gittrigger", "webhooks":  [
        {"hook_type": "GitHub",  "secret": "secret101"}],"image_change": {"last_triggered_image_id": "1001" }} ],"source": {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master" }, "binary" : {"as_file": ""},"docker_file":"",
        "images": [ { "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}, "pull_secret": "", "paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],
        "source_secret": globalAny.secrets_id}, "strategy":{ "build_type":"Source","source_strategy": {"env":[{"name":"DISABLE_ASSET_COMPILATION","value": "true"}],"from":{"kind": "ImageMark","name": "builder-image:latest","uid":"","api_version":"","resource_version":"",
        "field_path":"","origin":""}, "scripts": "http://somehost.com/scripts_directory" } },"output": { "to": { "kind": "ImageMark","name": "node-build-1:136c86c0" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
        "post_commit": {"script": "bundle exec rake test"},"node_selector": {"key":"value"},"last_version": 10,"successful_builds_history_limit": 10,"failed_builds_history_limit": 1}})
    .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the created build config missing assembly factory id', function(done) {
      request.post('/buildconfigs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-sample-build", "account":globalAny.account_id, "created_at":"","deleted_at":"", "deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory", "api_version":"v1", "name":"levi.megam.io",
        "uid":"", "block_owner_deletion":false}],"finalizers":[],"cluster_name":"chennai"},  "meta_data": { "name": "ruby-sample-build"},"status":{"phase":"pending"},"spec": { "run_policy": "Serial", "build_trigger_policys": [ {"trigger_type": "gittrigger", "webhooks":  [
          {"hook_type": "GitHub",  "secret": "secret101"}],"image_change": {"last_triggered_image_id": "1001" }} ],"source": {"git": {"uri": "https://github.com/rioadvancement/news-composer-network","reference" : "master" }, "binary" : {"as_file": ""},"docker_file":"",
          "images": [ { "from": {"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}, "pull_secret": "", "paths": [{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]} ],
          "source_secret": globalAny.secrets_id}, "strategy":{ "build_type":"Source","source_strategy": {"env":[{"name":"DISABLE_ASSET_COMPILATION","value": "true"}],"from":{"kind": "ImageMark","name": "builder-image:latest","uid":"","api_version":"","resource_version":"",
          "field_path":"","origin":""}, "scripts": "http://somehost.com/scripts_directory" } },"output": { "to": { "kind": "ImageMark","name": "node-build-1:136c86c0" ,"uid":"","api_version":"", "resource_version":"", "field_path":"","origin":""} },
          "post_commit": {"script": "bundle exec rake test"},"node_selector": {"key":"value"},"last_version": 10,"successful_builds_history_limit": 10,"failed_builds_history_limit": 1}})
          .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all build config', function(done) {
      request.get('/buildconfigs')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });
    it('returns the update build config', function(done) {
      request.put('/buildconfigs/'+globalAny.bc_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-sample-build","account":globalAny.account_id,"created_at":"2018-03-08T12:20:50.144227068+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi11.megam.io","uid":globalAny.asm_fac_id,
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"meta_data":{"name":"ruby-sample-build"},"status":{"phase":"pending","message":"","reason":"","conditions":[]},"spec":{"run_policy":"Serial","build_trigger_policys":[{"trigger_type":"gittrigger","webhooks":[{"hook_type":"GitHub",
        "secret":"secret101"}],"image_change":{"last_triggered_image_id":"1001"}}],"source":{"binary":{"as_file":""},"docker_file":"","git":{"uri":"https://github.com/rioadvancement/news-composer-network","reference":"master"},"source_secret":globalAny.secrets_id,"images":[{"from":{"kind":"",
        "origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","paths":[{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]}]},"strategy":{"build_type":"Source","source_strategy":{"from":{
          "kind":"ImageMark","origin":"","name":"builder-image:latest","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[{"name":"DISABLE_ASSET_COMPILATION","value":"true"}],"scripts":"http://somehost.com/scripts_directory","incremental":"",
          "force_pull":false,"runtime_image":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}},"docker_strategy":{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[],
          "force_pull":false,"docker_filepath":"","image_optimization_policy":""}},"output":{"to":{"kind":"ImageMark","origin":"","name":"node-build-1:136c86c0","uid":"","api_version":"","resource_version":"","field_path":""}},"post_commit":{"script":"bundle exec rake test"},
          "node_selector":{"key":"value"},"last_version":10,"successful_builds_history_limit":10,"failed_builds_history_limit":1}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.bc_id);
          done(err);
        });
    });

    it('returns the update build config wrong build config id', function(done) {
      request.put('/buildconfigs/876543212345678909')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ruby-sample-build","account":globalAny.account_id,"created_at":"2018-03-08T12:20:50.144227068+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"levi11.megam.io","uid":globalAny.asm_fac_id,
        "block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],
        "uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":"chennai"},"meta_data":{"name":"ruby-sample-build"},"status":{"phase":"pending","message":"","reason":"","conditions":[]},"spec":{"run_policy":"Serial","build_trigger_policys":[{"trigger_type":"gittrigger","webhooks":[{"hook_type":"GitHub",
        "secret":"secret101"}],"image_change":{"last_triggered_image_id":"1001"}}],"source":{"binary":{"as_file":""},"docker_file":"","git":{"uri":"https://github.com/rioadvancement/news-composer-network","reference":"master"},"source_secret":globalAny.secrets_id,"images":[{"from":{"kind":"",
        "origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","paths":[{"source_path":"https:///avaf/vad","destination_dir":"/var/lib/"}]}]},"strategy":{"build_type":"Source","source_strategy":{"from":{
          "kind":"ImageMark","origin":"","name":"builder-image:latest","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[{"name":"DISABLE_ASSET_COMPILATION","value":"true"}],"scripts":"http://somehost.com/scripts_directory","incremental":"",
          "force_pull":false,"runtime_image":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}},"docker_strategy":{"from":{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""},"pull_secret":"","env":[],
          "force_pull":false,"docker_filepath":"","image_optimization_policy":""}},"output":{"to":{"kind":"ImageMark","origin":"","name":"node-build-1:136c86c0","uid":"","api_version":"","resource_version":"","field_path":""}},"post_commit":{"script":"bundle exec rake test"},
          "node_selector":{"key":"value"},"last_version":10,"successful_builds_history_limit":10,"failed_builds_history_limit":1}})
          .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build config update by id', function(done) {
      request.put('/buildconfigs/'+globalAny.bc_id+'/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"build config created successfully","reason":"","phase": "ready","conditions": [{"message":"", "reason":"","status": "True",
        "last_transition_time": "", "last_probe_time": "","condition_type":"","last_update_time": ""}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
         expect(res.body.id).to.equal(globalAny.bc_id);
          done(err);
        });
    });

    it('returns the build config update by invalid id', function(done) {
      request.put('/buildconfigs/987654345678909090/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"build config created successfully","reason":"","phase": "ready","conditions": [{"message":"", "reason":"","status": "True",
        "last_transition_time": "", "last_probe_time": "","condition_type":"","last_update_time": ""}]}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build config update by invalid id type', function(done) {
      request.put('/buildconfigs/9uyn5678909090/status')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"build config created successfully","reason":"","phase": "ready","conditions": [{"message":"", "reason":"","status": "True",
        "last_transition_time": "", "last_probe_time": "","condition_type":"","last_update_time": ""}]}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });


    it('invalid url for all build config get', function(done) {
      request.get('/buildconf')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build config by assembly factory id', function(done) {
      request.get('/buildconfigs/assemblyfactorys/'+globalAny.asm_fac_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('returns the build config by wrong assembly factory id', function(done) {
      request.get('/buildconfigs/assemblyfactorys/'+globalAny.job_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build config by assembly factory wrong url', function(done) {
      request.get('/assemblyfactorys/'+globalAny.asm_fac_id+'/buildconfigs')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the build config by id', function(done) {
      request.get('/buildconfigs/' + globalAny.bc_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.bc_id);
          done(err);
        });
    });

    it('returns the build config by wrong  id', function(done) {
      request.get('/buildconfigs/8907654345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the buildconfigs by wrong id type', function(done) {
      request.get('/buildconfigs/890765uikj4345677')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

  });
