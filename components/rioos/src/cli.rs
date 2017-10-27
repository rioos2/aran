// Copyright (c) 2017 RioCorp Inc.
//

use std::path::Path;
use std::result;

use clap::{App, AppSettings, Arg};
use regex::Regex;
use url::Url;

pub fn get() -> App<'static, 'static> {
    let alias_login = sub_auth_login()
        .about("Alias for 'auth login'")
        .aliases(&["log", "logi"])
        .setting(AppSettings::Hidden);
    let alias_logout = sub_auth_logout()
        .about("Alias for 'auth logout'")
        .aliases(&["log", "logo", "logou"])
        .setting(AppSettings::Hidden);
    let alias_init = sub_cli_init()
        .about("Alias for 'cli init'")
        .aliases(&["in", "ini"])
        .setting(AppSettings::Hidden);
    let alias_deploy = sub_digicloud_deploy()
        .about("Alias for 'digitalcloud deploy'")
        .aliases(&["d", "de", "dep", "deplo"])
        .setting(AppSettings::Hidden);
    let alias_deployapp = sub_app_deploy()
        .about("Alias for 'app deploy'")
        .aliases(&["d", "de", "dep", "deplo"])
        .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "\"Rio/OS is the worlds first secure cloud operating sytems\"")
        (version: super::VERSION)
        (author: "\nAuthors: The Rio/OS Maintainers <humans@riocorp.io>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand cli =>
            (about: "Commands relating to Rio/OS init/setup")
            (aliases: &["cl"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_cli_init().aliases(&["i", "in", "ini"]))
            (subcommand: sub_cli_completers().aliases(&["c", "co", "com", "comp"]))
        )
        (@subcommand auth =>
            (about: "Commands relating to Rio/OS identity and access")
            (aliases: &["aut"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_auth_login().aliases(&["l", "lo", "log", "logi"]))
            (subcommand: sub_auth_logout().aliases(&["logout"]))
            (subcommand: sub_auth_listproviders().aliases(&["l", "li", "lis", "list","listp"]))
        )
        (@subcommand origin =>
            (about: "Commands relating to Rio/OS origins")
            (aliases: &["o", "or", "ori", "orig", "origi"])
            (@setting ArgRequiredElseHelp)
            (@subcommand create =>
                (about: "Creates an origin for the user")
                (aliases: &["bi", "bin", "binl", "binli", "binlin"])
                (@arg ORG_IDENT: +required +takes_value
                    "An origin identifier (ex: riouser/myorigin1, riouser/itdevbox)")
            )
            (@subcommand get =>
                (about: "Displays the origin details for an user")
                (aliases: &["conf", "cfg"])
                (@arg ORG_IDENT: +required +takes_value
                    "An origin identifier (ex: riouser/myorigin1, riouser/itdevbox)")
            )
            (@subcommand list =>
                (about: "Displays all the origins for an user")
                (aliases: &["aaaaconf", "aaacfg"])
                (@arg SEARCH_TERM: +required +takes_value "Search term (ex: riouser.*)")
            )
        )
        (@subcommand digitialcloud =>
            (about: "Commands relating to Rio/OS digital cloud os")
            (aliases: &["d", "di", "digitalcloud"])
            (@setting ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Generates common digitalcloud os specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `DIGICLOUD_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'digicloud_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg DIGICLOUD_NAME: +takes_value "Name for the new digitalcloud os")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new digitalcloud os")
                (@arg WITH_ALL: --("with-all")
                    "Generate digitalcloud blu with all available digitalcloud options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your digitalcloud os (ex: ubuntu_16.04, centos_7.2)")
            )
            (@subcommand deploy =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["conf", "cfg"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand edit =>
                (about: "Executes a command using the 'PATH' context of an installed package")
                (aliases: &["exe"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg CMD: +required +takes_value
                    "The command to execute (ex: ls)")
                (@arg ARGS: +takes_value +multiple
                    "Arguments to the command (ex: -l /tmp)")
            )
            (@subcommand set =>
                (about: "Exports the package to the specified format")
                (aliases: &["exp"])
                (@arg FORMAT: +required +takes_value
                    "The export format (ex: docker, aci, mesos, or tar)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg BLDR_URL: --url -u +takes_value {valid_url}
                    "Retrieve the container's package from the specified Builder \
                    (default: https://bldr.habitat.sh)")
                (@arg CHANNEL: --channel -c +takes_value
                    "Retrieve the container's package from the specified release channel \
                    (default: stable)")
            )
            (@subcommand describe =>
                (about: "Generates a blake2b hashsum from a target at any given filepath")
                (aliases: &["ha", "has"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the target")
            )
            (subcommand: sub_pkg_install().aliases(
                &["i", "in", "ins", "inst", "insta", "instal"]))
            (@subcommand backup =>
                (about: "Prints the path to a specific installed release of a package")
                (aliases: &["p", "pa", "pat"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand snapshot =>
                (about: "Search installed Habitat packages for a given file")
                (@arg FILE: +required +takes_value
                    "File name to find")
                (@arg FULL_RELEASES: -r
                    "Show fully qualified package names \
                    (ex: core/busybox-static/1.24.2/20160708162350)")
                (@arg FULL_PATHS: -p "Show full path to file")
            )
            (@subcommand volumes =>
                (about: "Search for a package in Builder")
                (@arg SEARCH_TERM: +required +takes_value "Search term")
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
            )
            (@subcommand start =>
                (about: "Uploads a local Habitat Artifact to Builder")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                (@arg CHANNEL: --channel -c +takes_value
                    "Additional release channel to upload package to. \
                     Packages are always uploaded to `unstable`, regardless \
                     of the value of this option. (default: none)")
                (@arg HART_FILE: +required +multiple {file_exists}
                    "One or more filepaths to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand stop =>
                (about: "Promote a package to a specified channel")
                (aliases: &["pr", "pro"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis/3.2.1/20160729052715)")
                (@arg CHANNEL: +required +takes_value
                    "Promote to the specified release channel")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand reboot =>
                (about: "Demote a package from a specified channel")
                (aliases: &["de", "dem", "demo", "demot"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg CHANNEL: +required +takes_value
                    "Demote from the specified release channel")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand ssh =>
                (about: "Find out what channels a package belongs to")
                (aliases: &["ch", "cha", "chan", "chann", "channe", "channel"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value
                    "A fully qualified package identifier (ex: core/redis/3.2.1/20160729052715)")
            )
            (@subcommand watch =>
                (about: "Verifies a Habitat Artifact with an origin key")
                (aliases: &["v", "ve", "ver", "veri", "verif"])
                (@arg SOURCE: +required {file_exists}
                    "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
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
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand deploy =>
                (about: "Create an app --config file (with datacenter/secret/horizontalscaler) in the users namespace")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand edit =>
                (about: "Pull the digital cloud and allow editing the app.yaml ? ")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand set =>
                (about: "Configuration change ? What do we allow to change ?")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand describe =>
                (about: "Full detail description")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand start =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand stop =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand reboot =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand ssh =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand watch =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand rollback =>
                (about: "Generates common app specific configuration files. Executing without \
                    argument will create a `rioos` directory in your current folder for the \
                    app. If `APP_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'app_') that are set will be used \
                    in the generated app")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            )
        (@subcommand images =>
            (about: "Commands relating to Rio/OS image management")
            (aliases: &["i", "im", "ima", "imag", "image"])
            (@setting ArgRequiredElseHelp)

        )
        (@subcommand nodes =>
            (about: "Commands relating to Rio/OS infrastructure")
            (aliases: &["n", "no", "nod","node", "nodes"])
            (@setting ArgRequiredElseHelp)
            (@subcommand healthz =>
                (about: "Commands relating to node health")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand ping =>
                    (about: "Pings a nodes")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg USER: +required +takes_value "Name of the user key")
                )
            )
            (@subcommand register =>
                (about: "Manually register a node. Nodes are autodiscovered by nodelet.\
                    this is used for development testing only.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand list =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
        )
        (@subcommand storages =>
            (about: "Commands relating to Rio/OS Storage")
            (aliases: &["i", "im", "ima", "imag", "image"])
            (@setting ArgRequiredElseHelp)
            (@subcommand list =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand create =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )

        )
        (@subcommand datacenters =>
            (about: "Commands relating to Rio/OS Datacenters")
            (aliases: &["i", "im", "ima", "imag", "image"])
            (@setting ArgRequiredElseHelp)
            (@subcommand list =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand create =>
                (about: "Display all the nodes registered in Rio/OS.")
                (aliases: &["i", "in", "ini"])
                (@arg APP_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_ALL: --("with-all")
                    "Generate app blu with all available app options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )

        )
        (subcommand: alias_login)
        (subcommand: alias_logout)
        (subcommand: alias_init)
        (subcommand: alias_deploy)
        (after_help: "\nALIASES:\
            \n    login      Alias for: 'auth login'\
            \n    logout     Alias for: 'auth logout'\
            \n    init       Alias for: 'cli init'\
            \n    deploy     Alias for: 'digitialcloud deploy'\
            \n    deployapp  Alias for: 'app deploy'\
            \n"
        )
    )
}

fn alias_run() -> App<'static, 'static> {
    clap_app!(@subcommand run =>
        (about: "Run the Habitat Supervisor")
        (@setting Hidden)
    )
}

fn alias_start() -> App<'static, 'static> {
    clap_app!(@subcommand start =>
        (about: "Starts a Habitat-supervised service")
        (aliases: &["sta", "star"])
        (@setting Hidden)
    )
}

fn alias_stop() -> App<'static, 'static> {
    clap_app!(@subcommand stop =>
        (about: "Stop a running Habitat service.")
        (aliases: &["sto"])
        (@setting Hidden)
    )
}

fn alias_term() -> App<'static, 'static> {
    clap_app!(@subcommand term =>
        (about: "Gracefully terminate the Habitat Supervisor and all of it's running services")
        (@setting Hidden)
    )
}

fn sub_cli_init() -> App<'static, 'static> {
    clap_app!(@subcommand setup =>
        (about: "Sets up the CLI with reasonable defaults.")
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
    clap_app!(@subcommand logout =>
        (about: "Logout user from rioos.")
    )
}

fn sub_digicloud_deploy() -> App<'static, 'static> {
    clap_app!(@subcommand logout =>
        (about: "Logout user from rioos.")
    )
}


fn sub_app_deploy() -> App<'static, 'static> {
    clap_app!(@subcommand logout =>
        (about: "Logout user from rioos.")
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

fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
        (about: "Applies a configuration to a group of Habitat Supervisors")
        (@arg PEER: -p --peer +takes_value
            "A comma-delimited list of one or more Habitat Supervisor peers to infect \
            (default: 127.0.0.1:9638)")
        (@arg RING: -r --ring +takes_value
            "Ring key name, which will encrypt communication messages")
        (@arg SERVICE_GROUP: +required {valid_service_group}
            "Target service group (ex: redis.default)")
        (@arg VERSION_NUMBER: +required
            "A version number (positive integer) for this configuration (ex: 42)")
        (@arg FILE: {file_exists_or_stdin}
            "Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)")
        (@arg ORG: --org +takes_value "Name of service organization")
    )
}

fn sub_pkg_build() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand build =>
        (about: "Builds a Plan using a Studio")
        (aliases: &["bu", "bui", "buil"])
        (@arg HAB_ORIGIN_KEYS: -k --keys +takes_value
            "Installs secret origin keys (ex: \"unicorn\", \"acme,other,acme-ops\")")
        (@arg HAB_STUDIO_ROOT: -r --root +takes_value
            "Sets the Studio root (default: /hab/studios/<DIR_NAME>)")
        (@arg SRC_PATH: -s --src +takes_value
            "Sets the source path (default: $PWD)")
        (@arg PLAN_CONTEXT: +required +takes_value
            "A directory containing a `plan.sh` file \
            or a `habitat/` directory which contains the `plan.sh` file")
    );
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    if cfg!(target_os = "linux") {
        sub.arg(
            Arg::with_name("REUSE")
                .help(
                    "Reuses a previous Studio for the build (default: clean up before building)",
                )
                .short("R")
                .long("reuse"),
        ).arg(
                Arg::with_name("DOCKER")
                    .help(
                        "Uses a Dockerized Studio for the build (default: Studio uses a chroot on \
                        linux)",
                    )
                    .short("D")
                    .long("docker"),
            )
    } else if cfg!(target_os = "windows") {
        sub.arg(
            Arg::with_name("WINDOWS")
                .help("Use a Windows Studio instead of a Docker Studio")
                .short("w")
                .long("windows"),
        )
    } else {
        sub
    }
}

fn sub_pkg_install() -> App<'static, 'static> {
    clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from Builder or locally from a Habitat Artifact")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Specify an alternate Builder endpoint (default: https://bldr.habitat.sh)")
        (@arg CHANNEL: --channel -c +takes_value
            "Install from the specified release channel (default: stable)")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths \
            to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg BINLINK: -b --binlink "Binlink all binaries from installed package(s)")
        (@arg FORCE: -f --force "Overwrite existing binlinks")
    )
}

fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}

fn file_exists_or_stdin(val: String) -> result::Result<(), String> {
    if val == "-" { Ok(()) } else { file_exists(val) }
}


fn valid_service_group(val: String) -> result::Result<(), String> {
    let regex = Regex::new(r"([A-Za-z_0-9]+)\.([A-Za-z_0-9]+)").unwrap();
    if regex.is_match(&val) {
        Ok(())
    } else {
        Err(format!("SERVICE_GROUP: '{}' is invalid", &val))
    }
}

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
