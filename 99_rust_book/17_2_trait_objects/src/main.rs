use trait_objects::{Button, Screen, SelectBox};

fn main() {
    println!("Hello, world!");
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 0,
                height: 0,
                label: String::from("test"),
            }),
            Box::new(SelectBox {
                width: 0,
                height: 0,
                options: vec![String::from("first"), String::from("second")],
            }),
            // the below does not compile as String does not implement Draw
            // Box::new(String::from("astring")),
        ],
    };
    s.run();
}
