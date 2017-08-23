# Building Rio/OS Aran API from source

## Mac OS X for Linux Development

These install instructions assume you want to develop, build, and run the
various Rio/OS Aran API software components in a Linux environment.

1. Checkout the source by running `git clone git@gitlab.com.com:rioos/aran.git; cd aran`
1. Run `make` to compile all Rust software components (this will take a while)
1. (Optional) Run `make test` if you want to run the tests. This will take a while.
1. [postgresql](https://postgresql.com)

Everything should come up green. Congratulations - you have a working Rio/OS Aran API
development environment.

**Note:** The Makefile targets are documented. Run `make help` to show the
output (this target requires `perl`).

**Optional:**

To [install stable Rust](https://www.rust-lang.org/install.html), run: `curl -sSf
https://sh.rustup.rs | sh`. Additionally, the project maintainers use
[rustfmt](https://github.com/rust-lang-nursery/rustfmt) for code formatting. If
you are submitting changes, please ensure that your work has been run through
the latest version of rustfmt. An easy way to install it (assuming you have
Rust installed as above), is to run `cargo install rustfmt` and adding
`$HOME/.cargo/bin` to your `PATH`.

## Ubuntu: Latest (16.10/Yakkety)

This installation method uses as many packages from Ubuntu as possible.

First clone the codebase and enter the directory:

```
git clone https://gitlab.com/rioos/aran.git
cd aran
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/aran/install.sh /tmp/
sh support/linux/install_dev_0_ubuntu_latest.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make

```

These docs were tested with Ubuntu 16.04.

## Arch Linux

First clone the codebase and enter the directory:

```
git clone https://gitlab.com/rioos/aran.git
cd aran
```

Then, run the system preparation scripts and try to compile the project:

```
cp components/hab/install.sh /tmp/
sh support/linux/install_dev_0_arch.sh
sh support/linux/install_dev_9_linux.sh
. ~/.profile
make
```

## General build notes

- Once make has finished, executables will exist in `/src/target/debug/foo`,
  where `foo` is the name of an executable (`rioos-api-server`).
- Executable names are specified in each components `Cargo.toml` file in a TOML
  table like this:

	  [[bin]]
	  name = "rios-api-server"


# Building and running individual components

When you are working on an individual component in the /components directory, you may wish to build, install, then use that individual component.

Let's say you want to do this with the supervisor (which lives in the components/sup directory).

## Building

Change directories into the component you want to build

```
cd components/builder-authorize
```

Then run

```
cargo build
```

Once it is finished compiling, you can find the new build in root aran_repo/target/debug

Head back to the root of the Aran repo

And you will find your build in target/debug

If you built the `builder-authorize` component, this is where you would find the new build

```
target/debug/builder-authorize
```

## Running

You can now run this newly built component with

```
./target/debug/rios-api-server
```
