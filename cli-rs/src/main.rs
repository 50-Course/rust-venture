use std::env;
use std::path::PathBuf;



// Let's model our command line
// requires at least two arguments
// arg1: pattern in form of string, or regex-string (to be searched)
// arg2: path to search in
struct CliModel {
    pattern: String,
    path: PathBuf
}


fn main() {
    // grab some command line argument
    // and print them back to the console
    for arg in env::args() {
        println!("{}", arg);
    }

}
