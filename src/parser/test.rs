use crate::ast::ast;
use crate::mylexer::lexer;
use crate::token::token;
use std::any::Any;
// use crate::ast::ast::Node;
use super::parser;



#[test]
fn test_let_statment(){
    let tests = vec![
        ("let x =10;","x"),
        ("let y =10;","y"),
        ("let foobar =10;","foobar"),
    ];

    for (i,(input, expected)) in tests.iter().enumerate() {
        let l = lexer::Lexer::new(*input);
        let mut p = parser::Parser::new(l);
        let program = p.parse_program().unwrap();

        match program {
            ast::ASTNode::Program(program) =>{
                match program.statements[0].as_ref() {
                    ast::ASTNode::LetStatement(ref stmt) => {
                        assert_eq!(stmt.name.value,*expected.to_string());
                    }
                    _=> panic!("i want letStatement, the program is {:#?} \n ",program.statements),
                }
            }
            _=> panic!("parse program error: {:?}",program),
            
        }
    }
}


#[test]
fn test_function(){
    let input = "fn(x,y,z){z+y+x+2;}; ";
    let l = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program().unwrap();

    match program {
        ast::ASTNode::Program(program) =>{
            match program.statements[0].as_ref() {
                ast::ASTNode::ExpressionStatement(ref stmt) => {
                    match stmt.expression.as_ref() {
                        ast::ASTNode::FuncLiteral(ref func) =>{
                            // panic!("func = {:#?}", func);
                        },
                        _=> panic!("please give me funcliteral, but i recive {:?}",stmt),
                    }
                }
                _=> panic!("i want ExpressionStatement, the program is {:#?} \n ",program.statements),
            }
        }
        _=> panic!("parse program error: {:#?}",program),
        
    }
    // panic!(" {:#?}", program);
}

// #[test]
// pub fn test_return_stmt(){  
//     let input = "return 5;
//         return 10;
//         return 99999;
//     ";
//     let l = lexer::Lexer::new(input);
//     let mut p = parser::Parser::new(l);
//     let program = p.parse_program().unwrap();
    
//     check_parse_error(&p);

//     assert_eq!(3, program.statements.len());

//     for i in program.statements{
//         match i{
//             Some(stmt)=>{
//                 assert_eq!(stmt.token_literal(),"return");
//             },
//             _=> eprintln!("stmt not returnStatment, got={:?}",i),
//         }
//     }
// }






// #[test]
// fn test_id_expr() {
//     let input = "foobar;";
//     let l = lexer::Lexer::new(input);
//     let mut p = parser::Parser::new(l);
//     let program = p.parse_program().unwrap();
    
//     check_parse_error(&p);
//     if program.statements.len() != 1 {
//         panic!("program = {:?}",program.statements);
//     }
//     match &program.statements[0] {
//         Some(stmt)=>{
//             match stmt.as_any().downcast_ref::<ast::ExpressionStatement>(){
//                 Some(expr_stmt)=>{
//                     match &expr_stmt.expression{
//                         Some(ls)=> {
//                             match ls.as_any().downcast_ref::<ast::Identifier>(){
//                                 Some(b)=>{
//                                     assert_eq!(b.value, "foobar");
//                                     assert_eq!(b.token_literal(), "foobar");
//                                 },
//                                 _=>panic!("not letstmt, has {:#?}",program.statements),
//                             }
//                         },
//                         _=>panic!("expression is none"),
//                     }
                    
//                 },
//                 _=>panic!("not ast::ExpressionStatement"),
//             }
//         },
//         _=> panic!("program.statment[0] is not ast::Expr"),
//     }
// }


// #[test]
// fn test_int_literal_expression() {
//     let input = "5;";
//     let l = lexer::Lexer::new(input);
//     let mut p = parser::Parser::new(l);
//     let program = p.parse_program().unwrap();
    
//     check_parse_error(&p);
//     if program.statements.len() != 1 {
//         panic!("program = {:?}",program.statements);
//     }

