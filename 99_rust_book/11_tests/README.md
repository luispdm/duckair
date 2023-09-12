# tests

`cargo test` compiles the code in test mode and runs the result in the test binary.

Two sets of command-line options: one goes to the cargo test command, and the other goes to the test binary. Options that go to the cargo test command go right after the command itself. Example:
```shell
cargo test --package=pepito
```
Options that go to the test binary are separated by `--`. Example:
```shell
cargo test -- --quiet
```

By default, `cargo test` runs tests in parallel (each test gets its own thread) and it captures all the tests' output (i.e. `println!` results are not printed to screen). To specify the number of threads that should run the tests do:
```shell
cargo test -- --test-threads=n # n must be 1 if you don't want the tests to run in parallel
```
To show the output of successufl tests (the output of the failing tests is always shown):
```shell
cargo test -- --show-output
```

To filter the tests you want to run:
```shell
# let use assume that "pepito", "add_one", and "add_two" are the names of the tests and that the module "lib" offers another set of tests in a different file/folder
cargo test # it runs everything
cargo test pepito # it runs "pepito"
cargo test add # it runs all the tests whose name starts with "add"
cargo test lib:: # it runs all the tests in the "lib" module
```
`cargo test --ignored` runs only the ignored tests

`cargo test --test integration_test` to run only the integration tests in the file `tests/integration_test.rs`
