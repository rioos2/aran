import { expect } from 'chai';
import supertest = require('supertest');
const globalAny:any = global;

const request = supertest.agent(globalAny.apiServer);

describe('Ldap config creation API', function()    {
  it('returns the created ldap config', function(done) {
    request.post('/ldap/config')
    .ca(globalAny.rootCA)
      .send({"host": "ldap://ldap.example.com","port": "ldap port : eg: 636 as integer","enforce_starttls": true, "use_ldaps":true,"lookup_dn": "cn=admin,dc=megam,dc=org","lookup_password": "chennai28v","user_search": {"search_base": "dc=megam,dc=org",
      "search_filter_template": "cn={username}" },"group_search": {"search_base": "string","search_filter_template": "string","member_attributes": ["string"]},"ca_certs": "if the client needs to access LDAP  using TLS please add it the root chaing Certificate authority(CA) certificate","client_cert": "if the client needs to access LDAP  using TLS then add the certificate"})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
          globalAny.ldap_id =res.body.id;
        done(err);
      });
  });
/*  it(' test ldap config ', function(done) {
    request.post('/ldap/config/'+globalAny.ldap_id+'/test')
      .ca(globalAny.rootCA)
      .send({})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });
  it('import ldap config', function(done) {
    request.post('/ldap/import/'+globalAny.ldap_id)
      .ca(globalAny.rootCA)
      .send({})
      .expect(200)
      .end(function(err, res) {
        expect(res.body);
        done(err);
      });
  });*/

  });
