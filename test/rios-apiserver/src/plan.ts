import { expect } from 'chai';
import supertest = require('supertest');

const request = supertest('http://localhost:9636/api/v1');
const globalAny:any = global;

describe('Plan Factory API', function() {
  describe('plan factory creation API', function() {
    it('returns the created plan factory', function(done) {
      request.post('/planfactory')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .send({"group_name":"2_application_python","url": "/v3/plan/java","description": "The Apache Tomcat® software is an open source implementation of the Java Servlet, JavaServer Pages, Java Expression Language and Java WebSocket technologies.","tags": ["tomcat","java","jdk"],"origin":"rioos:2.0","artifacts":[],"services":[{ "name":"tomcat","description":"","href":"http://tomcat.apache.org/","characteristics":{}}]})
        .expect(200)
        .end(function(err, res) {
         expect(res.body.group_name).to.equal("2_application_python");
          done(err);
        });
    });

    it('returns all plan factory', function(done) {
      request.get('/plans')
        .set('Authorization', globalAny.bobo_bearer)
        .set('X-AUTH-RIOOS-EMAIL',globalAny.email)
        .expect(200)
        .end(function(err, res) {
          done(err);
        });
    });

  });

  });
