//TODO

// use crate::token::token;
// use super::ast;
// use super::ast::Node;
// #[test]
// fn test_string(){
//     // let mut program = ast::Program::new();
//     // let mut letstmt = ast::LetStatement::new();
//     // letstmt.token = token::Token::new_with_string(token::LET, "let");
//     // letstmt.name = ast::Identifier::default();
//     // letstmt
//     let  program =ast::Program{
//         statements: vec![
//             Some(Box::new(
//                 ast::LetStatement{
//                     token: token::Token::new_with_string(token::LET, "let"),
//                     name: ast::Identifier{ 
//                         token: token::Token::new_with_string(token::IDENT, "myVar"),
//                         value: "myVar".to_string(),
//                     },
//                     value: Some(Box::new(ast::Identifier{ 
//                         token: token::Token::new_with_string(token::IDENT, "anotherVar"),
//                         value: "anotherVar".to_string(),
//                     })),
//                 } )
//             )
//         ],
//     };

//     assert_eq!(program.node_to_string(), "let myVar = anotherVar;");
// }