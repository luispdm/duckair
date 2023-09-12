/*
 * Methods and properties used by multiple integration tests
 * must go in folders and files under the "tests" folder.
 * 
 * The code inside this folder won't be compiled as
 * a crate because it is located in a subfolder of
 * the "tests" folder (i.e. cargo assumes there are
 * no tests here).
 */
pub fn setup() {
    // setting up the DB...
}
