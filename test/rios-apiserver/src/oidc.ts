import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);

describe('open id provider  API', function()    {
  it('returns the created open id provider', function(done) {
    request.post('/auth/oidc/providers/oauth02')
    .ca(globalAny.rootCA)
      .send({"description": "Login with Google","issuer": "https://accounts.google.com","base_url": "<callback_url>", "client_secret": "0909090909 (from console.google)", "client_id": "0909090909 (from console.google)", "verify_server_certificate": true, "ca_certs": "string"})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
          globalAny.oidc_id =res.body.id;
        done(err);
      });
  });
  it('returns the list of all openid provider', function(done) {
    request.get('/auth/oidc/providers')
    .ca(globalAny.rootCA)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });

  it('returns the openid provider', function(done) {
    request.get('/auth/oidc/providers/'+globalAny.oidc_id)
    .ca(globalAny.rootCA)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });
  });
