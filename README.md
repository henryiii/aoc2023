# Advent of Code 2023 in Rust

I'm trying the [advent of code](https://adventofcode.com/2023) in Rust this
year to learn Rust. I'm not trying to be fast and place on the leaderboards
(which also require working at midnight, which I'm mostly not interested in
doing), I'm trying to be somewhat elegant and learn new things in Rust.

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
(opt-out). Over time, I've been cleaning up the older problems based on what
I've learned in newer problems, so looking at the history for a file might be
instructive.  I started using external crates like `itertools` & `derive_more`
around 10-12 or so, but backported a lot of the cleanups later.

A few of the crates I'm using or have used:

- `cached`: Python's `itertools.cache` basically
- `derive-new`: Powerful `new` creation (supports default, unlike `derive_more`'s `Constructor`.
- `derive_more`: Adds useful derives not part of the stdlib (like Add)
- `grid`: A simple 2D array library
- `indicatif`: Progress bars
- `log`, `env_logger`, `test-log`: logging facilities
- `num`: Needed `lcm` in a problem.
- `rayon` (not actively used): Easy multithreading
- `strum`: Powerful enum tools like conversion with strings & iteration over enums

I added some documentation to 13 to try `cargo doc`.
