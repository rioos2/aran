//7 test cases
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Ingress  API', function() {

    it('returns the created ingress', function(done) {
      request.post('/ingresses')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ingress-sample","account":globalAny.account_id,"owner_references":[ {"kind":"AssemblyFactory",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":false},
        { "kind":"Service","api_version":"v1", "name":"private", "uid":globalAny.services_id,"block_owner_deletion":false}]},"spec": {"rules": [{"host": "foo.bar.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo",
        "backend": {"service_port": 80}}]}}},{"host": "bar.baz.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo","backend": {"service_port": 80}}]}}}]}})
        .expect(200)
        .end(function(err, res) {
          globalAny.ingress_id =res.body.id;
          expect(res.body.type_meta.kind).to.equal(globalAny.ingress);
          expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
        });
    });

    it('update ingress ', function(done) {
      request.put('/ingresses/'+globalAny.ingress_id )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ingress-sample","account":globalAny.account_id,"created_at":"2018-07-11T09:06:10.430927061+00:00","deleted_at":"","deletion_grace_period_seconds":30,"labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"lev.megam.io","uid":globalAny.asm_fac_id,
        "block_owner_deletion":false},{"kind":"Service","api_version":"v1","name":"lbservice","uid":globalAny.services_id,"block_owner_deletion":false}],"initializers":{"pending":[],"result":{"type_meta":{"kind":"","api_version":""},"status":"","message":"",
        "reason":"","details":{"name":"","group":"","kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},"spec":{"backend":{"service_port":0},"tls":[],"rules":[{"host":"foo.bar.com","ingress_rule_value":{"http":{"paths":[{"path":"/foo",
        "backend":{"service_port":80}}]}}},{"host":"bar.baz.com","ingress_rule_value":{"http":{"paths":[{"path":"/foo","backend":{"service_port":80}}]}}}]},"status":{"ingress":[]},"created_at":""})
        .expect(200)
        .end(function(err, res) {
          expect(res.body.id).to.equal(globalAny.ingress_id);
          done(err);
        });
    });


    it('update ingress status', function(done) {
      request.put('/ingresses/'+globalAny.ingress_id+'/status' )
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"status": {"ingress": [{"ip": "192.168.1.66","hostname": "suganya.rioos.xyz"}] }})
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns the created ingress missing type name', function(done) {
      request.post('/ingresses')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"","account":globalAny.account_id,"owner_references":[ {"kind":"AssemblyFactory",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":false},
        { "kind":"Service","api_version":"v1", "name":"private", "uid":globalAny.services_id,"block_owner_deletion":false}]},"spec": {"rules": [{"host": "foo.bar.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo",
        "backend": {"service_port": 80}}]}}},{"host": "bar.baz.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo","backend": {"service_port": 80}}]}}}]}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created ingress missing owner_references', function(done) {
      request.post('/ingresses')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"ingress-sample","account":globalAny.account_id,"owner_references":[ {"kind":"AssemblyFactory",  "api_version":"v1",  "name":"lev.megam.io","uid":globalAny.asm_fac_id,"block_owner_deletion":false},
        { "kind":"Service","api_version":"v1", "name":"private", "uid":"","block_owner_deletion":false}]},"spec": {"rules": [{"host": "foo.bar.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo",
        "backend": {"service_port": 80}}]}}},{"host": "bar.baz.com","ingress_rule_value" :{"http" :{"paths":[{"path": "/foo","backend": {"service_port": 80}}]}}}]}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the ingress by assemblyfactorys', function(done) {
      request.get('/assemblyfactorys/'+globalAny.asm_fac_id+'/ingresses')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });


    it('returns the ingress by assemblyfactorys wrong id', function(done) {
      request.get('/assemblyfactorys/89756784567/ingresses')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
