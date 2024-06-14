use std::env;

pub fn greet() {
    println!("Hello, {}!", env::var("USERNAME").unwrap_or(String::from("User")));
}