import { expect } from 'chai';
import supertest = require('supertest');
const globalAny:any = global;
const request = supertest.agent(globalAny.apiServer);
describe('Saml provider  API', function()    {
  it('returns the created saml provider', function(done) {
    request.post('/auth/saml/providers')
      .ca(globalAny.rootCA)
      .send({"description": "Login with Fantastic SAML IdP","idp_metadata": "<xml metadata from providers like onelogin, openam>","sp_base_url": "<callback_url>"})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        globalAny.saml_id =res.body.id;
        done(err);
      });
  });

  it('returns the list of all saml provider', function(done) {
    request.get('/auth/saml/providers')
      .ca(globalAny.rootCA)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        expect(res.body.items.length).to.equal(1);
        done(err);
      });
  });

  it('returns the saml provider', function(done) {
    request.get('/auth/saml/providers/'+globalAny.saml_id)
      .ca(globalAny.rootCA)
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });

  });
