pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of; // bringing name into scope with the use keyword to avoid lengthy dec in main

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow}; // bringing multiple variants into scope

use TrafficLight::*; // bringing all name into scope


fn main() {
    // main.rs has beeen created in communications lib to use the module
    // this file is just for part two

    //As you can see, referring to the fully qualified name can get quite
    //lengthy. Fortunately, Rust has a keyword to make these calls more concise
    a::series::of::nested_modules();

    of::nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;// since all name are into scope now we can use Green
    let gr = Green;

    println!("Hello, world!");
}
