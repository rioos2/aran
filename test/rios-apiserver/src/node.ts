import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;


  describe('Node API', function() {
    it('returns the created node', function(done) {
      request.post('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"node_ip":"192.168.2.47","object_meta":{"name":"822240955804426240","origin":"","uid":"","created_at":"","cluster_name":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}]},"type_meta":{"kind":"Node","api_version":"v1"},"spec":{"assembly_cidr":"","external_id":"rajesh","provider_id":"","unschedulable":false,"taints":[{"key":"","value":"","effect":"","time_added":"2017-09-21T06:35:16Z"}]},"status":{"capacity":{"cpu":"4","memory":"16331164 MiB","pods":"110","storage":"1633 MiB"},"allocatable":{"cpu":"4","memory":"16228764 KiB","pods":"110","storage":"161 MiB"},"phase":"pending","conditions":[{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk"},{"message":"nodelet has sufficient memory available","reason":"NodeletHasSufficientMemory","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"MemoryPressure"},{"message":"nodelet has no disk pressure","reason":"NodeletHasNoDiskPressure","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"DiskPressure"},{"message":"nodelet is posting ready status","reason":"NodeletReady","status":"True","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"Ready"}],"addresses":[{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"rajesh"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_type":"private_ipv4","bridge_type":"linux"}]}}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.node_id =res.body.id;
          done(err);
        });
    });
    it('returns all nodes', function(done) {
      request.get('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    

    it('returns the node by id', function(done) {
      request.get('/nodes/'+globalAny.node_id)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.node_id);
          done(err);
        });
    });


  });
