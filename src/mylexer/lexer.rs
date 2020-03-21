use crate::token::token;

const CHAR0: char = 0 as char;

#[derive(Debug,Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Lexer {
        let mut l = Lexer {
            input: input.into(),
            position: 0,
            read_position: 0,
            ch: CHAR0,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap_or(CHAR0);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut tok: token::Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new_with_string(token::EQ, {
                        let mut x = ch.to_string();
                        x.push(self.ch);
                        x
                    });
                } else {
                    tok = token::Token::new(token::ASSIGN, self.ch)
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new_with_string(token::NOT_EQ, {
                        let mut x = ch.to_string();
                        x.push(self.ch);
                        x
                    });
                } else {
                    tok = token::Token::new(token::BANG, self.ch)
                }
            },
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
            '"'=>  tok = token::Token::new_with_string(token::STRING,self.read_string()),
            '[' => tok = token::Token::new(token::LBRACKET, self.ch),
            ']' => tok = token::Token::new(token::RBRACKET, self.ch),
            ':'=> tok = token::Token::new(token::COLON,self.ch),
            CHAR0 => tok = token::Token::new(token::EOF, CHAR0),
            _ => {
                if is_letter(self.ch) {
                    tok = token::Token::default();
                    tok.Literal = self.read_identifier();
                    tok.Type = token::lookup_ident(&tok.Literal);
                    return tok;
                } else if is_digit(self.ch) {
                    tok = token::Token::default();
                    tok.Type = token::INT;
                    tok.Literal = self.read_number();
                    return tok;
                } else {
                    tok = token::Token::new(token::ILLEGAL, self.ch);
                }
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        // while is_letter(self.ch) {
        //     self.read_char();
        // }
        while is_var_name(self.ch) {
            self.read_char();
        }

        unsafe {
            self.input
                .get_unchecked(position..self.position)
                // .slice_mut_unchecked(position, self.position)
                .to_string()
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                _ => break,
            }
        }
    }
    fn read_string(&mut self)->String{
        let position = self.position+1;
        loop{
            self.read_char();
            if self.ch == '"'{
                break;
            }
        }
        unsafe {
            self.input
                .get_unchecked(position..self.position)
                .to_string()
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        
        unsafe {
            self.input
                .get_unchecked(position..self.position)
                .to_string()
        }
    }

    fn peek_char(&mut self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or(CHAR0)
    }
}

//辅助函数
fn is_letter(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_var_name(ch: char)-> bool {
    is_letter(ch) || is_digit(ch)
}