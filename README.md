Example for using kcov with Rust
================================

First install the Rust toolchain from rust-lang.org.
Then download and compile kcov (https://github.com/SimonKagstrom/kcov).

Now compile this Rust library and her tests with `cargo test --no-run`.
Run both test executables with kcov:

```bash
kcov target/kcov target/debug/integration-*
kcov target/kcov target/debug/kcov_example-*
```

Coverage results will be placed in target/kcov/kcov-merged. The .json files
contain the coverage report as it is in the HTML files.
