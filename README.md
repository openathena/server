Athena
====
[![Build Status](https://travis-ci.org/openathena/server.svg?branch=master)](https://travis-ci.org/openathena/server)
## Documentation
https://github.com/openathena/server/wiki


## Installation
 - Install rust, if not already installed: `curl https://sh.rustup.rs -sSf | sh`
 - Inside this directory, use _nightly_ rust `rustup override set nightly`
 
 One of the dependencies, `rocket` requires rust _nightly_. As a result, occasional breaking changes may occur.
 
## Running the server

#### Development
`cargo run`

#### Production
`ROCKET_ENV=production cargo run --release`

## Troubleshooting
If the code won't compile or the server won't start, try updating rust and all the dependencies.
Since we are running on rust nightly this will occasionally be required.
 - `rustup update` (update rust)
 - `cargo update` updates Cargo.lock with the latest dependencies from Cargo.toml
 
 If any dependencies were updated (Cargo.lock file changed) please open a PR with the updates
 
