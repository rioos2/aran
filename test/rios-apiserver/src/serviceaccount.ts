import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Service account API', function() {

  it('returns the created serviceaccounts', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/serviceaccounts')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{"name":"assemblyfactory-controller","account":"","created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{"group":"development"},"annotations":{"rioos.io/serviceaccount":"job"},
      "owner_references":[{"kind":"", "api_version":"", "name":"", "uid":"", "block_owner_deletion":true}],"finalizers":[],"cluster_name":""},"metadata":{"origin":globalAny.origin_id},
      "secrets":[{"kind":"", "origin":"", "name":"", "uid":"", "api_version":"","resource_version":"","field_path":""}]})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        globalAny.servacc_name =res.body.object_meta.name;
        globalAny.servacc_id =res.body.id;
        expect(res.body.roles.length).to.equal(1);
        done(err);
      });
  });


  it('returns  serviceaccounts list by origin and name', function(done) {
    request.get('/origins/'+globalAny.origin_id+'/serviceaccounts/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the all serviceaccounts', function(done) {
    request.get('/serviceaccounts')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .expect(200)
      .end(function(err, res) {
        expect(res.body.items.length).to.equal(1);
        done(err);
      });
  });
  it('returns  serviceaccounts show by name', function(done) {
    request.get('/serviceaccounts/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  permissions by service account name', function(done) {
    request.get('/permissions/serviceaccounts/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  permissions by wrong service account name', function(done) {
    request.get('/permissions/serviceaccounts/assembly_servacc_name')
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns the updated serviceaccounts', function(done) {
    request.put('/origins/'+globalAny.origin_id+'/serviceaccounts/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"id":globalAny.servacc_id,"object_meta":{"name":"assemblyfactory-controller","account":"","created_at":"2018-03-07T10:25:06.561355166+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{"group":"development"},"annotations":{"rioos.io/serviceaccount":"job"},"owner_references":[{"kind":"","api_version":"",
      "name":"","uid":"","block_owner_deletion":true}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"",
      "kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"type_meta":{"kind":"ServiceAccount","api_version":"v1"},"metadata":{"origin":"rioos_system"},
      "secrets":[{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}]})
      .expect(200)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns bad request error for name is empty', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/serviceaccounts')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{"name":"","account":"","created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{"group":"development"},"annotations":{"rioos.io/serviceaccount":"job"},
      "owner_references":[{"kind":"", "api_version":"", "name":"", "uid":"", "block_owner_deletion":true}],"finalizers":[],"cluster_name":""},"metadata":{"origin":globalAny.origin_id},
      "secrets":[{"kind":"", "origin":"", "name":"", "uid":"", "api_version":"","resource_version":"","field_path":""}]})
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns malformed for no secrets field', function(done) {
    request.post('/origins/'+globalAny.origin_id+'/serviceaccounts')
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{"name":"assemblyfactory-controller","account":"","created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"labels":{"group":"development"},"annotations":{"rioos.io/serviceaccount":"job"},
      "owner_references":[{"kind":"", "api_version":"", "name":"", "uid":"", "block_owner_deletion":true}],"finalizers":[],"cluster_name":""},"metadata":{"origin":globalAny.origin_id},
      })
      .expect(400)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  record not found error serviceaccounts list by origin and name', function(done) {
    request.get('/origins/'+globalAny.origin_id+'/serviceaccounts/'+globalAny.origin_id)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });

  it('returns  serviceaccounts show by wrong name', function(done) {
    request.get('/serviceaccounts/'+globalAny.servacc_id)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });
  it('returns  serviceaccounts show by wrong url', function(done) {
    request.get('/serviceacc/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
    .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .set('Authorization', globalAny.bobo_bearer)
      .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });
  it('returns the updated serviceaccounts wrong service account id', function(done) {
    request.put('/origins/'+globalAny.origin_id+'/serviceaccounts/'+globalAny.servacc_name)
    .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"id":"932411525895831552","object_meta":{"name":"assemblyfactory-controller","account":"","created_at":"2018-03-07T10:25:06.561355166+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{"group":"development"},"annotations":{"rioos.io/serviceaccount":"job"},"owner_references":[{"kind":"","api_version":"",
      "name":"","uid":"","block_owner_deletion":true}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"","reason":"","details":{"name":"","group":"",
      "kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"type_meta":{"kind":"ServiceAccount","api_version":"v1"},"metadata":{"origin":"rioos_system"},
      "secrets":[{"kind":"","origin":"","name":"","uid":"","api_version":"","resource_version":"","field_path":""}]})
        .expect(404)
      .end(function(err, res) {
        done(err);
      });
  });

  });
