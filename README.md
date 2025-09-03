# RMDB: LMDB oxidized in Rust

The [Lightning Memory-Mapped Database][LMDB] (LMDB), translated automatically
from C to Rust using [c2rust].

## Project Status

The project is currently (26 August 2025) in active development.

## Project Goal

The goal of the project is to make LMDB safer to contribute to by refactoring it in safe (or safer) Rust. It doesn't mean that it will not contain unsafe blocks (we need one to mmap a file anyway).

By using Rust and providing good enough tests in the CI, I'll feel safer to accept contributions without introducing UBs. I would love to use miri but I doubt it supports external mmapped files.

First step is to keep the same C ABI as LMDB so that project owners can try it fairly easily. The advantage is to profit from the multiple, higher-level, test suites, i.e. Meilisearch, OpenLDAP.

Second step is to introduce improvements to the library without breaking the ABI. As a maintainer of Meilisearch, we would benefit from some improvements that I fear adding to the C codebase of LMDB.

Finally, once the project is stable enough and is equally or more performant than LMDB, I plan to make an actual Rust idiomatic interface on top of it and keep the original C ABI.

## Contributing

You must be on macOS (Sequoia 15.6) and have a working Rust toolchain.

```bash
rustup install nightly
rustup override set nightly
cargo check
```

## Important information

The commit from which we started converting LMDB to Rust is in the `lmdb.commit` file. It will be important when we will have to maintain and port the updates to RMDB. We have [a weekly scheduled CI](https://github.com/Kerollmops/rmdb/actions/workflows/track-lmdb-changes.yml) that tracks the latest commits from the LMDB repository.

## Fun facts

[Another RMDB repository][another-rmdb] dedicated to do the same thing as this one exists.
Unfortunately, it is not maintained anymore and is eight years old.
The sole difference is that it planned to use Corrode instead of c2rust.

[LMDB]: https://symas.com/lmdb/
[c2rust]: https://github.com/immunant/c2rust
[start-commit]: https://github.com/LMDB/lmdb/commit/14d6629bc8a9fe40d8a6bee1bf71c45afe7576b6
[another-rmdb]: https://github.com/oxidizers/rmdb
