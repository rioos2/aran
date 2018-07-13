import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('Settings Map  API', function() {
    it('returns the created settings map', function(done) {
      this.timeout(4000)
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"object_meta":{"name":"cluster_info"},"metadata":{"origin":"rioos_system"},"data":{"ui.digicloud.compute_type":"cpu","ui.digicloud.cpu":"1","ui.digicloud.disk":"10","ui.digicloud.disk_type":"hdd","ui.digicloud.domain":".svc.local","ui.digicloud.os_name":"ubuntu","ui.digicloud.os_version":"16.04",
        "ui.digicloud.ram":"1","ui.digicloud.secret_key_length":"2048","ui.digicloud.secret_type": "SSH-1(RSA3)","ui.digicloud.secret_type_names":"SSH-1(RSA), SSH-1(RSA2), SSH-1(RSA3)","ui.digicloud.trusted_key":"rioos_sh/ssh-auth"}})
        .expect(200)
        .end(function(err, res) {
          globalAny.set_map_name =res.body.object_meta.name;
          expect(res.body);
          done(err);
        });
    });

    it('returns the created settings map missing origin', function(done) {
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{"name":"cluster_info"},"metadata":{},"data":{"ui.digicloud.compute_type":"cpu","ui.digicloud.cpu":"1","ui.digicloud.disk":"10","ui.digicloud.disk_type":"hdd","ui.digicloud.domain":".svc.local","ui.digicloud.os_name":"ubuntu","ui.digicloud.os_version":"16.04",
      "ui.digicloud.ram":"1","ui.digicloud.secret_key_length":"2048","ui.digicloud.secret_type": "SSH-1(RSA3)","ui.digicloud.secret_type_names":"SSH-1(RSA), SSH-1(RSA2), SSH-1(RSA3)","ui.digicloud.trusted_key":"rioos_sh/ssh-auth"}})
        .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns the created settings map missing name', function(done) {
      request.post('/settingsmap')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{"name":""},"metadata":{"origin":"rioos_system"},"data":{"ui.digicloud.compute_type":"cpu","ui.digicloud.cpu":"1","ui.digicloud.disk":"10","ui.digicloud.disk_type":"hdd","ui.digicloud.domain":".svc.local","ui.digicloud.os_name":"ubuntu","ui.digicloud.os_version":"16.04",
      "ui.digicloud.ram":"1","ui.digicloud.secret_key_length":"2048","ui.digicloud.secret_type": "SSH-1(RSA3)","ui.digicloud.secret_type_names":"SSH-1(RSA), SSH-1(RSA2), SSH-1(RSA3)","ui.digicloud.trusted_key":"rioos_sh/ssh-auth"}})
      .expect(400)
        .end(function(err, res) {
          expect(res.body);
          done(err);
        });
    });

    it('returns settings map by origin and map name', function(done) {
      request.get('/origins/rioos_system/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by origin and map name', function(done) {
      request.get('/origins/rioos_system/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
        .expect(406)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by origin and invalid map name', function(done) {
      request.get('/origins/rioos_system/settingsmap/clus')
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by invalid origin and map name', function(done) {
      request.get('/origins/rioos/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

    it('returns settings map by origin and map name invalid url', function(done) {
      request.get('/origin/rioos_system/settingsmap/'+globalAny.set_map_name)
      .ca(globalAny.rootCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(404)
        .end(function(err, res) {
          done(err);
        });
    });

  });
