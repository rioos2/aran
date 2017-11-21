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

    it('returns all storage connectors', function(done) {
      request.get('/storageconnectors')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the storage connectors by id', function(done) {
      request.get('/storageconnectors/' + globalAny.st_id)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
         expect(res.body.id).to.equal(globalAny.st_id);
          done(err);
        });
    });

    it('update storage connectors status', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id+'/status' )
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status": {"phase": "ready","message": "update","reason": "","conditions": [{ "message": "","reason": "","status": " ","last_transition_time": " ","last_probe_time": "", "condition_type": " " }] }})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update storage connectors', function(done) {
      request.put('/storageconnectors/'+globalAny.st_id)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"id":globalAny.st_id, "name":"iscsi","host_ip":"192.168.1.110","storage_type":"iscsi","parameters": {"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"}, "storage_info":{"disks": [{"disk": "/dev/sdb","disk_type": "/dev/sdb1","point": "/home","size": "50GB"},{"disk": "/dev/sdb1","disk_type": "/dev/sdb2","point": "/home/ranji","size": "500GB"}]},"status":{"phase":"","message": "","reason": "","conditions":[{"message":"","reason":"","status":" ","last_transition_time":" ","last_probe_time":" ","condition_type":" "}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body);
          globalAny.st_id =res.body.id;
          done(err);
        });
    });
  });

  });
