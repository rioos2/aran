
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Team API', function() {

it('returns the created Team', function(done) {
  request.post('/teams')
  .ca(globalAny.rootCA)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({  "name": "development",  "object_meta": { "name":"deployment","account":"875703416347697152","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":""  },"metadata": {"origin":"rioos", "team":"development"}  })
    .expect(200)
    .end(function(err, res) {
    globalAny.team_id=res.body.name;
      done(err);
    });
});

it('returns the created Team missing name', function(done) {
  request.post('/teams')
  .ca(globalAny.rootCA)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({  "name": "",  "object_meta": { "name":"deployment","account":"875703416347697152","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":""  },"metadata": {"origin":"rioos", "team":"development"}  })
    .expect(400)
    .end(function(err, res) {
      done(err);
    });
});


it('returns the created Team missing origin', function(done) {
  request.post('/teams')
  .ca(globalAny.rootCA)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({  "name": "support",  "object_meta": { "name":"deployment","account":"875703416347697152","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":""  },"metadata": {}  })
    .expect(400)
    .end(function(err, res) {
      done(err);
    });
});


it('returns the created Team missing account', function(done) {
  request.post('/teams')
  .ca(globalAny.rootCA)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({  "name": "development",  "object_meta": { "name":"deployment","account":"","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":"" },"metadata": {"origin":"rioos", "team":"development"} })
    .expect(400)
    .end(function(err, res) {
      done(err);
    });
});

it('returns the created Team in valid url', function(done) {
  request.post('/team')
  .ca(globalAny.rootCA)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({  "name": "development",  "object_meta": { "name":"deployment","account":"875703416347697152","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":""  },"metadata": {"origin":"rioos", "team":"development"}  })
    .expect(404)
    .end(function(err, res) {
      done(err);
    });
});

it('returns the created Team without header', function(done) {
  request.post('/teams')
  .ca(globalAny.rootCA)
  .send({  "name": "development",  "object_meta": { "name":"deployment","account":"875703416347697152","labels":{},"annotations":{}, "owner_references":[ {"kind":"","api_version":"",  "name":"","uid":"",  "block_owner_deletion":false} ],"created_at":"", "deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],
     "cluster_name":""  },"metadata": {"origin":"rioos", "team":"development"}  })
    .expect(401)
    .end(function(err, res) {
      done(err);
    });
});


});
