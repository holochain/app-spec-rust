# Holochain app specification in Rust
This repository contains a Holochain app that functions as a living specification of Holochain and its HDK (Holochain Development Toolkit).

As new features, or changes to the HDK (and the API) are being designed, they will be made concrete by adding a use case to this example app and putting those changes as a pull-request to this repository. As soon as the current master branch in https://github.com/holochain/holochain-rust implements all used features, the PR gets merged here so that this repository's master branch always reflects the feature set available in Holochain's master branch.


## Build

In order to get from the source directory tree to a Holochain app bundle, several steps need to be taken which all should be automated soon by [hcdev](https://github.com/holochain/holochain-cmd).

For now, the file dist/bundle.json was created with the following steps:

* run `cargo build --target wasm32-unknown-unknown --release` in each of the rust project sub-directories (zomes/blog/capabilities/\*/code, zomes/entry_types/post/validation)
* copy the WASM binaries to where they are expected in the DNA tree:
*
  ```
  cp zomes/blog/capabilites/hc_lifecycle/code/target/wasm32-unknown-unknown/code.wasm zomes/blog/capabilities/hc_lifecycle/code

  cp zomes/blog/capabilites/main/code/target/wasm32-unknown-unknown/code.wasm zomes/blog/capabilities/main/code

  cp zomes/blog/entry_types/post/validation/target/wasm32-unknown-unknown/validation.wasm zomes/blog/entry_types/post/validation
  ```

* move those three project directories out of the repository temporarily for the next step
* run `hcdev package --strip-meta` in the repo root (with the build of [hcdev](https://github.com/holochain/holochain-cmd) in your path)
* find bundle.json in root and be happy
* copy that file to dist and do the following steps to match the current valid DNA specification:
  * remove hidden files and directories like `.DS_STORE` and `.git`
  * remove `README.md`
  * change `zomes` and `capabilities` from an object to an array and move the name of the subitems into the subitems
