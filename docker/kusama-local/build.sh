#!/usr/bin/env bash
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

echo 'build release version'
cargo build --release
echo 'build spec kusama-local'
./target/release/polkadot build-spec --chain kusama-local --disable-default-bootnode > kusama-local.json
cp ./target/release/polkadot ./docker/kusama-local/.
cp ./kusama-local.json ./docker/kusama-local/.

echo 'run docker'
echo 'docker-compose -f docker-compose-validator.yml up --build'

