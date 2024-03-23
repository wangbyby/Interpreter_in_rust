use std::io;
use std::io::prelude::*;

use crate::evaluator::evaluator;
use crate::mylexer::Lexer;
use crate::parser::parser;

const PROMPT: &'static str = ">>> ";

pub fn start() {
    let mut env = evaluator::Environment::new();
    loop {
        let mut scanned = String::new();
        io::stdout().write(PROMPT.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut scanned).unwrap();

        let mut lexer = Lexer::new(&scanned);
        let mut p = parser::Parser::new(lexer);
        let program = p.parse_program().unwrap();

        // println!("{:#?}",program);
        let evaled = evaluator::eval(&program, &mut env);
        println!("{}", evaled.inspect());
    }
}
