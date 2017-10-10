import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Storage API', function() {
  describe('Storage Connector API', function() {
    it('returns the created storage connectors', function(done) {
      request.post('/storageconnectors')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"name":"iscsi","host_ip":"192.168.1.100","storage_type":"iscsi","parameters": {"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"}, "storage_info":{"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/ranji","size": "500GB"}]},"status":{"phase":"","message": "","reason": "","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":" ","condition_type":" "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.st_id =res.body.id;
          done(err);
        });
    });

  });

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

    it('returns the storage pool status update', function(done) {
      request.put('/storagespool/'+ globalAny.stp_id+'/status')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"pending", "message":"", "reason":"", "conditions":[{"message":"", "reason":"", "status":"ready", "last_transition_time":"", "last_probe_time":"", "condition_type":""}]}})
        .expect(200)
        .end(function(err, res) {
         expect(res.body);
          done(err);
        });
    });

  });


  });
