import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('open id provider  API', function()    {
  it('returns the created open id provider', function(done) {
    request.post('/auth/oidc/providers/'+globalAny.provider_id)
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
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });

  it('returns the saml provider', function(done) {
    request.get('/auth/oidc/providers/'+globalAny.oidc_id)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });
  });
