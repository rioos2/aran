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
        .send({"object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"category": "application","version": "5.2.0","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"pending","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.plan_id =res.body.id;
          expect(res.body.type_meta.kind).to.equal(globalAny.plan);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          expect(res.body.object_meta.name).to.equal("rails");
          done(err);
        });
    });

    it('returns the BadRequest error for empty category', function(done) {
      request.post('/plans')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"category": "","version": "5.2.0","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
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
        .send({"object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"version": "5.2.0","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
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
        .send({"object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"category": "application","version": "","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
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
        .send({"object_meta":{"name":"","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"category": "application","version": "5.2.0","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
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
        .send({"object_meta":{"name":"rails","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},"category": "application","version": "5.2.0","characteristics" :{"image_pullpolicy": "always","git":"source url"},"icon" : "rails.png","description": "Rails is a framework for building websites. As such, Rails establishes conventions for easier collaboration and maintenance","ports": [{"container_port": 80,"host_ip":"192.168.1.10","host_port": 8001,"protocol":"TCP/UDP"}],"envs":{"RUBY_HOME":{"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true",  "value":"/home/rails/app",  "editable":"true"}},"lifecycle": {"postStart":{"exec":{"command": ["/bin/sh","-c","echo Hello from the postStart handler > /usr/share/message"]} },"preStop": {"exec": {"command": ["/usr/sbin/nginx","-s","quit"]}}},"status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]}})
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

  });
  });
