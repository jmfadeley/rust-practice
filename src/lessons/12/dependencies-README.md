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