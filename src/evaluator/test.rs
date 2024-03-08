
use std::collections::HashMap;

use super::evaluator::eval;
use super::evaluator;
use crate::object::object;
use crate::parser::parser;
use crate::mylexer::lexer;

fn test_eval<S: Into<String>>(input: S) ->Box<object::TheObject>{
    let mut env = evaluator::Environment::new();
    let l = lexer::Lexer::new(input.into());
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    eval(&program, &mut env)
}


fn test_int_obj(obj:Box<object::TheObject>, expected:i64){
    match obj.as_ref() {
        object::TheObject::Integer(i)=> assert_eq!(*i, expected),
        _=> panic!("not IntObject ... but is {:#?}",obj.as_ref()),
    }
}

fn test_bool_obj(obj:Box<object::TheObject>, expected:bool){
    match obj.as_ref() {
        object::TheObject::Boolean(b)=> assert_eq!(*b, expected),
        _=> panic!("not bool ... but  is {:#?}",obj.as_ref()),
    }
}

#[test]
fn test_eval_int_expr() {
    let tests = vec![
        ("10",10),
        ("5",5),
        ("-15",-15),
        ("-5",-5),
        ("5+5+5+5-10",10),
        ("2*2*2*2*2",32),
        ("-50+100+-50",0),
        ("5*2+10",20),
        ("5+2*10",25),
        ("20+2*-10",0),
        ("50/2*2+10",60),
        ("2*(5+10)",30),
        ("3*3*3+10",37),
        ("3*(3*3)+10",37),
        ("(5+10*2+15/3)*2+-10",50),

    ];

    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_int_obj(evaled,expected);
    }
}

#[test]
fn test_eval_bool_expr() {
    let tests = vec![
        ("true",true),
        ("false",false),
        ("1 <   2",true),
        ("1 >   2",false),
        ("1 >   1",false),
        ("1 <   1",false),
        ("1 ==   1",true),
        ("1 !=   1",false),
        ("1 ==   2",false),
        ("1 !=   2",true),
        ("true == true",true),
        ("true != true",false),
        ("false == false",true),
        ("false != false",false),
        ("true != false",true),
        ("false != true",true),
        ("!true",false),
        ("!false",true),
        ("!5",false),
        ("!!true",true),
        ("!!false",false),
        ("!!5",true),

        ];

    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_bool_obj(evaled,expected);
    }
}

#[test]
fn test_ifelse() {
    let tests = vec![
        ("if (true){10}",10),
        ("if (1){10}",10),
        ("if (1<2){10}",10),
        ("if (2){10}",10),
        ("if (1>2){10} else{20}",20),
        ("if (1<2){10} else{20}",10),
    ];
    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_int_obj(evaled,expected);

    }

}

// #[test]
// fn test_error_handler() { //改了报错内容
//     let tests = vec![
//         (" if(10>1){
//             if(10>1){
//                 return true+false;
//             }
//             return 1;
//         }","unknown operator: BOOLEAN + BOOLEAN" ),
//         ("5+true","type dismatch: INTEGER + BOOLEAN" ),
//         ("5+true; 5;","type dismatch: INTEGER + BOOLEAN" ),
//         ("-true", "unknown operator: -BOOLEAN"),
//         ("-true;", "unknown operator: -BOOLEAN"),
//         ("false+true", "unknown operator: BOOLEAN + BOOLEAN"),
//     ];
//     for (input, expected) in tests{
//         let evaled = test_eval(input);

//         match evaled.as_ref().type_of(){
//             object::ERROR_OBJ=>assert_eq!(evaled.as_ref().inspect(),expected),
//             _=>panic!("not errors, {:?}",evaled.as_ref()),
//         }
//     }
// }


#[test]
fn test_let_statment() {
    let tests = vec![
        ("let a=10;a;",10),
        ("let a=5*5;a;",25),
        ("let a=10;let b=a;b;",10),
        ("let a=5;let b=a;let c = a+b+5;c;",15),
    ];
    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_int_obj(evaled,expected);

    }
}

#[test]
fn test_return_stmt() {
    let tests = vec![
        (" if(10>1){
            if(10>1){
                return 10;
            }
            return 1;
        }
        ",10),
    ];
    
    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_int_obj(evaled,expected);

    }
}

#[test]
fn test_func_obj() {
    let input = "fn(x){x+2;};";
    
    let evaled = test_eval(input);
    match evaled.as_ref(){
        object::TheObject::Func(ref params,ref body, ref env)=>{
            assert_eq!(1,params.len());
            assert_eq!("x".to_string(), params[0].clone().unwrap().as_ref().value);
            // panic!("body = {:#?}", body);
        }, 
        _=>panic!("not Func {:#?}",evaled.as_ref()),
    }
    
}




#[test]
fn test_func_apply() {
    let tests = vec![
    ("let ident=fn(x){x;}; ident(5);",5),
    ("fn(x){x;}(5)",5),
    ("let ident=fn(x){return x;}; ident(5);",5),
    ("let double=fn(x){return 2*x;}; double(5);",10),
    ("let add=fn(x,y){return x+y;}; add(5,5);",10),
    ("let add=fn(x,y){return x+y;}; add(5+5,add(5,5));",20),
        //这里有一点小bug fn(x,y){x+y;} 不识别
    ("
        let newadd = fn(x){fn(y){x+y}; };
        let addTwo = newadd(2);
        addTwo(2);
    ",4)    
    ];
    for (input, expected) in tests{
        let evaled = test_eval(input);
        test_int_obj(evaled,expected);
    }
}