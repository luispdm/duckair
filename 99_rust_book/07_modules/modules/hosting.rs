mod network {
    /*
     * This is how to export a property with a different name.
     * Both the property and the statement must use "pub" to make
     * the property accessible outside the module.
     *
     * "self" refers to the current module.
     */
    pub use self::godaddy::NAME as daddy;
    mod godaddy {
        pub const NAME: &str = "GO_DADDY";
    }
}

pub fn hosting() {
    println!("hosting {}", network::daddy);
}
