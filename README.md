# Holochain app specification in Rust
This repository contains a Holochain app that functions as a living specification of Holochain and its HDK (Holochain Development Toolkit).

As new features, or changes to the HDK (and the API) are being designed, they will be made concrete by adding a use case to this example app and putting those changes as a pull-request to this repository. As soon as the current master branch in https://github.com/holochain/holochain-rust implements all used features, the PR gets merged here so that this repository's master branch always reflects the feature set available in Holochain's master branch.


## Build

In order to get from the source directory tree to a Holochain app bundle, several steps need to be taken which all should be automated soon by [hcdev](https://github.com/holochain/holochain-cmd).

For now, the file dist/bundle.json was created with the following steps:

* run `hcdev package --strip-meta` in the repo root (with the build of [hcdev](https://github.com/holochain/holochain-cmd) in your path)
* find bundle.json in root and be happy
* copy that file to dist and do the following steps to match the current valid DNA specification:
  * remove hidden directories like `.git`
  * remove `README.md`
