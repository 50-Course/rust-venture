use std::env;
use std::path::{PathBuf, Path};


// Let's model our command line
// requirjs at least two arguments
//
// arg1: pattern in form of string, or regex-string (to be searched)
// arg2: path to search in
// TOdO: understand what #[warn(dead_code)] means, error given by LSP
// I would assue this is reffering to model not been used yet, hence
// would get deferenced or something by the compiler
struct Cli {
    pattern: String,
    path: PathBuf::fromPath
}


fn main() {
   // grab some command-line arg from the user
   // namespace (as it is been called in C/C++ and similar to other languages modules)
   // - has this unwraps() functions that allows us to access the values of some key-variable
   // essentially acting as a deserializer decorator (at least what i think but imma look it
   // up later)
   //
   // however contains panics if couldn't find a value for this K-V (that is when K-V is {K: Null}
   // and crashes the program.
   //
   // Found out theres a little method call expect that allows us to write out some human-readable
   // messages instead.
   let pattern = env::args().nth(1).expect("What exactly do you want to search, include a string");
   let path = env::args().nth(2).expect("Enter path to file");
   let args = Cli {
       pattern: pattern,
       path: PathBuf::from(path)
   };
}
