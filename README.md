# advent

[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/advent/build/master)](https://github.com/rossmacarthur/advent/actions?query=workflow%3Abuild)

My [🎄 Advent of Code](https://adventofcode.com) solutions. All in Rust of
course 🦀!

## Getting started

To run a specfic solution just use `cargo run` and pass in the binary name which
consists of the year and the day. For example, the following will run the
solution for 2020 Day 7.

```
cargo run --release --bin 202007
```

Tests can be run in a similar way.

```
cargo test --release --bin 202007
```

Benchmarks can be run by passing `--bench` to the binary.

```
cargo run --release --bin 202007 -- --bench
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
