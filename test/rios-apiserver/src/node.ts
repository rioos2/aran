import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Node API', function() {
  describe('Node API', function() {
    it('returns the created node', function(done) {
      request.post('/nodes')
        .set('Authorization', globalAny.bobo_bearer)
        .send({"spec":{"assemblyCIDR":"","externalID":"","providerID":"","unschedulable":"false","taints":[{"key":"","value":"","effect":"","timeAdded":""}]},
"status":{"capacity":{"cpu":{"fixed_range":{"value":"155.00","scale":"Core"},"infinite_range":{"unscale":"155.0000008115","scale":"Core"},
"quantity":"155core","format":"DecimalExponent"},"mem":{"fixed_range":{"value":"10.564215","scale":"Mega"},"infinite_range":{
"unscale":"86.15150002548","scale":"4"},"quantity":"86M","format":"DecimalSI"},"disk":{"fixed_range":{"value":"10.564215","scale":"Mega"},
"infinite_range":{"unscale":"86.15150002548","scale":"4"},"quantity":"86M","format":"DecimalSI"}},"allocatable":{"cpu":{"fixed_range":{
"value":"155.00","scale":"Core"},"infinite_range":{"unscale":"155.0000008115","scale":"Core"},"quantity":"155core","format":"DecimalExponent"},
"mem":{"fixed_range":{"value":"10.564215","scale":"Mega"},"infinite_range":{"unscale":"86.15150002548","scale":"4"},"quantity":"86M",
"format":"DecimalSI"},"disk":{"fixed_range":{"value":"10.564215","scale":"Mega"},"infinite_range":{"unscale":"86.15150002548","scale":"4"},
"quantity":"86M","format":"DecimalSI"}},"phase":"","addresses":[{"nodeAddress":{"nodeType":"","addresses":""}}],"nodeInfo":{"machineID":"","systemUUID":"","kernelVersion":"","oSImage":"","architecture":""},"conditions":[{"conditionType":"","status":"","lastHeartbeatTime":"","lastTransitionTime":"",
"reason":"","message":""}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });
  });
  });
