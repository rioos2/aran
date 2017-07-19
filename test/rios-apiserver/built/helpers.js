"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("mocha");
var globalAny = global;
// Users we can authenticate as
globalAny.bobo_bearer = "Bearer bobo";
globalAny.logan_bearer = "Bearer logan";
