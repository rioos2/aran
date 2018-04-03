import supertest = require("supertest");
import chai = require("chai");
import "mocha";
import fs = require('fs');
const path = require('path');

const globalAny:any = global;

globalAny.version = "v1"
globalAny.account = "Account";
globalAny.assemblys = "Assembly";
globalAny.assemblylist = "AssemblyList";
globalAny.plan = "PlanFactory";
globalAny.planlist = "PlanFactoryList";
globalAny.assemblyfactory = "AssemblyFactory";
globalAny.assemblyfactorylist = "AssemblyFactoryList";
globalAny.endpoint = "EndPoint";
globalAny.endpointlist = "EndPointList";
globalAny.rootCA = fs.readFileSync(path.join(process.env.RIOOS_HOME,'config/server-ca.cert.pem'));
globalAny.apiServer = 'https://localhost:7443/api/v1';
globalAny.joblist ="JobList";
globalAny.permissionlist= "PermissionList";
globalAny.rolelist= "RoleList";