//     match &program.statements[0] {
//         Some(stmt)=>{
//             match stmt.as_any().downcast_ref::<ast::ExpressionStatement>(){
//                 Some(expr_stmt)=>{
//                     match &expr_stmt.expression{
//                         Some(ls)=> {
//                             match ls.as_any().downcast_ref::<ast::IntegerLiteral>(){
//                                 Some(b)=>{
//                                     assert_eq!(b.value, 5);
//                                     assert_eq!(b.token_literal(), "5");
//                                 },
//                                 _=>panic!("not letstmt, has {:#?}",program.statements),
//                             }
//                         },
//                         _=>panic!("expression is none"),
//                     }
                    
//                 },
//                 _=>panic!("not ast::ExpressionStatement"),
//             }
//         },
//         _=> panic!("program.statment[0] is not ast::Expr"),
//     }
// }

// #[derive(Debug)]
// struct TestPrefix{
//     input: String,
//     operator:String,
//     intValue:i64,
// }
// impl TestPrefix{
//     fn new<S: Into<String>>(input: S,operator:S,intValue:i64)->Self{
//         TestPrefix{
//             input: input.into(), 
//             operator:operator.into(),
//             intValue,
//         }
//     }
// }
// #[test]
// fn test_prefix_expr(){
//     let pretests = vec![
//         TestPrefix::new("!5", "!", 5),
//         TestPrefix::new("-15", "-", 15),
//     ]; 
//     for i in pretests{
//         let l = lexer::Lexer::new(i.input);
//         let mut p = parser::Parser::new(l);
//         let program = p.parse_program().unwrap();
        
//         check_parse_error(&p);
//         if program.statements.len() != 1 {
//             panic!("program = {:?}",program.statements);
//         }
//         match &program.statements[0] {
//             Some(stmt)=>{
//                 match stmt.as_any().downcast_ref::<ast::ExpressionStatement>(){
//                     Some(expr_stmt)=>{
//                         match &expr_stmt.expression{
//                             Some(ls)=> {
//                                 match ls.as_any().downcast_ref::<ast::PrefixExpression>(){
//                                     Some(b)=>{
                                
//                                         assert_eq!(b.operator, i.operator);
//                                         test_int_literal(b.right.as_ref().unwrap(), i.intValue);
//                                     },
//                                     _=>panic!("not letstmt, has {:#?}",program.statements),
//                                 }
//                             },
//                             _=>panic!("expression is none"),
//                         }
//                     },
//                     _=>panic!("not ast::ExpressionStatement"),
//                 }
//             },
//             _=> panic!("program.statment[0] is not ast::Expr"),
//         }
//     }
// }

// fn test_int_literal(il:& Box<dyn ast::Node>, value:i64) {
//     match il.as_any().downcast_ref::<ast::IntegerLiteral>(){
//         Some(b)=>{
    
//             assert_eq!(b.value, value);
//             assert_eq!(b.token_literal(), format!("{}",value)); 
//         },
//         _=>panic!("not ast::IntegerLiteral, has {:#?}",il),
//     }
// }


// #[derive(Debug)]
// struct TestInfix{
//     input: String,
//     operator:String,
//     left_val:i64,
//     right_val:i64,
// }
// impl TestInfix{
//     fn new<S: Into<String>>(input: S,operator:S,lv:i64,rv:i64)->Self{
//         TestInfix{
//             input: input.into(), 
//             operator:operator.into(),
//             left_val:lv,
//             right_val:rv,
//         }
//     }
// }

// #[test]
// fn test_infix_expr(){
//     let test =vec![
//         TestInfix::new("5*5;", "*", 5, 5),
//         TestInfix::new("5/5;", "/", 5, 5),
//         TestInfix::new("5>5;", ">", 5, 5),
//         TestInfix::new("5<5;", "<", 5, 5),
//         TestInfix::new("5 + 5;", "+", 5, 5),
//         TestInfix::new("5-5;", "-", 5, 5),
        
        
//         TestInfix::new("5==5;", "==", 5, 5),
//         TestInfix::new("5!=5;", "!=", 5, 5),
//     ];
//     for i in test{
//         let l = lexer::Lexer::new(i.input);
//         let mut p = parser::Parser::new(l);
//         let program = p.parse_program().unwrap();
        
