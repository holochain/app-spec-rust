#! /bin/bash
hcdev package --output dist/app-spec-rust.hcpkg --strip-meta
cd test
npm install
npm run build
cd ..
holoconsole test/dist/bundle.js
