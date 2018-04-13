import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Settings Map  API', function() {
    it('returns the created settings map', function(done) {
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
        .send({"metadata":{"origin":"rioos_system"},"data":{},"object_meta":{"name": "cluster_info", "account":"", "labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          globalAny.set_map_name =res.body.object_meta.name;
          expect(res.body);
          done(err);
        });
    });

    it('returns the created settings map missing origin', function(done) {
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
        .send({"metadata":{},"data":{},"object_meta":{"name": "cluster_info", "account":"", "labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created settings map missing name', function(done) {
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
        .send({"metadata":{"origin":"rioos_system"},"data":{},"object_meta":{"name": "", "account":"", "labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns settings map by origin and map name', function(done) {
      request.get('/origins/rioos_system/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by origin and invalid map name', function(done) {
      request.get('/origins/rioos_system/settingsmap/clus')
      .ca(globalAny.rootCA)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by invalid origin and map name', function(done) {
      request.get('/origins/rioos/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by origin and map name invalid url', function(done) {
      request.get('/origin/rioos_system/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
