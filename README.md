# The Rust programming language
Examples on the Rust programming language coming from the Pluralsight course "Rust fundamentals" and the "Let's Get Rusty" video series following the Rust Book.

## More examples on cargo
```shell
cargo r --bin $BINARY_NAME # to run of the binaries listed in Cargo.toml. --bin or -b.
cargo r --package $BINARY_PACKAGE_NAME # to run one of the binary packages located in this repo (the folders with the main.rs files that have their own Cargo.toml). --package or -p.
cargo test --workspace # tests all members of the workspace
cargo test -p $PACKAGE_NAME # tests only that binary/library crate
cargo install ripgrep # will install the ripgrep binary as an executable in our system (similar to "go install XXX")
```

cargo can be extended with custom commands by prefixing the name of the command with "cargo-". This way, the binary can be used as an extension of cargo by running `cargo $COMMAND`. clippy is an extension of cargo because its binary is called `cargo-clippy`.

## Links
Let's Get Rusty YouTube playlist: https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8

Pluralsight course "Rust fundamentals": https://www.pluralsight.com/courses/fundamentals-rust

The Rust Book: https://doc.rust-lang.org/book/title-page.html
