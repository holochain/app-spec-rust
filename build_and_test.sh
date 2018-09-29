#! /bin/bash
mkdir dist
echo "===================================================================================="
echo "BUILDING genome with 'hcdev package --output dist/app-spec-rust.hcpkg --strip-meta':"
echo "------------------------------------------------------------------------------------"
hcdev package --output dist/app-spec-rust.hcpkg --strip-meta
echo "DONE."
echo "===================================================================================="
echo "BUILDING JS test file with webpack:"
echo "------------------------------------------------------------------------------------"
cd test
npm install
npm run build
cd ..
echo "DONE."
echo "RUNNING tests with holoconsole..."
echo "------------------------------------------------------------------------------------"
holoconsole test/dist/bundle.js
echo "DONE"
echo "==================================================================================="
