# Advent of Code 2023 in Rust

I'm trying the [advent of code](https://adventofcode.com/2023) in Rust this
year to learn Rust. I'm not trying to be fast and place on the leaderboards
(which also require working at midnight, which I'm mostly not interested in
doing), I'm trying to be somewhat elegant and learn new things in Rust. The
documentation is [live here](https://henryiii.github.io/aoc2023).

I highly recommend loading this up in a good editor, like Visual Studio Code or
VIM with the ALE plugin. It will add type information to all inferred types,
autocomplete, show documentation, etc.

## Formatting and linting

Use:

```bash
cargo fmt
cargo clippy --all
```

You should have Rust 1.74+ to use the `Cargo.toml` config for clippy. If you want
to auto-fix anything, you can:

```bash
cargo clippy --fix --allow-dirty --allow-staged
```

I also looked for removable features using
[unused-features](https://crates.io/crates/cargo-unused-features), both to
speed up compilation and it helped removed a small dependence on unicode in
regex.

## Tests

Use:

```bash
cargo test
```

Useful flags include `-- --nocapture` and `--bin <NUMBER>` for just one set of tests.

If you have `cargo-nextest` (say, from `brew install cargo-nextest`), then
`cargo nextest run` also works.

## Running

Download the input files to `input/<number>.txt`. For example, `input/01.txt`.

Use:

```bash
cargo run -r --bin 01
```

(`-r` for release mode highly recommended for some problems, like `05`!)


## Docs

You can build with:

```bash
cargo docs --no-deps
```

## Notes

This is mostly one file per project, with a few shared helpers on a small
number of days. I'm not worrying about visibility or nice error handling since
these are one-shot "scripts". I even played with the script feature in the
nightlies (`01` supports it), but I wanted `cargo fmt` and `cargo clippy` (and
then `cargo test`), so I went with the classic project-based approach.

Features used in each vary. For example, `05` has an optional progress bar
(opt-out). Over time, I've been cleaning up the older problems based on what
I've learned in newer problems, so looking at the history for a file might be
instructive.  I started using external crates like `itertools` & `derive_more`
around 10-12 or so, but backported a lot of the cleanups later. I added a few
tools for several problems in the aoc2023 crate, but only a small handful use
it.

A few of the crates I'm using or have used:

- `cached`: Python's `itertools.cache` basically
- `derive-new`: Powerful `new` creation (supports default, unlike `derive_more`'s `Constructor`).
- `derive_more`: Adds useful derives not part of the stdlib (like `Add`)
- `grid`: A simple 2D array library
- `indexmap`: Ordered map
- `indicatif`: Progress bars
- `intervalium`/`gcollections`: `IntervalSet`
- `log`, `env_logger`, `test-log`: logging facilities
- `num`: Needed `lcm` in a problem.
- `pest`/`pest_derive`: A PEG parser
- `petgraph` / `rustworkx-core`: Graph tools, similar to networkx for Python
- `rayon` (not actively used): Easy multithreading
- `regex`: Input parsing via regular expressions
- `strum`: Powerful enum tools like conversion with strings & iteration over enums

Also see [Blessed.rs](https://blessed.rs), a curated list of good Rust libraries.

I added fairly extensive docs to `13` to try `cargo doc`. Other days have some intro text,
but little in the way of inline docs.

More links:

- [Solution megathreads](https://www.reddit.com/r/adventofcode/search?q=flair_name%3A%22SOLUTION%20MEGATHREAD%22&restrict_sr=1)
- [Rust fast solutions](https://github.com/kcaffrey/aoc2023) [(Reddit)](https://www.reddit.com/r/adventofcode/s/FqpybvRYhk)
