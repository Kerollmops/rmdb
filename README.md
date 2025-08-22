# RMDB: LMDB oxidized in Rust

The [Lightning Memory-Mapped Database][LMDB] (LMDB), translated automatically
from C to Rust using [c2rust].

## Contributing

You must be on Linux (I use Debian) and have a working Rust toolchain.

```bash
rustup install nightly
rustup override set nightly
cargo check
```

## Important information

We started converting LMDB to Rust from [14d6629bc8a9fe40d8a6bee1bf71c45afe7576b6][start-commit].
It will be important when we will have to maintain and port the updates to RMDB. I would like to add a bot that creates issues for each new commit upstream.

## Fun facts

[Another RMDB repository][another-rmdb] dedicated to do the same thing as this one exists.
Unfortunately, it is not maintained anymore and is eight years old.
The sole difference is that it planned to use Corrode instead of c2rust.

[LMDB]: https://symas.com/lmdb/
[c2rust]: https://github.com/immunant/c2rust
[start-commit]: https://github.com/LMDB/lmdb/commit/14d6629bc8a9fe40d8a6bee1bf71c45afe7576b6
[another-rmdb]: https://github.com/oxidizers/rmdb
