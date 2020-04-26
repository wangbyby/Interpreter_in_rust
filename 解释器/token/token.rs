

use std::collections::HashMap;
pub fn lookup_ident(ident:& String) -> TokenType {
    let  mmap: HashMap<String,TokenType> = {
        let mut  map = HashMap::new();
        map.insert("fn".to_string(), FUNCTION);
        map.insert("let".to_string(), LET);
        map.insert("true".to_string(),TRUE);
        map.insert("false".to_string(),FALSE);
        map.insert("if".to_string(), IF);
        map.insert("else".to_string(),ELSE);
        map.insert("return".to_string(), RETURN);
        map.insert("class".to_string(), CLASS);
        map
    };
    mmap.get(ident).unwrap_or(&IDENT)
}


pub const ILLEGAL: &'static str  =  "ILLEGAL";
pub const EOF: &'static str =  "EOF";

//identifier + literal
pub const IDENT :&'static str =  "IDENT";
pub const INT :&'static str =  "INT";

// operator
pub const ASSIGN :&'static str =  "=";
pub const PLUS :&'static str =  "+";
pub const MINUS :&'static str = "-";
pub const BANG:&'static str = "!";
pub const ASTERISK :&'static str = "*";
pub const SLASH: &'static str =  "/";
pub const LT :&'static str = "<";
pub const GT :&'static str = ">";
pub const EQ :&'static str = "==";
pub const NOT_EQ :&'static str = "!=";
//delimiters
pub const COMMA :&'static str =  ",";
pub const SEMICOLON :&'static str =  ";";

pub const LPAREN :&'static str =  "(";
pub const RPAREN :&'static str =  ")";
pub const LBRACE :&'static str =  "{";
pub const RBRACE :&'static str =  "}";

pub const LBRACKET: &'static str  =  "[";
pub const RBRACKET: &'static str = "]";
pub const COLON: &'static str = ":";

//keywords
pub const FUNCTION : &'static str = "FUNCTION";
pub const LET : &'static str = "LET";
pub const TRUE : &'static str = "TRUE";
pub const FALSE : &'static str = "FALSE";
pub const IF : &'static str = "IF";
pub const ELSE : &'static str = "ELSE";
pub const RETURN : &'static str = "RETURN";
pub const STRING : &'static str = "STRING";
pub const CLASS : &'static str = "CLASS";


pub type TokenType  =  &'static str;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Token {
    pub Type: TokenType,
    pub Literal:String,
}


impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Token {
        Token {
            Type:token_type, 
            Literal: ch.to_string(),
        }
    }
    pub fn default() -> Token {
        Token {
            Type:ILLEGAL, 
            Literal: String::new(),
        }
    }
    pub fn new_with_string<S: Into<String>>(token_type: TokenType, string: S) -> Token {
        Token { 
            Type: token_type,
            Literal: string.into(),
        }
    }
}

