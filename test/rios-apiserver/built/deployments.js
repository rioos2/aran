"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var chai_1 = require("chai");
var supertest = require("supertest");
var request = supertest('http://localhost:9636/v1');
var globalAny = global;
describe('Deployment API', function () {
    describe('Create assembly neurosis', function () {
        it('returns the created assembly', function (done) {
            request.post('/assemblys')
                .set('Authorization', globalAny.bobo_bearer)
                .send({"name": "neurosis","uri":"/v1/assemblys","description":"ubuntuinstallation","tags": "","representation_skew":" ","external_management_resource":"","metadata":"" ,"component_collection": "","operation_collection": "","sensor_collection": "","plan": "" })
                .expect(201)
                .end(function (err, res) {
                chai_1.expect(res.body.name).to.equal("neurosis");
                globalAny.origin_neurosis = res.body;
                done(err);
            });
        });
    });
    describe('Get assembly neurosis', function () {
        it('returns the assembly', function (done) {
            request.get('/assemblys/1')
                .set('Authorization', globalAny.bobo_bearer)
                .expect(200)
                .end(function (err, res) {
                chai_1.expect(res.body).to.deep.equal(globalAny.origin_neurosis);
                done(err);
            });
        });
    });
});
