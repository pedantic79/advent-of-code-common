# Common module for Advent of Code

Use as a git subtree

## Undefined behavior checks

In a consuming crate with the `common_test` feature enabled, run all shared
utility tests under [Miri](https://github.com/rust-lang/miri):

```bash
rustup toolchain install nightly --profile minimal --component miri
cargo +nightly miri setup
cargo +nightly miri test --locked --lib --features common_test common::
```

For just the safety regressions covering numeric conversions, UTF-8 boundaries,
custom patterns, and mutable slice references, use `common::miri_tests` as the
test filter. These tests also run with ordinary `cargo test`. Miri checks the
executed paths; a passing run is not a proof that every possible input is sound.

## With remote

Setup:
```bash
git remote add -f common https://github.com/pedantic79/advent-of-code-common.git
git subtree add --prefix=src/common common main --squash
```

Update:
```bash
git subtree push --prefix=src/common common main
```

Pull:
```bash
git subtree pull --prefix=src/common common main --squash
```


## Without remote

Setup:
```bash
git subtree add --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main --squash
```

Update:
```bash
git subtree push --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main
```

Pull:
```bash
git subtree pull --prefix=src/common https://github.com/pedantic79/advent-of-code-common.git main --squash
```
