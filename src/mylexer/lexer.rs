use crate::token::token;
use std::fmt::Write;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::CharIndices;

const CHAR0: char = 0 as char;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<CharIndices<'a>>,
    position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new<'b>(input: &'b str) -> Lexer<'b> {
        Lexer {
            input: input.char_indices().peekable(),
            position: 0,
            ch: CHAR0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        (self.position, self.ch) = self.input.next()?;
        Some(self.ch)
    }
    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().map(|(_, c)| *c)
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        let mut tok: token::Token;

        self.skip_whitespace();

        match self.next_char()? {
            '=' => {
                if self.peek_char()? == '=' {
                    self.next_char();
                    tok = token::Token::new_with_string(token::EQ, "==".to_string());
                } else {
                    tok = token::Token::new(token::ASSIGN, self.ch)
                }
            }
            '!' => {
                if self.peek_char()? == '=' {
                    self.next_char();
                    tok = token::Token::new_with_string(token::NOT_EQ, "!=".to_string());
                } else {
                    tok = token::Token::new(token::BANG, self.ch)
                }
            }
            '(' => tok = token::Token::new(token::LPAREN, self.ch),
            ')' => tok = token::Token::new(token::RPAREN, self.ch),
            '{' => tok = token::Token::new(token::LBRACE, self.ch),
            '}' => tok = token::Token::new(token::RBRACE, self.ch),
            '+' => tok = token::Token::new(token::PLUS, self.ch),
            '-' => tok = token::Token::new(token::MINUS, self.ch),
            '*' => tok = token::Token::new(token::ASTERISK, self.ch),
            '/' => tok = token::Token::new(token::SLASH, self.ch),
            '<' => tok = token::Token::new(token::LT, self.ch),
            '>' => tok = token::Token::new(token::GT, self.ch),
            ',' => tok = token::Token::new(token::COMMA, self.ch),
            ';' => tok = token::Token::new(token::SEMICOLON, self.ch),
            '"' => tok = token::Token::new_with_string(token::STRING, self.read_string(None)),
            '[' => tok = token::Token::new(token::LBRACKET, self.ch),
            ']' => tok = token::Token::new(token::RBRACKET, self.ch),
            ':' => tok = token::Token::new(token::COLON, self.ch),
            ch if is_letter(ch) => {
                tok = token::Token::default();
                tok.Literal = self.read_identifier(Some(ch));
                tok.Type = token::lookup_ident(&tok.Literal);
            }
            ch if is_digit(ch) => {
                tok = token::Token::default();
                tok.Type = token::INT;
                tok.Literal = self.read_number(Some(ch));
            }
            CHAR0 => tok = token::Token::new(token::EOF, CHAR0),
            _ => {
                tok = token::Token::new(token::ILLEGAL, self.ch);
            }
        }
        Some(tok)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.next_char();
                }
                _ => break,
            }
        }
    }

    fn read_<F: Fn(char) -> bool>(&mut self, first_char: Option<char>, ok_fn: F) -> String {
        let mut id = String::with_capacity(8);
        println!("here char={:?}", first_char);
        if let Some(ch) =first_char  {
            id.write_char(ch).unwrap();
        }
        while let Some(n) = self.peek_char() {
            if ok_fn(n) {
                id.write_char(n).unwrap();
                self.next_char();
            } else {
                break;
            }
            println!("here {}", id);
        }
        id
    }

    fn read_identifier(&mut self, first_char: Option<char>) -> String {
        self.read_(first_char, is_var_name)
    }

    fn read_string(&mut self, first_char: Option<char>) -> String {
        self.read_(first_char, |c| c == '"')
    }

    fn read_number(&mut self, first_char: Option<char>) -> String {
        self.read_(first_char, is_digit)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = token::Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_letter(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_var_name(ch: char) -> bool {
    is_letter(ch) || is_digit(ch)
}

#[cfg(test)]
mod test_lexer {
    use crate::token::{
        self,
        token::{Token, TokenType},
    };

    use super::Lexer;

    #[test]
    fn test() {
        let s = "let a = 10";
        let mut lexer = Lexer::new(s);

        let mut a = Token::default();
        a.Literal = "let".to_string();
        a.Type = token::token::LET;
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::default();
        a.Literal = "a".to_string();
        a.Type = token::token::IDENT;
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::default();
        a.Literal = "=".to_string();
        a.Type = token::token::ASSIGN;
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::default();
        a.Literal = "10".to_string();
        a.Type = token::token::INT;
        assert_eq!(Some(a), lexer.next_token());
    }
}
