use std::env;

fn main() {
    let name = env::args().skip(1).next();
    println!("Hello, {}!", name.unwrap_or("world".into()));
}
/*
extern crate mylib;

fn main() {
    let thing = mylib::make_a_thing();
    println!("I made a thing: {:?}", thing);
}
*/
/*
// This crate is a library
#![crate_type = "lib"]
// The library is named "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}*/
