

pub mod lexer;
pub use lexer::Lexer;


// use crate::token::token;
// struct Tests{
//     expectedType : token::TokenType, 
//     expectedLiteral : &'static str,
// }

// impl Tests{
//     fn new(et : token::TokenType, el: &'static str) -> Tests{
//         Tests{expectedType:et, expectedLiteral: el}
//     }
// }

// #[test]
// fn test_next_token(){
//     let input = "let five = 5;
//     let ten = 10;
//     let add = fn(x,y) {
//         x+y;
//     };
//     !-/*5;
//     5 < 10 > 5;
//     if (5 < 10) {
//         return true;
//     }else{
//         return false;
//     }

//     10==10;
//     10 != 9;
// ";

//     let tests = [
//         Tests {expectedType: token::LET,expectedLiteral: "let"},
//         Tests {expectedType: token::IDENT,expectedLiteral: "five"},
//         Tests {expectedType: token::ASSIGN,expectedLiteral: "="},
//         Tests {expectedType: token::INT,expectedLiteral: "5"},
//         Tests {expectedType: token::SEMICOLON,expectedLiteral: ";"},
//         Tests {expectedType: token::LET,expectedLiteral: "let"},
//         Tests {expectedType: token::IDENT,expectedLiteral: "ten"},
//         Tests {expectedType: token::ASSIGN,expectedLiteral: "="},
//         Tests {expectedType: token::INT,expectedLiteral: "10"},
//         Tests {expectedType: token::SEMICOLON,expectedLiteral: ";"},
//         Tests {expectedType: token::LET,expectedLiteral: "let"},
//         Tests {expectedType: token::IDENT,expectedLiteral: "add"},
//         Tests {expectedType: token::ASSIGN,expectedLiteral: "="},
//         Tests {expectedType: token::FUNCTION,expectedLiteral: "fn"},
//         Tests {expectedType: token::LPAREN,expectedLiteral: "("},
//         Tests::new(token::IDENT, "x"),
//         Tests::new(token::COMMA, ","), 
//         Tests::new(token::IDENT, "y"),
//         Tests::new(token::RPAREN,")"),
//         Tests::new(token::LBRACE, "{"),
//         Tests::new(token::IDENT, "x"),
//         Tests::new(token::PLUS, "+"),
//         Tests::new(token::IDENT, "y"),
//         Tests::new(token::SEMICOLON, ";"),
//         Tests::new(token::RBRACE, "}"),
//         Tests::new(token::SEMICOLON, ";"),

//         ];

//     let mut l = Lexer::new(input.to_string());

//     for i in tests.iter(){
//         let tok = l.next_token();

//         assert_eq!(tok.Type, i.expectedType);
//         assert_eq!(tok.Literal, i.expectedLiteral);

//     }
// }