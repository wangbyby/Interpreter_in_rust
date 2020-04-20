use super::token;
use std::iter::Peekable;
use std::str::Chars;


const CHAR0: char = 0 as char;

#[derive(PartialOrd,Eq, PartialEq)]
enum DFAState {
    Error,
    START,
    ONEZERO,
    ONEDOT,
    OCT,
    HEX,
    DEC, //int
    DECFLOAT,//float 
    IDENT,
    ACC,
    IDENTACC, 
    OCTACC,
    HEXACC,
    DECINTACC,
    DECFLOATACC,
}



fn dfa_number_ident(s:& DFAState, ch: Option<char>) -> Option<DFAState> {
    use DFAState::*;
    ch.map(|ch|match (s, ch) {    
        (START, '0')=> Some(ONEZERO), 
        (START, '1'..='9')=> Some(DEC),
        (START, 'a'..='z')=> Some(IDENT),
        (START, 'A'..='Z')=> Some(IDENT),
        (START, '_')=> Some(IDENT),
        (ONEZERO, '1'..='7')=>Some(OCT),
        (ONEZERO,'x')=>Some(HEX),
        (ONEZERO,'.')=>Some(ONEDOT),
        (ONEZERO, _)=>Some(DEC),
        (ONEDOT, '0'..='9')=>Some(DECFLOAT),
        (DECFLOAT, '0'..='9')=>Some(DECFLOAT),
        (DECFLOAT,_)=>Some(DECFLOATACC),
        (OCT, '0'..='7')=>Some(OCT),
        (OCT, _)=>Some(OCTACC),
        (DEC, '0'..='9')=>Some(DEC),
        (DEC, '.')=>Some(ONEDOT),
        (DEC, _)=>Some(DECINTACC),
        (HEX, '0'..='9')=>Some(HEX), //(5, '0'..='9' | 'a'..='f' | 'A'..='F')=>Some(5)还在实验中
        (HEX, 'a'..='f' )=>Some(HEX),
        (HEX, 'A'..='F' )=>Some(HEX),
        (HEX, _)=>Some(HEXACC),
        (IDENT, '0'..='9')=>Some(IDENT),
        (IDENT, 'a'..='z')=>Some(IDENT),
        (IDENT, 'A'..='Z')=>Some(IDENT),
        (IDENT, _)=>Some(IDENTACC),
        _=>None,
    } ).flatten()
    
}

macro_rules! make_token {
    ($literal:expr, $type:ident) => {
        (format!("{}", $literal), token::$type)
    };
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }
    pub fn next_token(&mut self) -> Option<token::Token> {
        self.skip_whitespace();
        self.next_char().map(|ch| match ch {
            '=' => {
                if self.peek_char().unwrap_or(CHAR0) == '=' {
                    self.next_char();
                    make_token!("==", EQ)
                } else {
                    make_token!("=", ASSIGN)
                }
            },
            '.'=> make_token!(".", DOT),
            ';'=> make_token!(";",SEMICOLON),
            // TODO:
            _ => self.dfa(ch),
        })
    }

    fn dfa(&mut self,  ch: char)->token::Token{
        use DFAState::*;

        let mut state = START;
        let mut string1 = ch.to_string();
        state = dfa_number_ident(&state, Some(ch)).unwrap_or(Error);
        while let Some(s) = dfa_number_ident(&state,self.peek_char()) {
            
            state = s;
            if state > ACC {
                break;
            }
            self.next_char().map(|ch| string1.push(ch));
            
        }

        match state {
            IDENTACC=>token::lookup_keyword(&string1),
            OCTACC=> (string1, token::OCT),
            HEXACC=> (string1, token::HEX),
            DECINTACC=> (string1, token::DECINT),
            DECFLOATACC=> (string1, token::DECFLOAT),
            _=> (string1, token::ILLEGAL),
        }
        
        
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(c) => match c {
                    ' ' | '\t' | '\n' | '\r' => {
                        self.next_char();
                    }
                    _ => break,
                },
                None => break,
            }
        }
    }

    fn next_char(&mut self) -> Option<char> { //下一个字符,取出
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<char> {//康康下一个字符, 不取出
        self.input.peek().map(|ch| *ch)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_skip_whitespace() {
        let mut l = Lexer::new("     S");
        l.skip_whitespace();
        assert_eq!(l.next_char().unwrap(), 'S');
    }

    #[test]
    fn test_ident(){
        let s = " int a102a;";
        let mut l = Lexer::new(s);

        assert_eq!(l.next_token(),Some(("int".to_string(), token::IDENT)));
        assert_eq!(l.next_token(),Some(("a102a".to_string(), token::IDENT)));
        
    }

    #[test]
    fn test_number(){
        let s = "1234.0981  1234 0.1298;";
        let mut l = Lexer::new(s);
        assert_eq!(l.next_token(),Some(("1234.0981".to_string(), token::DECFLOAT)));
        assert_eq!(l.next_token(),Some(("1234".to_string(), token::DECINT)));
        assert_eq!(l.next_token(),Some(("0.1298".to_string(), token::DECFLOAT)));
        assert_eq!(l.next_token(),Some((";".to_string(), token::SEMICOLON)));
    }
}
