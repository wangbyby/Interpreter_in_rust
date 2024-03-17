use crate::token::token::{lookup_ident, Token, TokenType::*};
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

    pub fn next_token(&mut self) -> Option<Token> {
        let mut tok: Token;

        self.skip_whitespace();

        match self.next_char()? {
            '=' => {
                if self.peek_char()? == '=' {
                    self.next_char();
                    tok = Token::new(EQ, "==");
                } else {
                    tok = Token::new(ASSIGN, self.ch)
                }
            }
            '!' => {
                if self.peek_char()? == '=' {
                    self.next_char();
                    tok = Token::new(NotEQ, "!=");
                } else {
                    tok = Token::new(BANG, self.ch)
                }
            }
            '(' => tok = Token::new(LPAREN, self.ch),
            ')' => tok = Token::new(RPAREN, self.ch),
            '{' => tok = Token::new(LBRACE, self.ch),
            '}' => tok = Token::new(RBRACE, self.ch),
            '+' => tok = Token::new(PLUS, self.ch),
            '-' => tok = Token::new(MINUS, self.ch),
            '*' => tok = Token::new(ASTERISK, self.ch),
            '/' => tok = Token::new(SLASH, self.ch),
            '<' => tok = Token::new(LT, self.ch),
            '>' => tok = Token::new(GT, self.ch),
            ',' => tok = Token::new(COMMA, self.ch),
            ';' => tok = Token::new(SEMICOLON, self.ch),
            '"' => tok = Token::new(Str, self.read_string(None)),
            '[' => tok = Token::new(LBRACKET, self.ch),
            ']' => tok = Token::new(RBRACKET, self.ch),
            ':' => tok = Token::new(COLON, self.ch),
            ch if is_letter(ch) => {
                let lit = self.read_identifier(Some(ch));
                tok = Token::new(lookup_ident(&lit), lit);
            }
            ch if is_digit(ch) => {
                tok = Token::new(INT, self.read_number(Some(ch)));
            }
            CHAR0 => tok = Token::new(EOF, CHAR0),
            _ => {
                tok = Token::new(ILLEGAL, self.ch);
            }
        }
        Some(tok)
    }

    fn next_char(&mut self) -> Option<char> {
        (self.position, self.ch) = self.input.next()?;
        Some(self.ch)
    }
    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().map(|(_, c)| *c)
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
        if let Some(ch) = first_char {
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
    type Item = Token;
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
    use crate::token::token::{Token, TokenType};

    use super::Lexer;

    #[test]
    fn test() {
        use TokenType::*;
        let s = "let a = 10";
        let mut lexer = Lexer::new(s);

        let mut a = Token::new(Let, "let");
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::new(IDENT, 'a');
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::new(ASSIGN, '=');
        assert_eq!(Some(a), lexer.next_token());

        let mut a = Token::new(INT, "10");

        assert_eq!(Some(a), lexer.next_token());
    }
}
