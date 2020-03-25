
use std::io;
use std::io::prelude::*;
use monkey::repl::repl;
fn main() {
    
    io::stdout().write("Hello Monkey\n".as_bytes()).unwrap();
    io::stdout().flush().unwrap();
    
    
    repl::start();
    
    
}
