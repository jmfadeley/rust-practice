Cargo is the official Rust package management tool. It has:
- Dependency management and integration with crates.io
- Awareness of unit tests
- Awareness of benchmarks

Lessons "12.1 Dependencies"
https://doc.rust-lang.org/rust-by-example/cargo/deps.html

Most programs have dependencies on some libraries. Rust's caro manages dependencies for a project.

To create a new Rust project
`cargo new foo`
This creates a `main.rs` and `Cargo.toml` file.

For a library:
`cargo new --lib bar`

You can also add an authors field with something like `authors=["rob"]`

One can add fields like clap, rand or bar in the given examples for crates.io, git or local.
Then do `cargo build` to build or `cargo run` to both build and run.

Lesson "12.2 Conventions"
https://doc.rust-lang.org/rust-by-example/cargo/conventions.html

If we want ot have two binaries in the same project, cargo supports this. The default is `main` but additional
binaries are placed under the `bin/` directory.

To tell `cargo` to only compile or run this binary we just pass `cargo` the `--bin my_other_bin` flag, where
`my_other_bin` is the name of the binary we want to work with.

Lesson "12.3 Tests"
https://doc.rust-lang.org/rust-by-example/cargo/test.html

Be careful because tests run concurrently. See `ferris.txt` for the back and forth of the writing and not "ferris" 5 times followed
by "Corro" 5 times.

Running the tests can be run with `cargo test` in the appropriate folder (they'll be found in `src/lessons/12/foo`, but not higher)
or globly using `cargo test test_file` which will run both `test_file` and `test_file_also` but note that `west` isn't run.

To see this, go into `/src/lessons/12/tests` and run `cargo test test_file && cat ../ferris.txt`

Lesson "12.4 Scripts"
https://doc.rust-lang.org/rust-by-example/cargo/build_scripts.html

A build script details something that Cargo might need to run. It can be added to `Cargo.toml` with...
`[package]`
`build = "build.rs"`
But Cargo will look for a `build.rs` by default too.

The build script is another Rust file that is compiled and invoked before compiling anything else. Thus
it can take care of prerequisites. There are env variables available for input
(see https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)

The script provides output via stdout. All lines are written to `target/debug/build/<pkg>/output`
Lines prefixed with `cargo:` will be interpreted by Cargo directly and hence can be used to define parameters for the package's 
compilation.

See here for more:
https://doc.rust-lang.org/cargo/reference/build-scripts.html
