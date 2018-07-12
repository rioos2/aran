import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Plan Factory API', function() {
  describe('plan factory creation API', function() {
    it('returns the created plan factory', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
        "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
        "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
        "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
        "category": "machine", "version": "14.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.plan_id =res.body.id;
          expect(res.body.type_meta.kind).to.equal(globalAny.plan);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          expect(res.body.object_meta.name).to.equal("ubuntu");
          done(err);
        });
    });

    it('returns the BadRequest error for empty category', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
        "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
        "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
        "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
        "category": "", "version": "16.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
      .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });


    it('returns the Malformed error without category field', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
        "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
        "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
        "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
        "version": "16.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the BadRequest error for empty version', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
        "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
        "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
        "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
        "category": "machine", "version": "", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
      .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns the BadRequest error for empty name', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{ "name":"", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
        "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
        "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
        "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
        "category": "machine", "version": "16.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns all plan factory', function(done) {
      request.get('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.kind).to.equal(globalAny.planlist);
          expect(res.body.api_version).to.equal(globalAny.version);
          expect(res.body.items.length).to.equal(2);
          done(err);
        });
    });
    it('returns the plan by id', function(done) {
      request.get('/plans/'+globalAny.plan_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.type_meta.kind).to.equal(globalAny.plan);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          expect(res.body.id).to.equal(globalAny.plan_id);
          done(err);
        });
    });

    it('returns the plan by wrong id', function(done) {
      request.get('/plans/876654688765567')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns  unauthorized error created plan factory', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
      .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":"1024473253563072512","block_owner_deletion":false}]},
      "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
      "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
      "category": "machine", "version": "16.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for show plan', function(done) {
      request.get('/plans/876654688765567')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns unauthorized error for list plan factory', function(done) {
      request.get('/plans')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });
    it('update plan status', function(done) {
      request.put('/plans/'+globalAny.plan_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"ready","message":"","reason":"","conditions":[{"condition_type":"","message":"","reason":"","status":"ready","last_update_time":"","last_transition_time":"","last_probe_time":""}]}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });
    it('update plan status invalid id', function(done) {
      request.put('/plans/9876543213456/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"ready","message":"","reason":"","conditions":[{"condition_type":"","message":"","reason":"","status":"ready","last_update_time":"","last_transition_time":"","last_probe_time":""}]}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });
    it('update plan status missing phase', function(done) {
      request.put('/plans/'+globalAny.plan_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"","message":"","reason":"","conditions":[{"condition_type":"","message":"","reason":"","status":"ready","last_update_time":"","last_transition_time":"","last_probe_time":""}]}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });


  });
  });
