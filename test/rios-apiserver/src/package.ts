// account test case total -9
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.marketplaceServer);

describe('Package API', function() {

  it('returns the created Ubuntu', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"ubuntu","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"78787878","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.ubuntu_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("ubuntu");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created Centos', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"centos","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111000","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.centos_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("centos");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Debian', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"debian","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111001","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.debian_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("debian");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created Debian second ', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"debian","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111005","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.debian_sec_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("debian");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Coreos', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"coreos","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111002","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.coreos_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("coreos");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Fedora', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"fedora","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111004","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.fedora_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("fedora");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Freebsd', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"freebsd","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111006","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.freebsd_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("freebsd");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Windows', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"windows","account":globalAny.account_id,"labels":{"rioos_category":"machine"}},"version_number":"11111007","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.windows_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("windows");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created Nginx', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"nginx","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111003","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.nginx_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("nginx");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Jenkins', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"jenkins","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111008","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.jenkins_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("jenkins");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created MariaDB', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"mariadb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111009","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.mariadb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("mariadb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Influxdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"influxdb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111010","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.influxdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("influxdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Orientdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"orientdb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111011","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.orientdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("orientdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Cockroachdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"cockroachdb/cockroach","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111012","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.cockroachdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("cockroachdb/cockroach");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Rethinkdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"rethinkdb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111013","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.rethinkdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("rethinkdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });


  it('returns the created Apache', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"webdevops/php-apache-dev","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111014","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.apache_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("webdevops/php-apache-dev");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created neo4j', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"neo4j","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111015","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.neo4j_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("neo4j");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Couchdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"couchdb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111016","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.couchdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("couchdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Aerospike', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"aerospike","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111017","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.aerospike_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("aerospike");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Redis', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"redis","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111018","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.redis_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("redis");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created memcached', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"memcached","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111019","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.memcached_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("memcached");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Postgres', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"postgres","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111020","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.postgres_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("postgres");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Psitrax/Powerdns', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"psitrax/powerdns","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111021","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.powerdns_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("psitrax/powerdns");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });


  it('returns the created Fluentbit', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"brycekahle/fluentbit","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111022","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.fluentbit_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("brycekahle/fluentbit");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Chronograf', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"chronograf","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111023","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.chronograf_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("chronograf");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Cassandra', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"cassandra","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111024","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.cassandra_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("cassandra");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });it('returns the created Voltdb', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"voltdb","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111025","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.voltdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("voltdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Elasticsearch', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"elasticsearch","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111026","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.elasticsearch_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("elasticsearch");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created Kibana', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"kibana","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111027","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.kibana_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("kibana");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created fabric orderer', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"hyperledger/fabric-orderer","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111028","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.hyperledger_fabric_orderer_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("hyperledger/fabric-orderer");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created fabric CA', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"hyperledger/fabric-ca","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111029","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.hyperledger_fabric_ca_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("hyperledger/fabric-ca");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created fabric peer', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"hyperledger/fabric-peer","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111030","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.hyperledger_fabric_peer_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("hyperledger/fabric-peer");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created tyk gateway', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"tykio/tyk-gateway","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111031","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.tyk_gateway_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("tykio/tyk-gateway");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created tyk dashboard', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"tykio/tyk-dashboard","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111032","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.tyk_dashboard_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("tykio/tyk-dashboard");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created tyk pump-docker-pub', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"tykio/tyk-pump-docker-pub","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111033","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.tyk_pump_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("tykio/tyk-pump-docker-pub");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created mongo', function(done) {
    request.post('/packages')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta": {"name":"mongo","account":globalAny.account_id,"labels":{"rioos_category":"container"}},"version_number":"11111034","extension":"tar.gz"})
      .expect(200)
      .end(function(err, res) {
        globalAny.mongo_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("mongo");
        expect(res.body.type_meta.kind).to.equal(globalAny.package);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

});
