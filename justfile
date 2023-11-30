# Lists all Just recipes
default:
    @just --list

# Runs clippy
check feature='default':
    cargo clippy --benches --tests --no-default-features
    cargo clippy --benches --tests  --no-default-features --features {{feature}}

# Runs clippy using pedantic options
check-pedantic feature='default':
    cargo clippy --benches --tests --no-default-features -- --warn clippy::nursery --warn clippy::pedantic
    cargo clippy --benches --tests  --no-default-features --features {{feature}} -- --warn clippy::nursery --warn clippy::pedantic

# Benchmarks a day using Criterion
bench feature='default':
    cargo test --no-default-features --features {{feature}} -q
    cargo bench --bench criterion --no-default-features --features {{feature}}

# Installs the tooling required for pgo
install-pgo:
    rustup component add llvm-tools-preview
    cargo install cargo-pgo

# Benchmarks a day using Criterion with PGO enabled
pgo feature='default':
    cargo pgo bench -- --bench criterion --no-default-features --features {{feature}} -- "real"
    cargo pgo optimize bench -- --bench criterion --no-default-features --features {{feature}} -- "real"

# Creates a build that can be profiled using an external profiler
build-profile feature='default':
    cargo build --profile profile --no-default-features --features {{feature}},profile
