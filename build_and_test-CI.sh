#! /bin/bash
hcdev package --output /holochain/app-spec-rust/dist/app-spec-rust.hcpkg --strip-meta
cd test
npm install
npm run build
cd ..
holoconsole /holochain/app-spec-rust/test/dist/bundle.js
