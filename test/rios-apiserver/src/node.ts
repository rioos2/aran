import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;


  describe('Node API', function() {
    it('returns the created node', function(done) {
      request.post('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"node_ip":"192.168.2.47","spec":{"assembly_cidr":"","external_id":"vino","provider_id":"","unschedulable":false,"taints":[{"key":"","value":"","effect":"","time_added":"2017-09-21T06:35:16Z"}]},"status":{"capacity":{"cpu":"4","memory": "16331164 KiB","storage": "16228764 MiB"},"allocatable":{"cpu":"4","memory": "1633116 KiB","storage": "1622876 MiB"},"phase":"pending","addresses":[{"node_type":"InternalIP","addresses":"192.168.2.47"}],"node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_type":"private_ipv4","bridge_type":"linux"}]},"conditions":[{"condition_type":"","status":"","last_heartbeat_time":"","last_transition_time":"",
        "reason":"","message":""}]},"object_meta":{"name":"xyz","origin":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"},"owner_references":[{"kind":"Node","api_version":"v1","name":"ddd","uid":"","block_owner_deletion":true}]},"type_meta":{"kind":"Node","api_version":"v1"}})
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

    it('update node status', function(done) {
      request.put('/nodes/'+ globalAny.node_id+'/status')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"ready", "message":"", "reason":"", "conditions":[{"message":"", "reason":"", "status":"ready", "last_transition_time":"", "last_probe_time":"", "condition_type":""}]}})
        .expect(200)
        .end(function(err, res) {
        expect(res.body.id).to.equal(globalAny.node_id);
          done(err);
        });
    });

    it('returns the node by id', function(done) {
      request.get('/nodes/' + globalAny.node_id)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.node_id);
          done(err);
        });
    });


  });
