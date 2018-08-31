// Copyright 2018 The Rio Advancement Inc
//

use std::path::Path;
use std::result;

use clap::{App, AppSettings};

pub fn get() -> App<'static, 'static> {
    let alias_login = sub_auth_login()
        .about("Alias for 'auth login'")
        .aliases(&["log", "logi"])
        .setting(AppSettings::Hidden);
    let alias_logout = sub_auth_logout()
        .about("Alias for 'auth logout'")
        .aliases(&["logo", "logou"])
        .setting(AppSettings::Hidden);
    let alias_new = sub_cli_new()
        .about("Alias for 'cli new'")
        .aliases(&["n", "ne", "new"])
        .setting(AppSettings::Hidden);
    let alias_list = sub_cli_list()
        .about("Alias for 'cli list'")
        .aliases(&["l", "li", "lis", "list"])
        .setting(AppSettings::Hidden);
    let alias_init = sub_cli_init()
        .about("Alias for 'digitalcloud deploy'")
        .aliases(&["d", "de", "dep", "deplo"])
        .setting(AppSettings::Hidden);
    let alias_whoami = sub_cli_whoami()
        .about("Alias for 'cli whoami'")
        .aliases(&["who", "whoam", "whoami"])
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
            (subcommand: sub_cli_init().aliases(&["i", "in", "int"]))
            (subcommand: sub_cli_list().aliases(&["l", "li", "lis"]))
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
        (@subcommand digitalcloud =>
            (about: "Commands relating to Rio/OS digital cloud os")
            (aliases: &["d", "di", "digitalcloud"])
            (@setting ArgRequiredElseHelp)
            (@subcommand deploy =>
                (about: "Deploys the Rioblu.yaml blueprint in Rio/OS")
                (aliases: &["digideplo"])
                (@arg SOURCE: +required +takes_value {file_exists} "A filepath of the riobluhscale.yaml")
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
        (@subcommand cluster =>
            (about: "Commands relating to Rio/OS cluster Setup")
            (aliases: &["clu"])
            (@setting ArgRequiredElseHelp)
            (@subcommand setup =>
                (about: "Setup the Cluster from file in Rio/OS")
                (aliases: &["digisetup"])
                (@arg SOURCE: +required +takes_value {file_exists} "A filepath of the cluster.yaml")
            )
        )
        (@subcommand secret =>
            (about: "Commands relating to Rio/OS Security")
            (aliases: &["sec"])
            (@setting ArgRequiredElseHelp)
            (@subcommand create =>
                (about: "Create the Secret from file in Rio/OS")
                (aliases: &["seccreate"])
                (@arg SOURCE: +required +takes_value {file_exists} "A filepath of the secret.yaml")
            )
            (@subcommand list =>
                (about: "Displays all the secrets ")
                (aliases: &["listsecret"])
                (@arg SEARCH_TERM: +takes_value "Search term (ex: riouser.*)")
            )
            (@subcommand describe =>
                (about: "Display the detailed state of secret")
                (aliases: &["secretdescribe"])
                (@arg SECRET_ID: +required +takes_value "Id for the Secret")
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
                (about: "Display the detailed state of node")
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
            (@subcommand edit =>
                (about: "Edit the Cluster Datacenter from file in Rio/OS")
                (aliases: &["digieditdc"])
                (@arg DATACENTER_IDENT: +required +takes_value"A digital cloud datacenter identifier (ex: 1, 2)")
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
            (@subcommand edit =>
                (about: "Edit the Cluster Network from file in Rio/OS")
                (aliases: &["digieditnet"])
                (@arg NETWORK_IDENT: +required +takes_value "A digital cloud network identifier (ex: 1, 2)")
            )
        )
        (subcommand: alias_new)
        (subcommand: alias_whoami)
        (subcommand: alias_login)
        (subcommand: alias_logout)
        (subcommand: alias_list)
        (subcommand: alias_init)
        (after_help: "\nALIASES:\
            \n    new        Alias for: 'cli new'\
            \n    whoami     Alias for: 'cli whoami'\
            \n    login      Alias for: 'auth login'\
            \n    logout     Alias for: 'auth logout'\
            \n    init       Alias for: 'cli init'\
            \n    list       Alias for: 'cli list'\
            \n"
        )
    )
}

fn sub_cli_list() -> App<'static, 'static> {
    clap_app!(@subcommand list =>
        (about: "List the blueprints deployed.")
    )
}

fn sub_cli_init() -> App<'static, 'static> {
    clap_app!(@subcommand init =>
        (about: "Deploys the Rioblu.yaml digitalcloud os blueprint in Rio/OS")
        (@arg SOURCE: +required +takes_value {file_exists} "A filepath of the riobluhscale.yaml")
    )
}

fn sub_cli_new() -> App<'static, 'static> {
    clap_app!(@subcommand new =>
        (about: "Signup new User.")
    )
}
fn sub_cli_whoami() -> App<'static, 'static> {
    clap_app!(@subcommand whoami =>
        (about: "Dispaly the current User.")
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

fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}
