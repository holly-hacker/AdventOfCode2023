# Advent of Code 2023

This repository houses my solutions for [Advent of Code 2023](https://adventofcode.com/2023). You can run all solutions
using `cargo run`, or run a specific solution using `cargo run --no-default-features --features day01`.

All input files are embedded in the binary and there is currently no way to swap them out at runtime.

## Running test suite

Simply use `cargo test` as you would for any other rust project. To run the tests for a specific day, use
`cargo test --no-default-features --features day01`.

## Running benchmarks

These commands require the [just command runner](https://just.systems). If you don't wish to
install this, adapt the commands from `justfile`.

### Standard benchmark

```shell
just bench day01
```

### PGO-optimized benchmark

You can run benchmarks with profile-guided optimization. This doesn't seem to be beneficial for most benchmarks, though.

```shell
# once needed once
just install-pgo

just pgo day01
```
