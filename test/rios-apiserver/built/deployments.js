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
                .send({ "name": "neurosis" })
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
            request.get('/assembly/1')
                .set('Authorization', globalAny.bobo_bearer)
                .expect(200)
                .end(function (err, res) {
                chai_1.expect(res.body).to.deep.equal(globalAny.origin_neurosis);
                done(err);
            });
        });
    });
});
