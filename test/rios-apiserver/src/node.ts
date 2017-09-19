import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Node API', function() {
  describe('Node API', function() {
    it('returns the created node', function(done) {
      request.post('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"spec":{"assembly_cidr":"","external_id":"","provider_id":"","unschedulable":false,"taints":[{"key":"","value":"","effect":"","time_added":""}]},"status":{"capacity":{"cpu":"4","memory": "16331164Ki","storage": "16228764Mb"},"allocatable":{"cpu":"4","memory": "16331164Ki","storage": "16228764Mb"},"phase":"","addresses":[{"node_type":"","addresses":""}],"node_info":{"machine_id":"","system_uuid":"","kernel_version":"","os_image":"","architecture":""},"conditions":[{"condition_type":"","status":"","last_heartbeat_time":"","last_transition_time":"",
        "reason":"","message":""}]},"object_meta":{"name":"xyz","namespace":"","uid":"","created_at":"","cluster_name":"","labels":{"group":"development","key2":"value2"},"annotations":{"key1":"value1","key2":"value2"},"owner_references":[{"kind":"Node","api_version":"v1","name":"ddd","uid":"","block_owner_deletion":true}]},"type_meta":{"kind":"Assemblyfactory","api_version":"v1"}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
    it('returns all nodes', function(done) {
      request.get('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .expect(200)
        .end(function(err, res) {
          expect(res.body.results.length).to.equal(1);
          done(err);
        });
    });
  });
  });
