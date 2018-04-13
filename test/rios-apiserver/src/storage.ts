import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Storage API', function() {
  describe('Storage Connector API', function() {
    it('returns the created storage connectors', function(done) {
      request.post('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"host_ip": "172.168.1.1","node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" },"storage_type":"iscsi","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.st_id =res.body.id;
          done(err);
        });
    });
    it('returns the created storage connectors missing host_ip', function(done) {
      request.post('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"host_ip": "","node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" },"storage_type":"iscsi","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created storage connectors without host ip', function(done) {
      request.post('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"storage_type":"iscsi","node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" },"status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB" ,"used_size":"10GB"}, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns the created storage connectors empty storage type', function(done) {
      request.post('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"host_ip": "172.168.1.1","storage_type":"","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns the created storage connectors without storage type', function(done) {
      request.post('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"host_ip": "172.168.1.1","node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" },"status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB","used_size":"10GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/sugan",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });


    it('returns all storage connectors', function(done) {
      request.get('/storageconnectors')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });

    it('returns all storage connectors invalid url ', function(done) {
      request.get('/storageconnector')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns all storage connectors without header ', function(done) {
      request.get('/storageconnectors')
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the storage connectors by id', function(done) {
      request.get('/storageconnectors/' + globalAny.st_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.st_id);
          done(err);
        });
    });

    it('returns the storage connectors by wrong id', function(done) {
      request.get('/storageconnectors/876541122345556')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the storage connectors by wrong id type', function(done) {
      request.get('/storageconnectors/87654112hhyuj2345556')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update storage connectors status', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"",  "reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update storage connectors status missing phase', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"",  "reason":"","phase": "","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update storage connectors status by wrong id', function(done) {
      request.put('/storageconnectors/8765434567809/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"message":"",  "reason":"","phase": "ready","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]}})
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update storage connectors', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"id":globalAny.st_id,"node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" }, "host_ip": "172.168.1.1","storage_type":"iscsi","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","used_size":"10GB","disk_type": "/dev/sdb1","point": "/home","size": "50GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji",  "size": "500GB","used_size":"10GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('update storage connectors wrong connector id', function(done) {
      request.put('/storageconnectors/876890655433')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"id":"876890655433","node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" }, "host_ip": "172.168.1.1","storage_type":"iscsi","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","used_size":"10GB","disk_type": "/dev/sdb1","point": "/home","size": "50GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2","used_size":"10GB",  "point": "/home/ranji",  "size": "500GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(404)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('update storage connectors missing host ip', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id)
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"id":globalAny.st_id,"node_info":{"machine_id": "589f17c8cc084c078c5d364241433afc", "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version": "4.4.0-93-generic","os_image": "Ubuntu 16.04.3 LTS","architecture": "amd64" },"storage_type":"iscsi","status":{"message":"",  "reason":"","phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]},"storage_info": {"disks": [{"disk": "/dev/sdb","used_size":"10GB","disk_type": "/dev/sdb1","point": "/home","size": "50GB" }, { "disk": "/dev/sdb1", "disk_type": "/dev/sdb2",  "point": "/home/ranji","used_size":"10GB",  "size": "500GB"}]},"parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},"object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
  });

  });
