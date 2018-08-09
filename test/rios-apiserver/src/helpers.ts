import supertest = require("supertest");
import chai = require("chai");
import "mocha";
import fs = require('fs');
const path = require('path');

const globalAny:any = global;

globalAny.version = "v1"
globalAny.user_agent= "Rio/OS Aran";
globalAny.account = "Account";
globalAny.package = "Package";
globalAny.marketplace = "Marketplace";
globalAny.assemblys = "Assembly";
globalAny.assemblylist = "AssemblyList";
globalAny.plan = "PlanFactory";
globalAny.planlist = "PlanFactoryList";
globalAny.assemblyfactory = "AssemblyFactory";
globalAny.assemblyfactorylist = "AssemblyFactoryList";
globalAny.endpoint = "Endpoints";
globalAny.endpointlist = "EndPointList";
globalAny.rootCA = fs.readFileSync(path.join(process.env.RIOOS_HOME,'config/server-ca.cert.pem'));
globalAny.rootMarketplaceCA = fs.readFileSync(path.join(process.env.RIOOS_HOME,'config/client-appstores.cert.pem'));
globalAny.apiServer = 'https://localhost:7443/api/v1';
globalAny.marketplaceServer = 'https://appstores.rioos.xyz:6443/api/v1';
globalAny.joblist ="JobList";
globalAny.Licenses ="LicenseList";
globalAny.License ="License";
globalAny.permissionlist= "PermissionList";
globalAny.teamlist= "TeamList";
globalAny.pending= "Pending";
globalAny.ingress= "Ingress";
globalAny.stacksfactorys = "StacksFactory";
globalAny.stacksfactoryslist = "StacksFactoryList";
