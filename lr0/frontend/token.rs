
pub const ILLEGAL: &'static str  =  "ILLEGAL";
pub const EOF: &'static str =  "EOF";

//identifier + literal
pub const IDENT :&'static str =  "IDENT";
pub const DECINT :&'static str =  "DECINT";
pub const DECFLOAT :&'static str =  "DECFLOAT";
pub const OCT :&'static str =  "OCT";
pub const HEX :&'static str =  "HEX";

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
pub const DOT :&'static str = ".";
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

pub type Token = (String, TokenType); //(Literal, Type)


pub fn lookup_keyword(s:&String) -> Token{
    match s.as_str() {
        "fn"=> (s.to_string(), FUNCTION ) ,
        //TODO
        _=> (s.to_string(), IDENT) , 
    }
}