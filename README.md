https://leetcode.com/ playground for Rust and C++.

Solutions live under a per-language folder, named after the problem in both:

- Rust: `rust/src/<mnemonic>_<nnnn>.rs`
- C++: `cpp/src/<mnemonic>_<nnnn>.cpp`

where `<nnnn>` is the LeetCode frontend problem ID.

## Rust

```sh
cargo test --manifest-path rust/Cargo.toml
cargo bench --manifest-path rust/bench/Cargo.toml
```

## C++

```sh
make -C cpp test          # build and run every solution's tests
make -C cpp test T=gcd    # only solutions whose file name contains "gcd"
make -C cpp list          # list discovered solutions
make -C cpp clean
```

Each `cpp/src/*.cpp` is a standalone translation unit carrying the solution, its
tests, and (via `cpp/include/test.hpp`) its own `main`. There is no shared
library, so files never interfere. Benchmarks are Rust-only for now.
