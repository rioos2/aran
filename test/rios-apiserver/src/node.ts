//12 node test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);


  describe('Node API', function() {
    it('returns the created node', function(done) {
      request.post('/nodes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"node_ip": "192.168.1.10","status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}},"spec":{"assembly_cidr": "2345","external_id": "87654", "provider_id": "7654","unschedulable": false,"taints": [{"key": "key","value": "value","effect": "NoSchedule","time_added": ""  }]},"object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.node_id =res.body.id;
          done(err);
        });
    });

    it('without header to created node', function(done) {
      request.post('/nodes')
      .ca(globalAny.rootCA)
        .send({"node_ip": "192.168.1.10","status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}},"spec":{"assembly_cidr": "2345","external_id": "87654", "provider_id": "7654","unschedulable": false,"taints": [{"key": "key","value": "value","effect": "NoSchedule","time_added": ""  }]},"object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(406)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });


    it('returns the created node empty node ip', function(done) {
      request.post('/nodes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"node_ip": "","status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}},"spec":{"assembly_cidr": "2345","external_id": "87654", "provider_id": "7654","unschedulable": false,"taints": [{"key": "key","value": "value","effect": "NoSchedule","time_added": ""  }]},"object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created node empty name', function(done) {
      request.post('/nodes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"node_ip": "192.168.1.10","status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}},"spec":{"assembly_cidr": "2345","external_id": "87654", "provider_id": "7654","unschedulable": false,"taints": [{"key": "key","value": "value","effect": "NoSchedule","time_added": ""  }]},"object_meta":{"name":"","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created node without node ip', function(done) {
      request.post('/nodes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}},"spec":{"assembly_cidr": "2345","external_id": "87654", "provider_id": "7654","unschedulable": false,"taints": [{"key": "key","value": "value","effect": "NoSchedule","time_added": ""  }]},"object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns all nodes', function(done) {
      request.get('/nodes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.items.length).to.equal(1);
          done(err);
        });
    });
    it('invalid url for get all nodes', function(done) {
      request.get('/node')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the node by id', function(done) {
      request.get('/nodes/'+globalAny.node_id)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.node_id);
          done(err);
        });
    });

    it('returns the wrong node by id', function(done) {
      request.get('/nodes/87655434578765')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update node status', function(done) {
      request.put('/nodes/'+globalAny.node_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}}})
          .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update node status missing machine id', function(done) {
      request.put('/nodes/'+globalAny.node_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}}})
          .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update node status wrong node id', function(done) {
      request.put('/nodes/89098765423456543/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}}})
          .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update node status wrong node id type', function(done) {
      request.put('/nodes/908ujhy6789/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,"allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},"phase": "pending","conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],"addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"suganya"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}}})
          .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });


  });
