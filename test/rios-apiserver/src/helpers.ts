import supertest = require("supertest");
import chai = require("chai");
import "mocha";

const globalAny: any = global;

// Users we can authenticate as
globalAny.bobo_bearer = "Bearer ydukl6BhNeJi5V6pT5";
globalAny.email = "vino";
globalAny.logan_bearer = "Bearer logan";
