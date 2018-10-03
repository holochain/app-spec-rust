# Holochain app specification in Rust

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![PM](https://img.shields.io/badge/pm-waffle-blue.svg?style=flat-square)](https://waffle.io/holochain/holochain-rust)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)

This repository contains a Holochain app that functions as a living specification of Holochain and its [HDK](https://github.com/holochain/hdk-rust) (Holochain Development Toolkit).

As new features, or changes to the HDK (and the API) are being designed, they will be made concrete by adding a use case to this example app and putting those changes as a pull-request to this repository. As soon as the current master branch in https://github.com/holochain/holochain-rust implements all used features, the PR gets merged here so that this repository's master branch always reflects the feature set available in Holochain's master branch.

Please see the [Contribute section](https://github.com/holochain/holochain-rust/blob/develop/README.md#app-spec-driven-development) for our protocol on how we do this.

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

## Contribute
Holochain is an open source project.  We welcome all sorts of participation and are actively working on increasing surface area to accept it.  Please see our [contributing guidelines](https://github.com/holochain/org/blob/master/CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

Copyright (C) 2018, Holochain Trust

This program is free software: you can redistribute it and/or modify it under the terms of the license p
rovided in the LICENSE file (GPLv3).  This program is distributed in the hope that it will be useful, bu
t WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 PURPOSE.

**Note:** We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.  See [this article](https://medium.com/holochain/licensing-needs-for-truly-p2p-software-a3e0fa42be6c) for some of our thinking on licensing for distributed application frameworks.
