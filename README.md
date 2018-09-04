# Holochain app specification in Rust
This repository contains a Holochain app that functions as a living specification of Holochain and its HDK (Holochain Development Toolkit).

As new features, or changes to the HDK (and the API) are being designed, they will be made concrete by adding a use case to this example app and putting those changes as a pull-request to this repository. As soon as the current master branch in https://github.com/holochain/holochain-rust implements all used features, the PR gets merged here so that this repository's master branch always reflects the feature set available in Holochain's master branch.


## Build

### tl;dr:

Just run:

```
./build_and_test.sh
```

### Dependencies

You will need both `hcdev` and `holoconsole` build and in your $PATH.
* https://github.com/holochain/holochain-cmd
* https://github.com/holochain/holosqape

### Detailed description

In order to get from the source directory tree to a Holochain app bundle, several steps need to be taken which are all automated by [hcdev](https://github.com/holochain/holochain-cmd).

This includes compiling any Rust code projects to WASM, and then assembling a DNA file (.hcpkg) with all configuration snippets and the WASM in it.

`hcdev` does this when called like so:

```
hcdev package
```

In `build_and_test.sh` we are calling it with

```
hcdev package --output dist/app-spec-rust.hcpkg --strip-meta
```

to create a DNA file without the meta information that would be needed to recreate the same directory structure from the file. Currently Holochain would not accept that DNA file as valid, but we might wanna change that (or change the default behaviour of hcdev).
