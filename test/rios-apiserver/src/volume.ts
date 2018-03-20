//13 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('volume  API', function() {

    it('returns the created volume', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta": {"name":"", "account":globalAny.account_id,"labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": "50 GiB"})
        .expect(200)
        .end(function(err, res) {
          globalAny.vol_id =res.body.id;
          expect(res.body);
          done(err);
        });
    });
    it('update volume status', function(done) {
      request.put('/volumes/'+globalAny.vol_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status":{"phase":"ready","message":"","reason":"","conditions":[{"condition_type":"","message":"","reason":"","status":"ready","last_update_time":"","last_transition_time":"","last_probe_time":""}]}})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });



    it(' created volume empty account', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta": {"name":"", "account":"","labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": "50 GiB"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created volume missing account parameter', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta": {"name":"","labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": "50 GiB"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it(' created volume empty mount path', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta": {"name":"", "account":globalAny.account_id,"labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "","allocated": "50 GiB"})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('created volume empty allocated ', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta": {"account":globalAny.account_id,"name":"","labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": ""})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns error without header create volume', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
      .send({"object_meta": {"account":globalAny.account_id,"name":"","labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.assembly_id,"block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": "50 GiB"})
      .expect(401)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns create volume with owner reference', function(done) {
      request.post('/volumes')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"account":globalAny.account_id,"name":"","labels":{}, "annotations":{},"owner_references":[ {"kind":"Assembly",  "api_version":"v1",  "name":"lev.megam.io","uid":"","block_owner_deletion":false},{ "kind":"StoragePool","api_version":"v1", "name":"private", "uid":globalAny.stp_id,"block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,  "finalizers":[],"cluster_name":""}, "status": {"phase": "pending","message": "","reason": "", "conditions": [ {"message": "","reason": "","status": "False","last_transition_time": "2017-09-21T06:35:16Z","last_probe_time": "2017-09-21T06:35:16Z","condition_type": "OutOfDisk", "last_update_time": "2017-09-21T06:35:16Z"  }  ]}, "mount_path": "/var/lib/path","allocated": "50 GiB"})
      .expect(400)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the volumes by assembly', function(done) {
      request.get('/assemblys/'+globalAny.assembly_id+'/volumes')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the volumes by assembly invalid url', function(done) {
      request.get('/volumes/'+globalAny.assembly_id+'/assemblys')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the volumes by assembly wrong id', function(done) {
      request.get('/assemblys/89756784567/volumes')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('update volume ', function(done) {
      request.put('/volumes/'+globalAny.vol_id )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":"876234567","created_at":"2017-12-15T11:22:56.243005857+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"Assembly",
        "api_version":"v1","name":"lev.megam.io","uid":"876543212345678","block_owner_deletion":false},{"kind":"StoragePool","api_version":"v1","name":"private","uid":"87654567876544567","block_owner_deletion":false}], "finalizers":["orphan"],"cluster_name":""},
        "type_meta":{"kind":"Volume","api_version":"v1"},"mount_path":"/var/lib/path","allocated":"50 GiB","status":{"phase":"pending","message":"","reason":"","conditions":[{"condition_type":"OutOfDisk","message":"nodelet has sufficient disk space available",
        "reason":"NodeletHasSufficientDisk","status":"False","last_update_time":"2017-09-21T06:35:16Z","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z"}]}})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.vol_id);
          done(err);
        });
    });

    });
