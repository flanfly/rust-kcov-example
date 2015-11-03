Example for using kcov with Rust
================================

First install the Rust toolchain from rust-lang.org.
Then download and compile kcov (https://github.com/SimonKagstrom/kcov).

Now compile this Rust library and her tests with `cargo test --no-run`.
Run both test executables with kcov:

``
kcov target/kcov target/debug/integration-3f8286225d0e7931
kcov target/kcov target/debug/kcov_example-b255e26cbfdb882e
``

Coverage results will be placed in target/kcov/kcov-merged. The .json files
contain the coverage report as it is in the HTML files.
