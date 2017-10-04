Athena
====
[![Build Status](https://travis-ci.org/openathena/server.svg?branch=master)](https://travis-ci.org/openathena/server)
## Documentation

http://openathena.github.io/docs/server (coming soon...)

## Installation
 - Install rust, if not already installed: `curl https://sh.rustup.rs -sSf | sh`
 - Inside this directory, use _nightly_ rust `rustup override set nightly`
 
 One of the dependencies, `rocket` requires rust _nightly_. As a result, occasional breaking changes may occur.
 
## Running the server

#### Development
`cargo run`

#### Production
`ROCKET_ENV=production cargo run --release`