//         check_parse_error(&p);
//         if program.statements.len() != 1 {
//             panic!("program = {:#?}",program.statements);
//         }
    
//         match &program.statements[0] {
//             Some(stmt)=>{
//                 match stmt.as_any().downcast_ref::<ast::ExpressionStatement>(){
//                     Some(expr_stmt)=>{
//                         match &expr_stmt.expression{
//                             Some(ls)=> {
//                                 match ls.as_any().downcast_ref::<ast::InfixExpression>(){
//                                     Some(b)=>{
                                
//                                         assert_eq!(b.operator, i.operator);
//                                         test_int_literal(b.left.as_ref().unwrap(), i.left_val);
//                                     },
//                                     _=>panic!("not letstmt, has {:#?}",program.statements),
//                                 }
//                             },
//                             _=>panic!("expression is none"),
//                         }
//                     },
//                     _=>panic!("not ast::ExpressionStatement"),
//                 }
//             },
//             _=> panic!("program.statment[0] is not ast::Expr"),
//         }
//     }
// }


// #[test]
// fn test_operator_precedence() { 
//     let tests = vec![
//         ("true","true"),
//         ("false","false"),
//         ("3>5==false","((3 > 5) == false)"),
//         ("3<5==false","((3 < 5) == false)"),
//         // "-a*b",
//         // "!-a",
//         // "a+b+c",
//         // "a+b-c",
//         // "a*b*c",
//         // "a*b/c",
//         // "a+b/c",
//         // "a+b*c+d/e-f",
//         // "3+4; -5*5",
//         // "5>4==3<4",
//         // "3+4*5==3*1+4*5",
//         // "3+4*5==3*1+4*5",
//     ];
//     for (a,b) in tests{
//         let l = lexer::Lexer::new(a);
//         let mut p = parser::Parser::new(l);
//         let program = p.parse_program().unwrap();
        
//         check_parse_error(&p);
    
//     }
// }

// // #[test]
// // fn test_boolean() {
// //     let tests = vec![
// //         ("!true","!",true),
// //         ("!false","!",false),
// //     ];
// //     for (a,b,c) in tests{
// //         let l = lexer::Lexer::new(a);
// //         let mut p = parser::Parser::new(l);
// //         let program = p.parse_program().unwrap();
        
// //         check_parse_error(&p);
// //         let actual = program.node_to_string();
// //         panic!("{}", actual);
// //         // assert_eq!(actual.parse::<bool>(),Ok(c));
// //     }
// // }

// #[test]
// fn test_ifexpression(){
//     let input = "if (x<y) {x}";

//     let l = lexer::Lexer::new(input);
//     let mut p = parser::Parser::new(l);
//     let program = p.parse_program().unwrap();

//     check_parse_error(&p);

//     if program.statements.len() != 1 {
//         panic!("program = {:?}",program.statements);
//     }
    
//     match &program.statements[0] {
//         Some(stmt)=>{
//             match stmt.as_any().downcast_ref::<ast::ExpressionStatement>(){
//                 Some(expr_stmt)=>{
//                     match &expr_stmt.expression{
//                         Some(ls)=> {
//                             match ls.as_any().downcast_ref::<ast::IfExpression>(){
//                                 Some(b)=>{
//                                     // panic!("{:#?}",b);
                                    
//                                 },
//                                 _=>panic!("not letstmt, has {:#?}",program.statements),
//                             }
//                         },
//                         _=>panic!("expression is none"),
//                     }
//                 },
//                 _=>panic!("not ast::ExpressionStatement"),
//             }
//         },
//         _=> panic!("program.statment[0] is not ast::Expr"),
//     }
// }

// // fn test_infix_expression<T, S:Into<String>>(exp: Option<Box<dyn ast::Expression>>, left: T,  operator: S, right: T) {
    
// //     if let Some(exp) = exp {
// //         match exp.as_any().downcast_ref::<ast::InfixExpression>(){
// //             Some(infix_expr)=>{
// //                 assert_eq!(infix_expr.operator, operator.into());
// //             },
// //             _=>panic!("not ast::IfExpression"),
// //         }
// //     }
    
// // }

