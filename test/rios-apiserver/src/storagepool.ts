import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);


    describe('Storage pool API', function() {
      it('returns the created storage pool', function(done) {
        request.post('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"connector_id": globalAny.st_id,"parameters":{},"storage_info": {"disks": [{"disk": "/dev/sdb","used_size":"10GB","disk_type": "/dev/sdb1","point": "/home","size": "50GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/suganya","size": "500GB","used_size":"10GB"}]},"status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }, "object_meta":{"name":"chennai", "account":"","labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"", "name":"","uid":"","block_owner_deletion":false}], "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,  "finalizers":[], "cluster_name":"" }})
          .expect(200)
          .end(function(err, res) {
            expect(res.body);
            globalAny.stp_id =res.body.id;
            done(err);
          });
      });
      it('returns the created storage pool empty connector id', function(done) {
        request.post('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"connector_id":"","parameters":{},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/suganya","size": "500GB","used_size":"10GB"}]},"status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }, "object_meta":{"name":"chennai", "account":"","labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"", "name":"","uid":"","block_owner_deletion":false}], "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,  "finalizers":[], "cluster_name":"" }})
          .expect(400)
          .end(function(err, res) {
            expect(res.body);
            done(err);
          });
      });
      it('returns the created storage pool without  connector id', function(done) {
        request.post('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"parameters":{},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/suganya","size": "500GB","used_size":"10GB"}]},"status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }, "object_meta":{"name":"chennai", "account":"","labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"", "name":"","uid":"","block_owner_deletion":false}], "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,  "finalizers":[], "cluster_name":"" }})
          .expect(400)
          .end(function(err, res) {
            expect(res.body);
            done(err);
          });
      });

      it('returns the created storage pool invalid connector id', function(done) {
        request.post('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"connector_id": "8765345674567","parameters":{},"storage_info": {"disks": [{"disk": "/dev/sdb","used_size":"10GB","disk_type": "/dev/sdb1","point": "/home","size": "50GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/suganya","size": "500GB","used_size":"10GB"}]},"status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }, "object_meta":{"name":"chennai", "account":"","labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"", "name":"","uid":"","block_owner_deletion":false}], "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,  "finalizers":[], "cluster_name":"" }})
          .expect(500)
          .end(function(err, res) {
            expect(res.body);
            done(err);
          });
      });


      it('returns the created storage pool empty name', function(done) {
        request.post('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"connector_id": globalAny.st_id,"parameters":{},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/suganya","size": "500GB","used_size":"10GB"}]},"status":{"message":"","reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }, "object_meta":{"name":"","account":"","labels":{},"annotations":{}, "owner_references":[{"kind":"","api_version":"", "name":"","uid":"","block_owner_deletion":false}], "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,  "finalizers":[], "cluster_name":"" }})
          .expect(400)
          .end(function(err, res) {
            expect(res.body);
            done(err);
          });
      });
      it('returns storage pool by id', function(done) {
        request.get('/storagespool/'+globalAny.stp_id)
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
           expect(res.body.id).to.equal(globalAny.stp_id);
            done(err);
          });
      });
      it('returns all storage pool', function(done) {
        request.get('/storagespool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            expect(res.body.items.length).to.equal(1);
            done(err);
          });
      });

      it('returns all storage pool invalid url', function(done) {
        request.get('/storagepool')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns all storage pool without header', function(done) {
        request.get('/storagespool')
        .ca(globalAny.rootCA)
          .expect(406)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storagespool by storage connectors id ', function(done) {
        request.get('/storageconnectors/' + globalAny.st_id+'/storagespool')
        .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the storagespool by storage connectors by wrong id', function(done) {
        request.get('/storageconnectors/87612345678/storagespool')
        .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storagespool by storage connectors id wrong url ', function(done) {
        request.get('/storageconnector/' + globalAny.st_id+'/storagespool')
        .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the storagespool by  wrong storagespool id ', function(done) {
        request.get('/storagespool/876567866543')
        .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storagespool by  wrong storage pool id type ', function(done) {
        request.get('/storagespool/8765678fff66543')
        .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the storage pool status update', function(done) {
        request.put('/storagespool/'+ globalAny.stp_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }})
          .expect(200)
          .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.stp_id);
            done(err);
          });
      });
      it('returns the storage pool status update wrong id', function(done) {
        request.put('/storagespool/89076534567654345/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }})
          .expect(404)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storage pool status update wrong id type', function(done) {
        request.put('/storagespool/89076534fgghj567654345/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"message":"","reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storage pool status update missing phase', function(done) {
        request.put('/storagespool/'+ globalAny.stp_id+'/status')
        .ca(globalAny.rootCA)
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"message":"","reason":"","phase": "","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}] }})
          .expect(400)
          .end(function(err, res) {
            done(err);
          });
      });

    });
