# About

This is a tactical fantasy RPG.

The licenses here are provided for template purposes: this repository itself is provided under MIT-0.
Feel free to use, hack and adopt this freely: no attribution needed.

### Running

Use `cargo run`.
This repo is set up to always build with full optimizations, so there's no need for a `--release` flag in most cases.
Dynamic linking is enabled to ensure build times stay snappy.

To run an example, use `cargo run --example_name`, where `example_name` is the file name of the example without the `.rs` extension.

### Publishing

A build will be produced for Windows, MacOS and Linux each time a [tag](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/managing-commits/managing-tags) is pushed to GitHub.

These can be found under the [Releases](https://docs.github.com/en/rest/reference/releases) tab of your project.

## Contributing

See [CONTRIBUTING.md](https://github.com/Sheepyhead/tactical-rpg/blob/main/CONTRIBUTING.md)!

### Testing

1. Use doc tests aggressively to show how APIs should be used.
You can use `#` to hide a setup line from the doc tests.
2. Unit test belong near the code they are testing. Use `#[cfg(test)]` on the test module to ignore it during builds, and `#[test]` on the test functions to ensure they are run.
3. Integration tests should be stored in the top level `tests` folder, importing functions from `lib.rs`.

Use `cargo test` to run all tests.

### CI

The CI will:

1. Ensure the code is formatted with `cargo fmt`.
2. Ensure that the code compiles.
3. Ensure that (almost) all `clippy` lints pass.
4. Ensure all tests pass on Windows, MacOS and Ubuntu.

Check this locally with:

1. `cargo run -p ci`
2. `cargo test --workspace`

To manually rerun CI:

1. Navigate to the `Actions` tab.
2. Use the dropdown menu in the CI run of interest and select "View workflow file".
3. In the top-right corner, select "Rerun workflow".

### Documentation

Reference documentation is handled with standard Rust doc strings.
Use `cargo doc --open` to build and then open the docs.

Design docs (or other book-format documentation) is handled with [mdBook](https://rust-lang.github.io/mdBook/index.html).
Install it with `cargo install mdbook`, then use `mdbook serve --open` to launch the docs.

### Benchmarking

To run the benchmarks, use `cargo bench`.

For more documentation on making your own benchmarks, check out [criterion's docs](https://bheisler.github.io/criterion.rs/book/index.html).
