# Building a rioos Mac Binary

As Habitat currently does not have first class support for the Mac platform, a pragmatic approach has been taken to build a `rioos` binary for Mac OS X. A wrapper script called `mac-build.sh` attempts to install any missing pre-requisites. Currently, the following are required on the Mac system performing the build:

* Xcode CLI tools
* Homebrew
* `coreutils`, `gnu-tar`, and `wget` Homebrew packages
* Rust
* Cargo nightly

The `mac-build.sh` will install the latest release from Bintray if `/bin/rioos` cannot be found locally, otherwise the `rioos` on your `PATH` will be used.

## Usage

```sh
cd components/rioos/mac
sudo ./mac-build.sh
```

Assuming success, this will produce a local `./results` directory with the artifact.

Alternatively, as `mac-build.sh` is a wrapper around the build program, it can be just as easily invoked from the root of the source tree with:

```sh
sudo ./components/rioos/mac/mac-build.sh components/rioos/mac
```
