use std::{collections::HashMap, io::Write};

// pub const ILLEGAL: &'static str = "ILLEGAL";
// pub const EOF: &'static str = "EOF";

// //identifier + literal
// pub const IDENT: &'static str = "IDENT";
// pub const INT: &'static str = "INT";

// // operator
// pub const ASSIGN: &'static str = "=";
// pub const PLUS: &'static str = "+";
// pub const MINUS: &'static str = "-";
// pub const BANG: &'static str = "!";
// pub const ASTERISK: &'static str = "*";
// pub const SLASH: &'static str = "/";
// pub const LT: &'static str = "<";
// pub const GT: &'static str = ">";
// pub const EQ: &'static str = "==";
// pub const NOT_EQ: &'static str = "!=";
// //delimiters
// pub const COMMA: &'static str = ",";
// pub const SEMICOLON: &'static str = ";";

// pub const LPAREN: &'static str = "(";
// pub const RPAREN: &'static str = ")";
// pub const LBRACE: &'static str = "{";
// pub const RBRACE: &'static str = "}";

// pub const LBRACKET: &'static str = "[";
// pub const RBRACKET: &'static str = "]";
// pub const COLON: &'static str = ":";

// //keywords
// pub const FUNCTION: &'static str = "FUNCTION";
// pub const LET: &'static str = "LET";
// pub const TRUE: &'static str = "TRUE";
// pub const FALSE: &'static str = "FALSE";
// pub const IF: &'static str = "IF";
// pub const ELSE: &'static str = "ELSE";
// pub const RETURN: &'static str = "RETURN";
// pub const STRING: &'static str = "STRING";
// pub const CLASS: &'static str = "CLASS";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    EOF,
    IDENT,
    INT,
    ASSIGN,    // =
    PLUS,      // +
    MINUS,     // -
    BANG,      // !
    ASTERISK,  // "*"
    SLASH,     // "/"
    LT,        // "<"
    GT,        // >
    EQ,        // ==
    NotEQ,     // !=
    COMMA,     // ,
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }
    LBRACKET,  // [
    RBRACKET,  // ]
    COLON,     // :
    Function,  // fn
    Let,       // let
    True,      // true
    False,     // false
    If,        // if
    Else,      // else
    Return,    // return
    Str,       // "..."
    Class,     // class
    ILLEGAL,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        match self {
            EOF => f.write_str("EOF"),
            IDENT => f.write_str("ident"),
            INT => f.write_str("int"),
            ASSIGN => f.write_str("="),
            PLUS => f.write_str("+"),
            MINUS => f.write_str("-"),
            BANG => f.write_str("!"),
            ASTERISK => f.write_str("*"),
            SLASH => f.write_str("/"),
            LT => f.write_str("<"),
            GT => f.write_str(">"),
            EQ => f.write_str("=="),
            NotEQ => f.write_str("!="),
            COMMA => f.write_str(","),
            SEMICOLON => f.write_str(";"),
            LPAREN => f.write_str("("),
            RPAREN => f.write_str(")"),
            LBRACE => f.write_str("{"),
            RBRACE => f.write_str("}"),
            LBRACKET => f.write_str("["),
            RBRACKET => f.write_str("]"),
            COLON => f.write_str(":"),
            Function => f.write_str("fn"),
            Let => f.write_str("let"),
            True => f.write_str("true"),
            False => f.write_str("false"),
            If => f.write_str("if"),
            Else => f.write_str("else"),
            Return => f.write_str("return"),
            Str => f.write_str("string"),
            Class => f.write_str("class"),
            ILLEGAL => f.write_str("ILLEGAL"),
        }
    }
}

pub fn lookup_ident(ident: &String) -> TokenType {
    use TokenType::*;
    let mmap: HashMap<String, TokenType> = {
        let mut map = HashMap::new();
        map.insert("fn".to_string(), Function);
        map.insert("let".to_string(), Let);
        map.insert("true".to_string(), True);
        map.insert("false".to_string(), False);
        map.insert("if".to_string(), If);
        map.insert("else".to_string(), Else);
        map.insert("return".to_string(), Return);
        map.insert("class".to_string(), Class);
        map
    };
    mmap.get(ident).map(|a| *a).unwrap_or(IDENT)
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::ILLEGAL
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub ty: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new<S: Into<String>>(token_type: TokenType, ch: S) -> Token {
        Token {
            ty: token_type,
            literal: ch.into(),
        }
    }

    pub fn is_ty(&self, ty: TokenType) -> bool {
        self.ty == ty
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            ty: TokenType::default(),
            literal: String::new(),
        }
    }
}
