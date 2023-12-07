# Advent of Code 2023 in Rust

I'm trying the [advent of code](https://adventofcode.com/2023) in Rust this year to learn Rust.

## Formatting and linting

Use:

```bash
cargo fmt
cargo clippy
```

## Tests

Use:

```bash
cargo test
```

## Running

Download the input files to `input/<number>.txt`. For example, `input/01.txt`.

Use:

```bash
cargo run -r --bin 01
```

(`-r` for release mode highly recommended for `05` and `06`!)


## Notes

This is mostly one file per project. I'm not worrying about visibility or nice
error handling since these are one-shot "scripts". I even played with the script
feature in the nightlies (`01` supports it), but I wanted `cargo fmt` and
`cargo clippy` (and then `cargo test`), so I went with the classic project-based
approach.

Features used in each vary. For example, `05` has an optional progress bar
(opt-out).