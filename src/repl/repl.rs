
use std::io;
use std::io::prelude::*;

use crate::mylexer::Lexer;
use crate::parser::parser;
use crate::evaluator::evaluator;


const PROMPT : &'static str= ">>> ";


pub fn start() {
    let mut env =  evaluator::Environment::new();
    loop {
        let mut scanned = String::new();
        io::stdout().write(PROMPT.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut scanned).unwrap();
        
        let  lexer = Lexer::new(scanned.clone());
        let mut p = parser::Parser::new(lexer);
        let  program = p.parse_program();

        if p.errors.len() != 0 {
            println!("get Errors: {:?}", p.error());
            continue;
        }
        
        // println!("{:#?}",program); 
        let evaled = evaluator::eval(&program,&mut env);
        println!("{}",evaled.inspect()); 
            
        
        
    
        
    }
}