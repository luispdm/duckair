// "cargo test --test integration_test" to run only
// the integration tests in this folder

// it is best practice to place the integration tests
// inside the  "tests" folder
use tests::add;
mod common;
use common::setup;

// binary crates (the ones having a "main.rs") cannot be
// tested with integration tests directly

/*
 * I don't need to specify "#[cfg(test)]" as cargo knows that
 * all the files inside the "tests" folder are tests.
 * Cargo will turn each of the files inside this folder into a crate.
 */
#[test]
fn integration() {
    setup();
    assert_eq!(4, add(2, 2));
}
