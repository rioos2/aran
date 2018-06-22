// account test case total -9
import { expect } from 'chai';
import supertest = require('supertest');

const globalAny:any = global;
const request = supertest.agent(globalAny.marketplaceServer);

describe('Marketplace API', function() {

  it('returns the created Ubuntu', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"ubuntu", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"ubuntu","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"ubuntu", "uid":globalAny.ubuntu_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "16.04","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension":  "tar.gz"},"icon" : "ubuntu.png","description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
      "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
      "category": "machine", "version": "16.04", "icon": "ubuntu.png", "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("ubuntu");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Centos', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"centos", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"centos","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"centos", "uid":globalAny.centos_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "7.4","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "centos.png","description": "The CentOS Project is a community-driven free software effort focused on delivering a robust open source ecosystem. For users, we offer a consistent manageable platform that suits a wide variety of deployments. ",
      "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
      "category": "machine", "version": "7.4", "icon": "centos.png", "description": "The CentOS Project is a community-driven free software effort focused on delivering a robust open source ecosystem. For users, we offer a consistent manageable platform that suits a wide variety of deployments. ",
      "status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("centos");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Debian', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"debian", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"debian","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"debian", "uid":globalAny.debian_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "8","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "debian.png","description": "Debian is a free operating system (OS) for your computer. An operating system is the set of basic programs and utilities that make your computer run. ",
      "status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
      "category": "machine", "version": "8", "icon": "debian.png", "description": "Debian is a free operating system (OS) for your computer. An operating system is the set of basic programs and utilities that make your computer run. ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("debian");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Debian second ', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"debian", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"debian","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"debian", "uid":globalAny.debian_sec_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "9","characteristics" :{"extension": "tar.gz"},"icon" : "debian.png","description": "Debian is a free operating system (OS) for your computer. An operating system is the set of basic programs and utilities that make your computer run. ","status":{"phase":"SyncPending"},
      "metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}], "category": "machine", "version": "9", "icon": "debian.png", "description": "Debian is a free operating system (OS) for your computer. An operating system is the set of basic programs and utilities that make your computer run. ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("debian");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Coreos', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"coreos", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"coreos","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"coreos", "uid":globalAny.coreos_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "1576.5.0","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "coreos.png","description": "Container Linux by CoreOS (formerly CoreOS Linux) is an open-source lightweight operating system based on the Linux kernel and designed for providing infrastructure to clustered deployments, while focusing on automation, ease of application deployment, security, reliability and scalability.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop":
      {"command": []}, "post_start": {"command": []}}}], "category": "machine", "version": "1576.5.0", "icon": "coreos.png", "description": "Container Linux by CoreOS (formerly CoreOS Linux) is an open-source lightweight operating system based on the Linux kernel and designed for providing infrastructure to clustered deployments, while focusing on automation, ease of application deployment, security, reliability and scalability.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("coreos");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Fedora', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"fedora", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"fedora","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"fedora", "uid":globalAny.fedora_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "27","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "fedora.png","description": "Fedora contains software distributed under various free and open-source licenses and aims to be on the bleeding edge of such technologies.Fedora is the upstream source of the commercial Red Hat Enterprise Linux distribution.","status":{"phase":"SyncPending"},
      "metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
       "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}], "category": "machine", "version": "27", "icon": "fedora.png", "description": "Fedora contains software distributed under various free and open-source licenses and aims to be on the bleeding edge of such technologies.Fedora is the upstream source of the commercial Red Hat Enterprise Linux distribution.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("fedora");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Freebsd', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"freebsd", "account":globalAny.account_id}, "plans":[{"object_meta": {"name":"freebsd", "account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"freebsd", "uid":globalAny.freebsd_package_id,
      "block_owner_deletion":false}]},"category": "machine","version": "11.1","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "freebsd.png","description": "FreeBSD is an operating system used to power modern servers, desktops, and embedded platforms. ","status":{"phase":"SyncPending"},
      "metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket":
      {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}], "category": "machine", "version": "11.1", "icon": "freebsd.png",
      "description": "FreeBSD is an operating system used to power modern servers, desktops, and embedded platforms.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("freebsd");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Windows', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"windows", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"windows","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"windows", "uid":globalAny.windows_package_id,"block_owner_deletion":false}]},
      "category": "machine","version": "2008","characteristics" :{"rioos_sh_image_extension": "img", "rioos_sh_market_image_extension": "tar.gz"},"icon" : "windows.png","description": "Windows Server 2008 helps IT professionals to increase the flexibility and reliability of their server infrastructure while offering developers a more robust web and applications platform for building connected applications and services. ","status":{"phase":"SyncPending"},
      "metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}], "category": "machine", "version": "2008", "icon": "windows.png", "description": "Windows Server 2008 helps IT professionals to increase the flexibility and reliability of their server infrastructure while offering developers a more robust web and applications platform for building connected applications and services.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("windows");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Nginx', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"nginx", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"nginx","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"nginx", "uid":globalAny.nginx_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "1.13","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "nginx.png","description": "nginx [engine x] is an HTTP and reverse proxy server, a mail proxy server, and a generic TCP/UDP proxy server, originally written by Igor Sysoev.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}},
      "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":80,"host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "1.13", "icon": "nginx.png", "description": "Nginx [engine x] is an HTTP and reverse proxy server, a mail proxy server, and a generic TCP/UDP proxy server, originally written by Igor Sysoev.",
      "status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("nginx");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Jenkins', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"jenkins", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"jenkins","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"windows", "uid":globalAny.jenkins_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "2.60.3-alpine","characteristics" :{ "rioos_sh_market_image_extension": "tar.gz"},"icon" : "jenkins.png","description": "Jenkins is a powerful application that allows continuous integration and continuous delivery of projects, regardless of the platform you are working on. It is a free source that can handle any kind of build or continuous integration. ","status":{"phase":"SyncPending"},
      "metadata": {"origin": "rioos_system"},"ports":[{"container_port":8080,     "host_ip":"","host_port":0,"protocol":""}], "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}],
      "category": "container", "version": "2.60.3-alpine", "icon": "jenkins.png", "description": "Jenkins is a powerful application that allows continuous integration and continuous delivery of projects, regardless of the platform you are working on. It is a free source that can handle any kind of build or continuous integration.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("jenkins");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Influxdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/rioosinfluxdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosinfluxdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"influxdb", "uid":globalAny.influxdb_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "1.3.7","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "influxdb.png","description": "InfluxDB is a time series database built from the ground up to handle high write and query loads. InfluxDB is meant to be used as a backing store for any use case involving large amounts of timestamped data, including DevOps monitoring, application metrics, IoT sensor data, and real-time analytics. ","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [],
       "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},
       "ports":[{"container_port":8086, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "1.3.7", "icon": "influxdb.png", "description": "InfluxDB is a time series database built from the ground up to handle high write and query loads. InfluxDB is meant to be used as a backing store for any use case involving large amounts of timestamped data, including DevOps monitoring, application metrics, IoT sensor data, and real-time analytics.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosinfluxdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });it('returns the created Orientdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/rioosorientdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosorientdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"orientdb", "uid":globalAny.orientdb_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "2.0.18","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "orientdb.png","description": "OrientDB is the first Multi-Model Open Source NoSQL DBMS that combines the power of graphs and the flexibility of documents into one scalable, high-performance operational database. ","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop":
      {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":2424, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "2.0.18", "icon": "orientdb.png", "description": "OrientDB is the first Multi-Model Open Source NoSQL DBMS that combines the power of graphs and the flexibility of documents into one scalable, high-performance operational database.",
      "status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        globalAny.orientdb_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosorientdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });it('returns the created Cockroachdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/riooscockroachdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooscockroachdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"cockroachdb/cockroach", "uid":globalAny.cockroachdb_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "1.1.3","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "cockroachdb.png","description": "CockroachDB is an open source, survivable, strongly consistent, scale-out SQL database. ","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"} ,
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []},
       "post_start": {"command": []}},"ports":[{"container_port":26257, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "1.1.3", "icon": "cockroachdb.png", "description": "CockroachDB is an open source, survivable, strongly consistent, scale-out SQL database. ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooscockroachdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Rethinkdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/rioosrethinkdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosrethinkdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"rethinkdb", "uid":globalAny.rethinkdb_package_id,"block_owner_deletion":false}]},
      "category": "container","stateful_volumes": [{"name": "postgres_logs","volumes": {"host_path": "/var/logs/postgres"},"volume_mounts": {"mount_path": "/var/logs/postgres_inside_container/postgres.conf"},"settingmap" : {"uri" : "","uid" : "8574692245852336421","rioos_binder": ["mongodb", "redis"],
      "map_type":"static/template/public_url" }},{"name": "postgres_data","volumes": {"host_path": "/var/lib/pgdata"},"volume_mounts": {"mount_path": "/var/lib/pgdata_inside_container"},"settingmap" : {"map_type":"static/template/public_url" }}],"version": "2.3.6","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "rethinkdb.png","description": "RethinkDB is an open-source, document database that makes it easy to build and scale realtime apps.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":8080, "host_ip":"","host_port":0,"protocol":""}]}],
      "category": "container", "version": "2.3.6", "icon": "rethinkdb.png", "description": "RethinkDB is an open-source, document database that makes it easy to build and scale realtime apps. ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosrethinkdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });


  it('returns the created Apache', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/rioosapache", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosapache","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"webdevops/php-apache-dev", "uid":globalAny.apache_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "2.4.33","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "apache.png","description": "PHP with Apache for Development (eg. with xdebug)","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [],
      "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},
      "ports":[{"container_port":80, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "2.4.33", "icon": "apache.png", "description": "PHP with Apache for Development (eg. with xdebug) ","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        globalAny.apache_package_id =res.body.id;
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosapache");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Fabric', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta": { "name": "rioosfabric", "account": globalAny.account_id }, "plans": [{ "object_meta": { "name": "hyperledger/fabric-orderer", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "hyperledger/fabric-orderer", "uid": globalAny.hyperledger_fabric_orderer_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "x86_64-1.0.5", "characteristics": { "rioos_sh_image_extension": "raw", "rioos_sh_market_image_extension": "tar.gz" }, "icon": "hyperledger_fabric_orderer.png", "description": "Fabric Orderer", "ports": [{ "container_port": 7050, "host_ip": "", "host_port": 0, "protocol": "" }], "envs": { "ORDERER_GENERAL_LOGLEVEL": { "value": "debug", "required": "true", "editable": "" }, "ORDERER_GENERAL_LISTENADDRESS": { "value": "0.0.0.0", "required": "true", "editable": "" }, "ORDERER_GENERAL_LISTENPORT": { "value": "7050", "required": "true", "editable": "" }, "ORDERER_GENERAL_GENESISPROFILE": { "value": "ChainHero", "required": "true", "editable": "" }, "ORDERER_GENERAL_GENESISMETHOD": { "value": "file", "required": "true", "editable": "" }, "ORDERER_GENERAL_GENESISFILE": { "value": "/var/hyperledger/orderer/orderer.genesis.block", "required": "true", "editable": "" }, "ORDERER_GENERAL_LOCALMSPID": { "value": "hf.chainhero.io", "required": "true", "editable": "" }, "ORDERER_GENERAL_LOCALMSPDIR": { "value": "/var/hyperledger/orderer/msp", "required": "true", "editable": "" }, "ORDERER_GENERAL_TLS_ENABLED": { "value": "true", "required": "true", "editable": "" }, "ORDERER_GENERAL_TLS_PRIVATEKEY": { "value": "/var/hyperledger/orderer/tls/server.key", "required": "true", "editable": "" }, "ORDERER_GENERAL_TLS_CERTIFICATE": { "value": "/var/hyperledger/orderer/tls/server.crt", "required": "true", "editable": "" }, "ORDERER_GENERAL_TLS_ROOTCAS": { "value": "/var/hyperledger/orderer/tls/ca.crt", "required": "true", "editable": "" } }, "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } } }, { "object_meta": { "name": "hyperledger/fabric-ca", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "hyperledger/fabric-ca", "uid": globalAny.hyperledger_fabric_ca_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "x86_64-1.0.5", "characteristics": { "rioos_sh_image_extension": "raw", "rioos_sh_market_image_extension": "tar.gz" }, "icon": "hyperledger_fabric_ca.png", "description": "Fabric CA", "ports": [{ "container_port": 7054, "host_ip": "", "host_port": 0, "protocol": "" }], "envs": { "FABRIC_CA_HOME": { "value": "/etc/hyperledger/fabric-ca-server", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_CA_NAME": { "value": "ca.org1.hf.chainhero.io", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_CA_CERTFILE": { "value": "/etc/hyperledger/fabric-ca-server-config/ca.org1.hf.chainhero.io-cert.pem", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_CA_KEYFILE": { "value": "/etc/hyperledger/fabric-ca-server-config/5289b538c2d82ffaedc0922070c8425ca5763a6727710a71803590228fd35a72_sk", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_TLS_ENABLED": { "value": "true", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_TLS_CERTFILE": { "value": "/etc/hyperledger/fabric-ca-server-config/ca.org1.hf.chainhero.io-cert.pem", "required": "true", "editable": "" }, "FABRIC_CA_SERVER_TLS_KEYFILE": { "value": "/etc/hyperledger/fabric-ca-server-config/5289b538c2d82ffaedc0922070c8425ca5763a6727710a71803590228fd35a72_sk", "required": "true", "editable": "" } }, "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } } }, { "object_meta": { "name": "hyperledger/fabric-peer", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "hyperledger/fabric-peer", "uid": globalAny.hyperledger_fabric_peer_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "x86_64-1.0.5", "characteristics": { "rioos_sh_image_extension": "raw", "rioos_sh_market_image_extension": "tar.gz" }, "icon": "hyperledger_fabric_peer.png", "description": "Fabric peer connector", "ports": [{ "container_port": 7051, "host_ip": "", "host_port": 0, "protocol": "" }], "envs": { "CORE_VM_ENDPOINT": { "value": "unix:///host/var/run/docker.sock", "required": "true", "editable": "" }, "CORE_VM_DOCKER_ATTACHSTDOUT": { "value": "true", "required": "true", "editable": "" }, "CORE_LOGGING_LEVEL": { "value": "DEBUG", "required": "true", "editable": "" }, "CORE_PEER_NETWORKID": { "value": "chainhero", "required": "true", "editable": "" }, "CORE_PEER_PROFILE_ENABLED": { "value": "true", "required": "true", "editable": "" }, "CORE_PEER_TLS_ENABLED": { "value": "true", "required": "true", "editable": "" }, "CORE_PEER_TLS_CERT_FILE": { "value": "/var/hyperledger/tls/server.crt", "required": "true", "editable": "" }, "CORE_PEER_TLS_KEY_FILE": { "value": "/var/hyperledger/tls/server.key", "required": "true", "editable": "" }, "CORE_PEER_TLS_ROOTCERT_FILE": { "value": "/var/hyperledger/tls/ca.crt", "required": "true", "editable": "" }, "CORE_PEER_ID": { "value": "peer0.org1.hf.chainhero.io", "required": "true", "editable": "" }, "CORE_PEER_ADDRESSAUTODETECT": { "value": "true", "required": "true", "editable": "" }, "CORE_PEER_ADDRESS": { "value": "peer0.org1.hf.chainhero.io:7051", "required": "true", "editable": "" }, "CORE_PEER_GOSSIP_EXTERNALENDPOINT": { "value": "peer0.org1.hf.chainhero.io:7051", "required": "true", "editable": "" }, "CORE_PEER_GOSSIP_USELEADERELECTION": { "value": "true", "required": "true", "editable": "" }, "CORE_PEER_GOSSIP_ORGLEADER": { "value": "false", "required": "true", "editable": "" }, "CORE_PEER_GOSSIP_SKIPHANDSHAKE": { "value": "true", "required": "true", "editable": "" }, "CORE_PEER_LOCALMSPID": { "value": "org1.hf.chainhero.io", "required": "true", "editable": "" }, "CORE_PEER_MSPCONFIGPATH": { "value": "/var/hyperledger/msp", "required": "true", "editable": "" }, "CORE_PEER_TLS_SERVERHOSTOVERRIDE": { "value": "peer0.org1.hf.chainhero.io", "required": "true", "editable": "" } }, "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } } }], "category": "blockchain_template", "version": "x86_64-1.0.5", "icon": "fabric.png", "description": "Hyperledger Fabric is a blockchain framework implementation and one of the Hyperledger projects hosted by The Linux Foundation.", "status": { "phase": "SyncPending" } })
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("rioosfabric");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });

  it('returns the created tyk', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta": { "name": "tyk", "account": globalAny.account_id }, "plans": [{ "object_meta": { "name": "mongo", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "mongo", "uid": globalAny.mongo_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "latest", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "mongo.png", "description": "MongoDB document databases provide high availability and easy scalability.", "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } },"ports":[{"container_port":27017,"host_ip":"","host_port":0,"protocol":""}],"stateful_volumes":[{"name":"mongo-data","volumes":{"host_path":"/data/db"},"volume_mounts":{"mount_path":"/data/db"},"settingmap":{"uri":"","uid":"","rioos_binder":[],"map_type":""}}]}, { "object_meta": { "name": "registry.rioos.xyz:5000/rioosredis", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "redis", "uid": globalAny.redis_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "3.2.11", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "redis.png", "description": "Redis is an open source key-value store that functions as a data structure server.", "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } }, "ports": [{ "container_port": 6379, "host_ip": "", "host_port": 0, "protocol": "" }],"stateful_volumes":[{"name":"redis-data","volumes":{"host_path":"/var/lib/redis/redis-data"},"volume_mounts":{"mount_path":"/var/lib/redis/redis-data"},"settingmap":{"uri":"","uid":"","rioos_binder":[],"map_type":""}},{"name":"redis-conf","volumes":{"host_path":"/etc/redis/redis.conf"},"volume_mounts":{"mount_path":"/etc/redis/redis.conf"},"settingmap":{"uri":"https://marketplaces.rioos.xyz:6443/api/v1/marketplaces/966350214048833536/settingmap/redis-conf","uid":"8574692245852336421","rioos_binder":[],"map_type":"static"}}] }, { "object_meta": { "name": "tykio/tyk-gateway", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "tyk-gateway", "uid": globalAny.tyk_gateway_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "latest", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "tyk_gateway.png", "description": "Tyk API gateway.", "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } }, "ports": [{ "container_port": 8080, "host_ip": "", "host_port": 0, "protocol": "" }], "envs": { "REDIGOCLUSTER_SHARDCOUNT": { "value": "128", "required": "true", "editable": "" } }, "stateful_volumes": [{ "name": "tyk_gateway_standalone", "volumes": { "host_path": "/var/lib/rioos/tyk-gateway/tyk.standalone.conf" }, "volume_mounts": { "mount_path": "/opt/tyk-gateway/tyk.conf" } }, { "name": "tyk_gateway", "volumes": { "host_path": "/var/lib/rioos/tyk-gateway/apps" }, "volume_mounts": { "mount_path": "/opt/tyk-gateway/apps tykio/tyk-gateway" } } ],, "stateful_volumes": [{ "name": "tyk_gateway_standalone", "volumes": { "host_path": "/var/lib/rioos/tyk-gateway/tyk.standalone.conf" }, "volume_mounts": { "mount_path": "/opt/tyk-gateway/tyk.conf" } }, { "name": "tyk_gateway", "volumes": { "host_path": "/var/lib/rioos/tyk-gateway/apps" }, "volume_mounts": { "mount_path": "/opt/tyk-gateway/apps tykio/tyk-gateway" } } ] }, { "object_meta": { "name": "tykio/tyk-dashboard", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "tyk-dashboard", "uid": globalAny.tyk_dashboard_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "latest", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "tyk_dashboard.png", "description": "Tyk API dashboard.", "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } }, "ports": [{ "container_port": 3000, "host_ip": "", "host_port": 0, "protocol": "" }], "envs": { "REDIGOCLUSTER_SHARDCOUNT": { "value": "128", "required": "true", "editable": "" } } ,"stateful_volumes":[{"name":"tyk-dashboard-conf","volumes":{"host_path":"/etc/tyk-dashboard"},"volume_mounts":{"mount_path":"/etc/tyk-dashboard/tyk_analytics.conf"},"settingmap":{"uri":"https://marketplaces.rioos.xyz:6443/api/v1/marketplaces/966350214048833536/settingmap/tyk-dashboard-conf","uid":"1010057432442880000","rioos_binder":["mongo","registry.rioos.xyz:5000/rioosredis","tykio/tyk-gateway"],"map_type":"template"}}]}, { "object_meta": { "name": "tykio/tyk-pump", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1", "name": "tyk-pump", "uid": globalAny.tyk_pump_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "latest", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "tyk_pump.png", "description": "Tyk API pump.", "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "", "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } }, "ports": [], "envs": { "REDIGOCLUSTER_SHARDCOUNT": { "value": "128", "required": "true", "editable": "" } } ,"stateful_volumes":[{"name":"tyk-pump-conf","volumes":{"host_path":"/etc/tyk-pump/pump.conf"},"volume_mounts":{"mount_path":"/etc/tyk-pump/pump.conf"},"settingmap":{"uri":"https://marketplaces.rioos.xyz:6443/api/v1/marketplaces/966350214048833536/settingmap/tyk-pump-conf","uid":"1010054499559022592","rioos_binder":["mongo","registry.rioos.xyz:5000/rioosredis"],"map_type":"template"}}]}], "category": "container", "version": "latest", "icon": "tyk.png", "description": "Tyk api management", "status": { "phase": "SyncPending" } })
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("tyk");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created neo4j', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosneo4j", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosneo4j","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"neo4j", "uid":globalAny.neo4j_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "3.3.1","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "neo4j.png","description": "Neo4j is a highly scalable, robust native graph database.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":
      {"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":7687, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "3.3.1",
       "icon": "neo4j.png", "description": "Neo4j is a highly scalable, robust native graph database.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosneo4j");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Couchdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/riooscouchdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooscouchdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"couchdb", "uid":globalAny.couchdb_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "2.1.1","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "couchdb.png",
      "description": "CouchDB is a database that uses JSON for documents, an HTTP API, & JavaScript/declarative indexing.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get":
      {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},
      "ports":[{"container_port":5984, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "2.1.1", "icon": "couchdb.png", "description": "CouchDB is a database that uses JSON for documents, an HTTP API, & JavaScript/declarative indexing.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooscouchdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Aerospike', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosaerospike", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosaerospike","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"aerospike", "uid":globalAny.aerospike_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "3.15.0.2","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "aerospike.png","description": "Aerospike is reliable, high performance, distributed database optimized for flash and RAM.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{
      "probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}},
       "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":3000, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "3.15.0.2", "icon": "aerospike.png",
       "description": "Aerospike is reliable, high performance, distributed database optimized for flash and RAM.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosaerospike");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Redis', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta": { "name": "registry.rioos.xyz:5000/rioosredis", "account": globalAny.account_id }, "plans": [{ "object_meta": { "name": "registry.rioos.xyz:5000/rioosredis", "account": globalAny.account_id, "owner_references": [{ "kind": "Package", "api_version": "v1",
       "name": "redis", "uid": globalAny.redis_package_id, "block_owner_deletion": false }] }, "category": "container", "version": "3.2.11", "characteristics": { "rioos_sh_market_image_extension": "tar.gz" }, "icon": "redis.png", "description": "Redis is an open source key-value store that functions as a data structure server.",
       "status": { "phase": "SyncPending" }, "metadata": { "origin": "rioos_system" }, "lifecycle": { "probe": { "env": {}, "exec": [], "http_get": { "host": "", "path": "", "port": "",
       "scheme": "" }, "tcp_socket": { "host": "", "port": "" }, "http_headers": {} }, "pre_stop": { "command": [] }, "post_start": { "command": [] } }, "ports": [{ "container_port": 80, "host_ip": "",
       "host_port": 0, "protocol": "" }] }], "category": "container", "version": "3.2.11", "icon": "redis.png", "description": "Redis is an open source key-value store that functions as a data structure server.", "status": { "phase": "SyncPending" } })
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosredis");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created memcached', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosmemcached", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosmemcached","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"memcached", "uid":globalAny.memcached_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "1.5.3","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "memcached.png",
      "description": "Memcached is a general-purpose distributed memory caching system. It is often used to speed up dynamic database-driven websites by caching data and objects in RAM to reduce the number of times an external data source (such as a database or API) must be read.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "",
      "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}}}], "category": "container",
      "version": "1.5.3", "icon": "memcached.png", "description": "Memcached is a general-purpose distributed memory caching system. It is often used to speed up dynamic database-driven websites by caching data and objects in RAM to reduce the number of times an external data source (such as a database or API) must be read.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosmemcached");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Postgres', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioospostgres", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioospostgres","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"postgres", "uid":globalAny.postgres_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "10.3","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "postgres.png","description": "The PostgreSQL object-relational database system provides reliability and data integrity.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":5432, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "10.3",
      "icon": "postgres.png", "description": "The PostgreSQL object-relational database system provides reliability and data integrity.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioospostgres");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Psitrax/Powerdns', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosorientdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosorientdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"orientdb", "uid":globalAny.powerdns_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "2.0.18","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "orientdb.png",
      "description": "OrientDB is the first Multi-Model Open Source NoSQL DBMS that combines the power of graphs and the flexibility of documents into one scalable, high-performance operational database. ","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},  "lifecycle":{"probe": {"env": {}, "exec": [],
      "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []},
      "post_start": {"command": []}},"ports":[{"container_port":2424, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "2.0.18", "icon": "orientdb.png", "description": "OrientDB is the first Multi-Model Open Source NoSQL DBMS that combines the power of graphs and the flexibility of documents into one scalable, high-performance operational database.",
      "status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosorientdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });


  it('returns the created Fluentbit', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosfluentbit", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosfluentbit","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"brycekahle/fluentbit", "uid":globalAny.fluentbit_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "0.12.15","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "fluentbit.png",
      "description": "Fluent Bit is an open source and multi-platform log forwarder tool which aims to be a generic Swiss knife for log collection and distribution.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get":
      {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start":
      {"command": []}},"ports":[{"container_port":5170, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "0.12.15", "icon": "fluentbit.png", "description": "Fluent Bit is an open source and multi-platform log forwarder tool which aims to be a generic Swiss knife for log collection and distribution.",
      "status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosfluentbit");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });
  it('returns the created Chronograf', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/riooschronograf", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooschronograf","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"chronograf", "uid":globalAny.chronograf_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "1.3.10.0","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "chronograf.png","description": "Chronograf is a visualization tool for time series data in InfluxDB.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":8888, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "1.3.10.0",
      "icon": "chronograf.png", "description": "Chronograf is a visualization tool for time series data in InfluxDB.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooschronograf");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Cassandra', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/riooscassandra", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooscassandra","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"cassandra", "uid":globalAny.cassandra_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "3.11.2","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "cassandra.png","description": "Apache Cassandra is an open-source distributed storage system.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
       "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":9042, "host_ip":"","host_port":0,"protocol":""},{"container_port":9142,
       "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "3.11.2", "icon": "cassandra.png", "description": "Apache Cassandra is an open-source distributed storage system.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooscassandra");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Voltdb', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/rioosvoltdb", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosvoltdb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"voltdb", "uid":globalAny.voltdb_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "7.8.2","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "voltdb.png",
      "description": "VoltDB is horizontally-scalable, in-memory SQL RDBMS designed for apps with extreme high throughput","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {}, "exec": [], "http_get":
      {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},
      "ports":[{"container_port":8080, "host_ip":"","host_port":0,"protocol":""},{"container_port":3021, "host_ip":"","host_port":0,"protocol":""},{"container_port":21211, "host_ip":"","host_port":0,"protocol":""},{"container_port":21212,
      "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "7.8.2", "icon": "voltdb.png", "description": "VoltDB is horizontally-scalable, in-memory SQL RDBMS designed for apps with extreme high throughput","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosvoltdb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });
  it('returns the created Elasticsearch', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/riooselasticsearch", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooselasticsearch","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"elasticsearch", "uid":globalAny.elasticsearch_package_id,"block_owner_deletion":false}]},
      "category": "container","version": "5.6.9","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "elasticsearch.png","description": "Elasticsearch is a powerful open source search and analytics engine that makes data easy to explore.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"lifecycle":{"probe": {"env": {},
      "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""}, "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},
      "ports":[{"container_port":9200, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "5.6.9", "icon": "elasticsearch.png", "description": "Elasticsearch is a powerful open source search and analytics engine that makes data easy to explore.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooselasticsearch");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });

  it('returns the created Kibana', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({"object_meta":{ "name":"registry.rioos.xyz:5000/riooskibana", "account":globalAny.account_id}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/riooskibana","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"kibana", "uid":globalAny.kibana_package_id,
      "block_owner_deletion":false}]},"category": "container","version": "5.6.9","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "kibana.png","description": "Kibana gives shape to any kind of data structured and unstructured indexed in Elasticsearch.","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},
      "lifecycle":{"probe": {"env": {}, "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},
      "http_headers": {}}, "pre_stop": {"command": []}, "post_start": {"command": []}},"ports":[{"container_port":5601, "host_ip":"","host_port":0,"protocol":""}]}], "category": "container", "version": "5.6.9",
      "icon": "kibana.png", "description": "Kibana gives shape to any kind of data structured and unstructured indexed in Elasticsearch.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/riooskibana");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
          done(err);
      });
  });


  it('returns the created MariaDB', function(done) {
    request.post('/marketplaces')
      .ca(globalAny.rootMarketplaceCA)
      .set('Authorization', globalAny.bobo_bearer)
      .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
      .send({ "object_meta":{ "name":"registry.rioos.xyz:5000/rioosmariadb", "account":"globalAny.account_id"}, "plans":[{"object_meta":{"name":"registry.rioos.xyz:5000/rioosmariadb","account":globalAny.account_id,"owner_references":[{"kind":"Package", "api_version":"v1","name":"mariadb",
      "uid": globalAny.mariadb_package_id,"block_owner_deletion":false}]},"category": "container","version": "10.3","characteristics" :{"rioos_sh_market_image_extension": "tar.gz"},"icon" : "mariadb.png",
      "description": "MariaDB is a community-developed fork of the MySQL relational database management system intended to remain free under the GNU GPL. ","status":{"phase":"SyncPending"},"metadata": {"origin": "rioos_system"},"envs":{"MYSQL_ROOT_PASSWORD":{"value":"team4rio","required":"true","editable":"true"}},"lifecycle":{"probe": {"env": {},
      "exec": [], "http_get": {"host": "", "path": "", "port": "", "scheme": ""}, "tcp_socket": {"host": "", "port": ""},"http_headers": {}}, "pre_stop":
      {"command": []}, "post_start": {"command": []}}}], "category": "container","version": "10.3", "icon": "mariadb.png", "description": "MariaDB is a community-developed fork of the MySQL relational database management system intended to remain free under the GNU GPL.","status":{"phase":"SyncPending"}})
      .expect(200)
      .end(function(err, res) {
        expect(res.body.object_meta.name).to.equal("registry.rioos.xyz:5000/rioosmariadb");
        expect(res.body.type_meta.kind).to.equal(globalAny.plan);
        expect(res.body.type_meta.api_version).to.equal(globalAny.version);
        done(err);
      });
  });


});
