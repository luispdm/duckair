// this is how you declare submodules
pub mod outer {
    pub mod inner {
        pub fn print_smth() {
            println!("smth")
        }
    }
}

fn print_one() {
    println!("1");
}

pub fn print_four_times() {
    // absolute path
    crate::modules::outer::inner::print_smth();
    // relative path
    outer::inner::print_smth();
    // relative path on the same level
    print_one();
    // relative path with "super": super references the parent module
    super::modules::print_one();
}

mod priv_mod {
    fn _priv_fn() {
        // relative path with "super": super references the parent module
        super::print_one();
    }
}

mod cooking {
    // struct can also be private or public
    struct _Lunch {
        appetizer: String,
        first: String,
    }
    // fields of a struct are private by default (i.e. cannot be used outside the module)
    pub struct _Dinner {
        first: String,
        second: String,
    }
    // i need to explicitly declare the fields public to make them editable from outside the module
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }

    impl Breakfast {
        // functions of a struct must be declared public to be used outside the module
        pub fn this_morning(&self) {
            println!("you eat {} and {}", self.toast, self.fruit)
        }
    }
}

pub fn eat() {
    /*
     * Lunch cannot be instantied as it is a private struct
     * Dinner cannot be instantiated with a struct literal as its fields are private
     * Dinner can be instantiated only if a constructor is created
     */
    let mut brk = cooking::Breakfast {
        toast: String::from("avocado"),
        fruit: String::from("apricot"),
    };
    brk.fruit = String::from("peach");
    brk.this_morning();
}

mod enums {
    /*
     * enums can be private or public
     * if the enum is private, the variants are private
     * if the enum is public, the variants are public, no need to specify "pub" next to them
     */
    pub enum _Appetizer {
        Soup,
        Salad,
    }
}
