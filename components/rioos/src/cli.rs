// Copyright (c) 2017 RioCorp Inc.
//

use std::path::Path;
use std::result;

use clap::{App, AppSettings, Arg};
use url::Url;

pub fn get() -> App<'static, 'static> {
    let alias_login = sub_auth_login()
        .about("Alias for 'auth login'")
        .aliases(&["log", "logi"])
        .setting(AppSettings::Hidden);
    let alias_logout = sub_auth_logout()
        .about("Alias for 'auth logout'")
        .aliases(&["logo", "logou"])
        .setting(AppSettings::Hidden);
    let alias_init = sub_cli_init()
        .about("Alias for 'cli init'")
        .aliases(&["in", "ini"])
        .setting(AppSettings::Hidden);
    let alias_list = sub_cli_list()
        .about("Alias for 'cli list'")
        .aliases(&["l", "li", "lis"])
        .setting(AppSettings::Hidden);
    let alias_deploy = sub_digicloud_deploy()
        .about("Alias for 'digitalcloud deploy'")
        .aliases(&["d", "de", "dep", "deplo"])
        .setting(AppSettings::Hidden);
    let alias_deployapp = sub_app_deploy()
        .about("Alias for 'app deploy'")
        .aliases(&["deploya", "deployap"])
        .setting(AppSettings::Hidden);

    clap_app!(rioos =>
        (about: "\"Rio/OS is the worlds first secure cloud operating sytem\"")
        (version: super::VERSION)
        (author: "\nAuthors: The Rio/OS Maintainers <humans@riocorp.io>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand cli =>
            (about: "Commands relating to Rio/OS init/setup")
            (aliases: &["cl"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_cli_init().aliases(&["i", "in", "ini"]))
            (subcommand: sub_cli_list().aliases(&["l", "li", "lis"]))
            (subcommand: sub_cli_completers().aliases(&["c", "co", "com", "comp"]))
            (subcommand: sub_cli_new().aliases(&["n", "ne", "new"]))
            (subcommand: sub_cli_whoami().aliases(&["who", "whoam", "whoami"]))

        )
        (@subcommand auth =>
            (about: "Commands relating to Rio/OS identity and access")
            (aliases: &["aut"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_auth_login().aliases(&["l", "lo", "log", "logi"]))
            (subcommand: sub_auth_logout().aliases(&["logo", "logou"]))
            (subcommand: sub_auth_listproviders().aliases(&["listp", "listpr", "listpro", "listprov","listprovi"]))
        )
        (@subcommand origin =>
            (about: "Commands relating to Rio/OS origins")
            (aliases: &["o", "or", "ori", "orig", "origi"])
            (@setting ArgRequiredElseHelp)
            (@subcommand create =>
                (about: "Creates an origin for the user")
                (aliases: &["createorigi"])
                (@arg ORG_IDENT: +required +takes_value
                    "An origin identifier (ex: riouser/myorigin1, riouser/itdevbox)")
            )
            (@subcommand get =>
                (about: "Displays the origin details for an user")
                (aliases: &["getorigi"])
                (@arg ORG_IDENT: +required +takes_value
                    "An origin identifier (ex: riouser/myorigin1, riouser/itdevbox)")
            )
            (@subcommand list =>
                (about: "Displays all the origins for an user")
                (aliases: &["listorigi"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
        )
        (@subcommand digitalcloud =>
            (about: "Commands relating to Rio/OS digital cloud os")
            (aliases: &["d", "di", "digitalcloud"])
            (@setting ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Generates a blueprint for digitalcloud os with configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    os. If `DIGICLOUD_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'digicloud_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg DIGICLOUD_NAME: +takes_value "Name for the new digitalcloud os")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new digitalcloud os")
                (@arg WITH_ALL: --("with-all")
                    "Generate a blueprint for deploying a digitalcloud with all available digitalcloud options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your digitalcloud blueprint (ex: ubuntu_16.04, centos_7.2)")
            )
            (@subcommand deploy =>
                (about: "Deploys the Rioblu.yaml blueprint in Rio/OS")
                (aliases: &["digideplo"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the rioblu.yaml")
            )
            (@subcommand list =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["l", "li","lis","list"])
                (subcommand: sub_digitalcloud_list().aliases(&["l", "li", "lis", "list"]))
            )
            (@subcommand edit =>
                (about: "Edit and update the definition of resources on the server by using default editor")
                (aliases: &["digiedi"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
                )
            (@subcommand describe =>
                (about: "Display the detailed state of digital cloud os")
                (aliases: &["digidescribe"])
                (@arg DIGICLOUD_NAME: +required +takes_value "Name for the new digitalcloud os")
            )
            (@subcommand backup =>
                (about: "Displays all the backups of digitalcloud os")
                (aliases: &["digiback", "digibacku"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand snapshot =>
                (about: "Displays all the snapshots of digitalcloud os")
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand volumes =>
                (about: "Displays all the volumes of digitalcloud os")
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand start =>
                (about: "Starts the digitalcloud os, if it can be started")
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand stop =>
                (about: "Stops the digitalcloud os, if it can be stopped")
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand reboot =>
                (about: "Reboots the digitalcloud os, if it can be rebooted")
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand ssh =>
                (about: "Securely shell connect to digitalcloud os")
                (aliases: &["digissh"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand get =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["g","ge","get"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
            (@subcommand watch =>
                (about: "Watch the logs for the deployed digital cloud os")
                (aliases: &["digiwatch"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "A digital cloud identifier (ex: 1, 2)")
            )
        )
        (@subcommand app =>
            (about: "Commands relating to Rio/OS apps and other app-specific configuration.")
            (aliases: &["ap"])
            (@setting ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["initapp"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand deploy =>
                (about: "Deploys the Rioblu.yaml blueprint in Rio/OS")
                (aliases: &["appdeplo"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the rioblu.yaml")
            )
            (@subcommand edit =>
                (about: "Edit and update the definition of resources on the server by using default editor")
                (aliases: &["appedi"])
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
                )
            (@subcommand describe =>
                (about: "Display the detailed state of digital cloud os")
                (aliases: &["appdescribe"])
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand backup =>
                (about: "Displays all the backups of app")
                (aliases: &["appback", "appbacku"])
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand snapshot =>
                (about: "Displays all the snapshots of app")
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand volumes =>
                (about: "Displays all the volumes of app")
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand start =>
                (about: "Starts the app, if it can be started")
                (@arg APP_IDENT: +required +takes_value
                    "An app cloud identifier (ex: 1, 2)")
            )
            (@subcommand stop =>
                (about: "Stops the app, if it can be stopped")
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand reboot =>
                (about: "Reboots the app, if it can be rebooted")
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand ssh =>
                (about: "Securely shell connect to app")
                (aliases: &["appssh"])
                (@arg APP_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
            (@subcommand watch =>
                (about: "Watch the logs for the deployed app")
                (aliases: &["appwatch"])
                (@arg DIGICLOUD_IDENT: +required +takes_value
                    "An app identifier (ex: 1, 2)")
            )
        )
        (@subcommand images =>
            (about: "Commands relating to Rio/OS image management")
            (aliases: &["i", "im", "ima", "imag", "image"])
            (@setting ArgRequiredElseHelp)
            (@subcommand get =>
                (about: "Displays the images details for an user")
                (aliases: &["getimag"])
                (@arg IMAGE_IDENT: +required +takes_value
                    "An image identifier (ex: 1, 2)")
            )
            (@subcommand list =>
                (about: "Displays all the images for an user")
                (aliases: &["listimag"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
        )
        (@subcommand nodes =>
            (about: "Commands relating to Rio/OS infrastructure")
            (aliases: &["n", "no", "nod","node", "nodes"])
            (@setting ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Create node for the stoarge")
                (aliases: &["i", "in", "ini"])
                (@arg NODE_NAME: +takes_value "Name for the new node")

            )
            (@subcommand healthz =>
                (about: "Commands relating to node health")
                (aliases: &["nodeheal", "nodehealth"])
                (@setting ArgRequiredElseHelp)
                (@subcommand ping =>
                    (about: "Pings a nodes")
                    (aliases: &["nodeping"])
                    (@arg NODE_IDENT: +required +takes_value
                        "A node identifier (ex: 1, 2)")
                )

            )
            (@subcommand register =>
                (about: "Manually register a node. Nodes are autodiscovered by nodelet.\
                    this is used for development testing only.")
                (aliases: &["nodereg", "noderegister"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the rioblu.yaml")
            )
            (@subcommand list =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["listnode"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
            (@subcommand describe =>
                (about: "Display the detailed state of datacenter")
                (aliases: &["nodescribe"])
                (@arg NODE_ID: +required +takes_value "Id for the node")
            )
        )
        (@subcommand storages =>
            (about: "Commands relating to Rio/OS Storage")
            (aliases: &["s", "st", "sto", "stor", "storages"])
            (@setting ArgRequiredElseHelp)
            (@subcommand list =>
                (about: "Display a detailed  state of the storages auto registered in Rio/OS.")
                (aliases: &["liststorages"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
            (@subcommand describe =>
                (about: "Display the detailed state of storage")
                (aliases: &["storedescribe"])
                (@arg STORAGE_ID: +required +takes_value "Id for the storage")
            )
        )
        (@subcommand datacenters =>
            (about: "Commands relating to Rio/OS Datacenters(Locations)")
            (aliases: &["datacent", "locations"])
            (@setting ArgRequiredElseHelp)
            (@subcommand create =>
                (about: "Create new datacenter/location in Rio/OS.")
                (aliases: &["dccreate", "loccreate"])
                (@arg APP_NAME: +takes_value "Name for the new datacenter/location")
                (@arg NODES_IDENT: +required +multiple
                    "One or more node identifiers (ex: 1, 2) to be grouped as a cluster")
                (@arg NETWORKS_IDENT: +required +multiple
                    "One or more network identifiers (ex: 1, 2) to be used by the clustered nodes")
                (@arg STORAGES_IDENT: +required +takes_value
                    "One or more storage identifiers (ex: 1, 2) to be used by the clustered nodes")
            )
            (@subcommand list =>
                (about: "Display all the datacenters/locations registered in Rio/OS.")
                (aliases: &["listdcs", "listlocations"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: nw01)")
            )
            (@subcommand get =>
                (about: "Displays the Single datacenter detail")
                (aliases: &["g","ge","get"])
                (@arg DATACENTER_ID: +required +takes_value
                    "A datacenter identifier (ex: 1, 2)")
            )
            (@subcommand describe =>
                (about: "Display the detailed state of datacenter")
                (aliases: &["datacendescribe"])
                (@arg DATACENTER_ID: +required +takes_value "Id for the Datacenter")
            )
        )
        (@subcommand jobs =>
            (about: "Commands relating to Rio/OS job management")
            (aliases: &["j", "jo", "job", "jobs"])
            (@setting ArgRequiredElseHelp)
            (@subcommand get =>
                (about: "Displays the jobs details for an user")
                (aliases: &["getjob"])
                (@arg IMAGE_IDENT: +required +takes_value
                    "An job identifier (ex: 1, 2)")
            )
            (@subcommand list =>
                (about: "Displays all the jobs for an user")
                (aliases: &["listjob"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )

        )
        (@subcommand networks =>
            (about: "Commands relating to Rio/OS job management")
            (aliases: &["net", "netw", "network", "networks"])
            (@setting ArgRequiredElseHelp)
            (@subcommand get =>
                (about: "Displays the network details for an user")
                (aliases: &["getnetwork"])
                (@arg IMAGE_IDENT: +required +takes_value
                    "An network identifier (ex: 1, 2)")
            )
            (@subcommand list =>
                (about: "Displays all the network for an user")
                (aliases: &["listnetwork"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
        )
        (subcommand: alias_login)
        (subcommand: alias_logout)
        (subcommand: alias_init)
        (subcommand: alias_deploy)
        (subcommand: alias_deployapp)
        (after_help: "\nALIASES:\
            \n    login      Alias for: 'auth login'\
            \n    logout     Alias for: 'auth logout'\
            \n    init       Alias for: 'cli init'\
            \n    list       Alias for: 'cli list'\
            \n    deploy     Alias for: 'digitialcloud deploy'\
            \n    deployapp  Alias for: 'app deploy'\
            \n    get        Alias for: 'node get'\
            \n"
        )
    )
}

fn sub_cli_init() -> App<'static, 'static> {
    clap_app!(@subcommand setup =>
        (about: "Generates a blueprint for deployment with reasonable defaults.")
    )
}

fn sub_cli_list() -> App<'static, 'static> {
    clap_app!(@subcommand setup =>
        (about: "List the blueprints deployed.")
    )
}

fn sub_cli_new() -> App<'static, 'static> {
    clap_app!(@subcommand new =>
        (about: "Create new User.")
    )
}
fn sub_cli_whoami() -> App<'static, 'static> {
    clap_app!(@subcommand whoami =>
        (about: "Dispaly the current User.")
    )
}


fn sub_cli_completers() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand completers =>
        (about: "Creates command-line completers for your shell."));

    let supported_shells = ["bash", "fish", "zsh", "powershell"];

    // The clap_app! macro above is great but does not support the ability to specify a range of
    // possible values. We wanted to fail here with an unsupported shell instead of pushing off a
    // bad value to clap.

    sub.arg(
        Arg::with_name("SHELL")
            .help(
                "The name of the shell you want to generate the command-completion. Supported \
               Shells: bash, fish, zsh, powershell",
            )
            .short("s")
            .long("shell")
            .required(true)
            .takes_value(true)
            .possible_values(&supported_shells),
    )
}

fn sub_auth_login() -> App<'static, 'static> {
    clap_app!(@subcommand login =>
        (about: "Login user to rioos.")
    )
}

fn sub_auth_logout() -> App<'static, 'static> {
    clap_app!(@subcommand logout =>
        (about: "Logout user from rioos.")
    )
}

fn sub_auth_listproviders() -> App<'static, 'static> {
    clap_app!(@subcommand list =>
        (about: "List configured authentication providers for your rioos.")
    )
}

fn sub_digitalcloud_list() -> App<'static, 'static> {
    clap_app!(@subcommand list =>
        (about: "List deployments.")
    )
}

fn sub_digicloud_deploy() -> App<'static, 'static> {
    clap_app!(@subcommand deploy =>
        (about: "Deploys the Rioblu.yaml digitalcloud os blueprint in Rio/OS")
    )
}


fn sub_app_deploy() -> App<'static, 'static> {
    clap_app!(@subcommand deploy =>
        (about: "Deploys the Rioblu.yaml app blueprint in Rio/OS")
    )
}



fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}


fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
