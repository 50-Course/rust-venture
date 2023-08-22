use std::env;

fn main() {
    // grab some command line argument
    // and print them back to the console
    for arg in env::args() {
        println!("{}", arg);
    }

//    println!("Hello, world!");
}
