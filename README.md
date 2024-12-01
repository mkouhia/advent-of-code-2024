# Advent of Code in Rust

## Develop

### New puzzle

To create a base solution for a new day add a new row to `Cargo.toml` in the
`members` array, and then run

    cargo new <solution>

where `solution` is the module name.

Copy contents of `template.rs` into `<solution>/src/main.rs`. From the Advent
of Code site, copy personal puzzle input to `<solution>/src/input`.

### Tests

Go to folder `<solution>` and then run

    cargo test

## Run solution

Go to folder `<solution>` and then run

    cargo run < input
