
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest('http://localhost:9636/api/v1');

describe('Origin API', function() {

it('returns the created origin', function(done) {
  request.post('/origins')
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({"object_meta": {"name": "rioosapi","account": globalAny.account_id,"created_at": "2017-12-11T11:29:50.547529+00:00","deleted_at": "","deletion_grace_period_seconds": 30,"cluster_name": "","labels": {},"annotations": {},"finalizers": [""
],"owner_references": [{"kind": "","api_version": "","name": "","uid": "","block_owner_deletion": false}]},"name": "rioosapi"})
    .expect(200)
    .end(function(err, res) {
    globalAny.origin_id=res.body.name;
      done(err);
    });
});

it('returns the created origin missing name', function(done) {
  request.post('/origins')
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({"object_meta": {"name": "rioosapi","account": globalAny.account_id,"created_at": "2017-12-11T11:29:50.547529+00:00","deleted_at": "","deletion_grace_period_seconds": 30,"cluster_name": "","labels": {},"annotations": {},"finalizers": [""
    ],"owner_references": [{"kind": "","api_version": "","name": "","uid": "","block_owner_deletion": false}]},"name": ""})
    .expect(400)
    .end(function(err, res) {
      done(err);
    });
});

it('returns the created origin missing account', function(done) {
  request.post('/origins')
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .send({"object_meta": {"name": "rioosapi","account": "","created_at": "2017-12-11T11:29:50.547529+00:00","deleted_at": "","deletion_grace_period_seconds": 30,"cluster_name": "","labels": {},"annotations": {},"finalizers": [""
],"owner_references": [{"kind": "","api_version": "","name": "","uid": "","block_owner_deletion": false}]},"name": "rioosapi"})
    .expect(400)
    .end(function(err, res) {
      done(err);
    });
});

it('returns the origin by name', function(done) {
  request.get('/origins/' + globalAny.origin_id)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .expect(200)
    .end(function(err, res) {
     expect(res.body.name).to.equal(globalAny.origin_id);
      done(err);
    });
});

it('returns the origin by wrong name', function(done) {
  request.get('/origins/' + globalAny.account_id)
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .expect(404)
    .end(function(err, res) {
      done(err);
    });
});

it('returns the all origin', function(done) {
  request.get('/origins')
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .expect(200)
    .end(function(err, res) {
    expect(res.body.items.length).to.equal(2);
      done(err);
    });
});

it('returns the all origin invalid url', function(done) {
  request.get('/origin')
  .set('Authorization', globalAny.bobo_bearer)
  .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
  .expect(404)
    .end(function(err, res) {
      done(err);
    });
});


it('returns the all origin without header ', function(done) {
  request.get('/origins')
  .expect(401)
    .end(function(err, res) {
      done(err);
    });
});
});
