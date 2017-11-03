import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Job  API', function() {

    it('returns the created job', function(done) {
      request.post('/jobs')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"type_meta": {"kind": "Jobs","api_version": "v1"},"object_meta": {"name": "811197535086452736","origin": "","uid": "","created_at": "","cluster_name": "","labels": {"group": "development","key2": "value2"},"annotations": {"key1": "value1","key2": "value2"}},"status": {"phase": "pending32","message": "","reason": "","conditions": [{ "message": "","reason": "","status": " ","last_transition_time": " ","last_probe_time": "", "condition_type": " " }] },"spec": {"node_id": "58974653215","target_ref": "8765431234567", "selector": {"group": "development","key2": "value2" }  }})
        .expect(200)
        .end(function(err, res) {
          globalAny.job_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });

    it('returns all nodes', function(done) {
      request.get('/jobs')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {

          done(err);
        });
    });

    it('update job status', function(done) {
      request.put('/jobs/'+globalAny.job_id+'/status' )
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status": {"phase": "pending","message": "update","reason": "","conditions": [{ "message": "","reason": "","status": " ","last_transition_time": " ","last_probe_time": "", "condition_type": " " }] }})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });




  });
