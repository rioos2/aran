import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;


    describe('Storage pool API', function() {
      it('returns the created storage pool', function(done) {
        request.post('/storagespool')
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"name":"pool2", "connector_id":globalAny.st_id, "storage_info":{"disks":[{"disk":"/dev/sdb1", "disk_type":"/dev/sdb0", "point":"/home", "size":"500GB"}]}, "parameters":{}, "status":{"phase":"pending", "message":"", "reason":"", "conditions":[{"message":"", "reason":"", "status":"", "last_transition_time":"", "last_probe_time":"", "condition_type":""}]}})
          .expect(200)
          .end(function(err, res) {
            expect(res.body);
            globalAny.stp_id =res.body.id;
            done(err);
          });
      });

      it('returns all storage pool', function(done) {
        request.get('/storagespool')
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });

      it('returns the storagespool by storage connectors id ', function(done) {
        request.get('/storagespool/' + globalAny.st_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .expect(200)
          .end(function(err, res) {
            done(err);
          });
      });


      it('returns the storage pool status update', function(done) {
        request.put('/storagespool/'+ globalAny.stp_id+'/status')
          .set('Authorization', globalAny.bobo_bearer)
          .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
          .send({"status":{"phase":"ready", "message":"", "reason":"", "conditions":[{"message":"", "reason":"", "status":"ready", "last_transition_time":"", "last_probe_time":"", "condition_type":""}]}})
          .expect(200)
          .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.stp_id);
            done(err);
          });
      });

    });
