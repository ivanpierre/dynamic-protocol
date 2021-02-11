# dynamic-protocol
A proof of concept version of dynamic Object with Protocols in Rust.

## Current state
WIP in an analysis and test state, and so a working version is out of sight for now.

As I'm a newbie in Rust, I hit all the bads of borrowing, referencing, Arcs, automatic derefs, timeline problems... But it begin to enter in my fingers... ;)

The idea, is not to have an optimized structure as first draft, but a working concepts framework, easy to refactor as experiments are tested.

This part would likely to be put in an individual crate for other projects to use it, for his dynamic abilities.

This project is make with the goal in view to make a Clojure compiler in Rust. But is usable for other programms to be able to manage Rust struct dynamically

Clojure Rust project is [Here](https://github.com/ivanpierre/clojurust).

##  Opiniated way to proceed, a wishfull thinking
* Begining with documentation changes before testing or implementing code, so to be up-to-date.
* integrating TTD into modules, easing the link between tests and code.

## Goals
* Define an internal object model.

## Documentation
The whole documentation is located in the [wiki](https://github.com/ivanpierre/dynamic-protocol/wiki)

## Copyrights
### dynamic-protocol

    Copyright (c) 2020 Ivan Pierre, kilroySoft, <Ivan Pierre, ivan@kilroysoft.ch>, under MPL 2.0.

* Code is on GitHub: https://github.com/ivanpierre/dynamic-protocol

### Crate for the persistent data structures

    Copyright 2017 Bodil Stokke, under MPL 2.0.

* Crate doc is here: https://crates.io/crates/im
* Code is on GitHub: https://github.com/bodil/im-rs

## Licence
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. 

If a copy of the MPL was not distributed within this file, You can obtain one at http://mozilla.org/MPL/2.0/.

