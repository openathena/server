Athena
====
[![Build Status](https://img.shields.io/badge/build-unknown-lightgrey.svg)](https://travis-ci.org/openathena/server)
## Documentation

http://openathena.github.io/docs/server (coming soon...)

## Installation
 - Install rust, if not already installed: `curl https://sh.rustup.rs -sSf | sh`
 - Inside this directory, use _nightly_ rust `rustup override set nightly`
 - Run the server with `cargo run --release`
 
 One of the dependencies, `rocket` requires rust _nightly_. As a result, occasional breaking changes may occur.