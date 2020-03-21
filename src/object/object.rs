use crate::ast::ast;
use crate::evaluator::evaluator::Environment;

use std::collections::HashMap;

type ObjectType= &'static str;

pub const INTEGER_OBJ:ObjectType = "INTEGER";
pub const BOOLEAN_OBJ: ObjectType = "BOOLEAN";
pub const NULL_OBJ: ObjectType = "NULL";
pub const RETURN_VALUE_OBJ:ObjectType = "RETURN_VALUE";
pub const ERROR_OBJ: ObjectType = "ERROR";
pub const FUNCTION_OBJ: ObjectType = "FUNCTION";
pub const STRING_OBJ: ObjectType = "STRING";
pub const ARRAY_OBJ: ObjectType = "ARRAY";
pub const MAP_OBJ: ObjectType = "MAP";

#[derive(Debug,Clone,PartialEq)]
pub enum TheObject {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<TheObject>),
    Errors(String),
    Func(Vec<Option<Box<ast::Identifier>>>, Box<ast::ASTNode>,  Environment),
    Stringobj(String),
    Array(Vec<Box<TheObject>>),
    Map(HashMap<String,Box<TheObject>>),
    NULL,
}

impl TheObject {
    pub fn default()->Self{
        TheObject::NULL
    }
    pub fn type_of(&self)->ObjectType{
        use self::TheObject::*;
        match self {
            Integer(_)=> INTEGER_OBJ,
            Boolean(_)=> BOOLEAN_OBJ,
            ReturnValue(_)=> RETURN_VALUE_OBJ,
            Errors(_)=> ERROR_OBJ,
            Func(_,_,_)=>FUNCTION_OBJ,
            Stringobj(_)=>STRING_OBJ,
            Array(_)=>ARRAY_OBJ,
            Map(_)=>MAP_OBJ,
            NULL => NULL_OBJ,
        }
    }

    pub fn inspect(&self)->String {
        use self::TheObject::*;
        match self{
            Integer(i)=> format!("{}",*i),
            Boolean(i)=> format!("{}",*i),
            ReturnValue(i)=> format!("{}",i.as_ref().inspect()),
            Errors(i)=> i.clone(),
            Func(_,_,_) => "".to_string(), //format!("env = {:#?} fn({:#?}){}\n{:#?}\n{} ",env,ident, "{",block,"}"),
            Stringobj(ref s)=>s.clone(),
            Array(ref s)=>format!("{:#?}",s),
            Map(ref s)=>format!("{:#?}",s),
            NULL => format!("()"),
        }
    }

    pub fn is_error(&self)->bool{
        self.type_of() == ERROR_OBJ
    }
}
