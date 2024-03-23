use std::collections::HashMap;

use crate::ast::ast;
use crate::object::object;
use crate::object::object::TheObject;

macro_rules! new_box {
    ($b:expr) => {
        Box::new($b)
    };
}

macro_rules! new_int {
    ($b:expr) => {
        new_box!(TheObject::Integer($b))
    };
}

macro_rules! theobject_null {
    () => {
        new_box!(TheObject::NULL)
    };
}

const TRUE: object::TheObject = object::TheObject::Boolean(true);
const FALSE: object::TheObject = object::TheObject::Boolean(false);

pub fn eval(node: &ast::ASTNode, mut env: &mut Environment) -> Box<object::TheObject> {
    use crate::ast::ast::ASTNode::*;

    match node {
        //关键的一行代码
        Program(ref value) => return eval_program(value, &mut env),
        ExpressionStatement(ref value) => return eval(&value.expression, &mut env),
        IntegerLiteral(ref value) => return new_box!(object::TheObject::Integer(value.value)),
        Boolean(ref boolean_obj) => return bool_obj(boolean_obj.value),
        PrefixExpression(ref value) => {
            return eval_prefix_expr(&value.operator, &eval(&value.right, &mut env))
        }
        InfixExpression(ref value) => {
            return eval_infix_expr(
                &value.operator,
                &eval(&value.left, &mut env),
                &eval(&value.right, &mut env),
            )
        }
        BlockStatement(ref value) => return eval_statements(&value.statements, &mut env),
        IfExpression(ref value) => return eval_ifexpression(value, &mut env),
        Identifier(ref value) => return eval_identifier(value, &env),
        LetStatement(ref value) => {
            let val = eval(&value.value, &mut env);
            if val.is_error() {
                return val;
            }
            env.store.insert(value.name.value.clone(), val.clone());
            return eval_identifier(&value.name, &env);
        }
        ReturnStatement(ref value) => {
            let val = eval(&value.return_value, env);
            if val.as_ref().is_error() {
                return val;
            }
            return new_box!(object::TheObject::ReturnValue(val));
        }
        FuncLiteral(ref val) => {
            return new_box!(object::TheObject::Func(
                val.params.clone(),
                val.body.clone(),
                env.clone()
            ))
        }
        CallExpression(ref value) => {
            let function = eval(&value.func, &mut env); //函数对象
            if function.as_ref().is_error() {
                return function;
            }

            //函数参数列表
            let args = eval_expressions(&value.args, &mut env);
            if args.len() == 1 && args[0].as_ref().is_error() {
                return args[0].clone();
            }
            return apply_function(&function, &args);
        }
        StringLiteral(_, ref value) => {
            return new_box!(object::TheObject::Stringobj(value.clone()))
        }
        ArrayLiteral(_, ref values) => {
            return new_box!(object::TheObject::Array(eval_expressions(
                &values, &mut env
            )))
        }
        IndexLiteral(_, ref left, ref right) => {
            let left = eval(&left, &mut env);
            let index = eval(&right, &mut env);

            return eval_index_expression(&left, &index);
        }
        HashLiteral(_, ref pairs) => {
            use crate::object::object::TheObject::*;

            let mut map: HashMap<String, Box<object::TheObject>> = HashMap::new();
            // let map : HashMap<String,Box<object::TheObject>> = pairs.iter()
            //                                                         .map(|(k,v)| (format!("{:?}",eval(k,&mut env)) , eval(v,&mut env)))
            //                                                         .collect();
            // let error: Vec<&Box<object::TheObject>> = map.iter()
            //                                                 .filter(|(_,v)| v.as_ref().is_error())
            //                                                 .map(|(_,v)| v)
            //                                                 .collect();
            // if error.len() != 0 {
            //     return error[0].clone();
            // }
            for (k, v) in pairs.iter() {
                let key = eval(k, &mut env);
                if key.as_ref().is_error() {
                    return new_error(format!("{:#?} is not a key type", key.as_ref()));
                }
                //key ==> String
                let key_str: String = format!("{:#?}", key.as_ref());

                let value = eval(v, &mut env);
                if value.as_ref().is_error() {
                    return new_error(format!("{:#?} is not a key type", value.as_ref()));
                }

                map.insert(key_str, value);
            }
            return new_box!(Map(map));
        }
        _ => return theobject_null!(),
    }
    theobject_null!()
}

fn apply_function(
    function: &Box<object::TheObject>,
    call_args: &Vec<Box<object::TheObject>>,
) -> Box<object::TheObject> {
    match function.as_ref() {
        object::TheObject::Func(ref params, ref body, ref env) => {
            let mut extended_env = Environment::new();
            extended_env.outer = Some(Box::new(env.clone()));

            // for (key, value) in call_args.iter().enumerate() {
            //     extended_env.store.insert(params[key].as_ref().unwrap().as_ref().value.clone(), value.clone());//问题?
            // }

            //函数调用参数加入环境
            call_args.iter().enumerate().for_each(|(index, value)| {
                extended_env.store.insert(
                    params[index].as_ref().unwrap().as_ref().value.clone(),
                    value.clone(),
                ); //问题?
            });

            //eval函数体
            let evaled = eval(body, &mut extended_env);

            match evaled.as_ref() {
                object::TheObject::ReturnValue(ref value) => return value.clone(),
                _ => return evaled,
            }
        }
        _ => new_error(format!(
            "not a function, is {}",
            function.as_ref().type_of()
        )),
    }
}

fn eval_expressions(
    exprs: &Vec<Box<ast::ASTNode>>,
    mut env: &mut Environment,
) -> Vec<Box<object::TheObject>> {
    let mut res = vec![];
    for i in exprs {
        let evaled = eval(&i, &mut env);

        if evaled.as_ref().is_error() {
            return vec![evaled];
        }
        res.push(evaled);
    }
    res
}

fn bool_obj(b: bool) -> Box<object::TheObject> {
    if b {
        new_box!(TRUE)
    } else {
        new_box!(FALSE)
    }
}

fn eval_identifier(node: &ast::Identifier, env: &Environment) -> Box<object::TheObject> {
    let err = new_error(format!(
        "identifier name not found: {}, \n and the environment is {:?}",
        node.value, env
    ));

    env.get(&node.value).unwrap_or(err)
}

fn eval_ifexpression(ie: &ast::IfExpression, mut env: &mut Environment) -> Box<object::TheObject> {
    let condition = eval(&ie.condition, &mut env);
    if condition.as_ref().is_error() {
        return condition;
    }
    if is_truthy(condition.as_ref()) {
        return eval(&ie.consequence, &mut env);
    } else if ie.alternative.as_ref().is_some() {
        return eval(&ie.alternative, &mut env);
    } else {
        return theobject_null!();
    }
}
fn is_truthy(obj: &object::TheObject) -> bool {
    match obj {
        TheObject::Boolean(b) => *b,
        TheObject::NULL => false,
        _ => true,
    }
}

fn eval_infix_expr(
    operator: &str,
    left: &Box<object::TheObject>,
    right: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    if left.type_of() == object::INTEGER_OBJ && right.type_of() == object::INTEGER_OBJ {
        return eval_integer_infix_expression(operator, left, right);
    } else if right.type_of() == object::STRING_OBJ && left.type_of() == object::STRING_OBJ {
        return eval_string_infix_expression(operator, left, right);
    } else if left.as_ref().type_of() != right.as_ref().type_of() {
        return new_error(format!(
            "type dismatch: {:#?} {} {:#?}",
            left, operator, right
        ));
    } else if operator == "==" || operator == "!=" {
        return cmp_boolobject(operator, left, right);
    } else {
        return new_error(format!(
            "unknown operator: {} {} {}",
            left.as_ref().type_of(),
            operator,
            right.as_ref().type_of()
        ));
    }
}

fn eval_string_infix_expression(
    operator: &str,
    left: &Box<object::TheObject>,
    right: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    if operator != "+" {
        return new_error(format!(
            "unknown operator: {} {} {}",
            left.as_ref().type_of(),
            operator,
            right.as_ref().type_of()
        ));
    }
    match (left.as_ref(), right.as_ref()) {
        (TheObject::Stringobj(a), TheObject::Stringobj(b)) => {
            new_box!(object::TheObject::Stringobj(format!("{}{}", a, b)))
        }
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            left.as_ref().type_of(),
            operator,
            right.as_ref().type_of()
        )),
    }
}

fn cmp_boolobject(
    operator: &str,
    left: &Box<object::TheObject>,
    right: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    match operator {
        "==" => bool_obj(left == right),
        "!=" => bool_obj(left != right),
        _ => theobject_null!(),
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: &Box<object::TheObject>,
    right: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    match (left.as_ref(), right.as_ref()) {
        (TheObject::Integer(a), TheObject::Integer(b)) => match operator {
            "+" => new_int!(a + b),
            "-" => new_int!(a - b),
            "*" => new_int!(a * b),
            "/" => new_int!(a / b),
            "<" => bool_obj(a < b),
            ">" => bool_obj(a > b),
            "==" => bool_obj(a == b),
            "!=" => bool_obj(a != b),
            _ => new_error(format!(
                "unknown operator: {} {} {}",
                left.as_ref().type_of(),
                operator,
                right.as_ref().type_of()
            )),
        },
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            left.as_ref().type_of(),
            operator,
            right.as_ref().type_of()
        )),
    }
}

fn eval_prefix_expr(operator: &str, right: &Box<object::TheObject>) -> Box<object::TheObject> {
    if right.as_ref().is_error() {
        return right.clone();
    }

    match operator {
        "!" => eval_bang_op_expr(right),
        "-" => eval_minus_op_expr(right),
        _ => new_error(format!(
            "unknown operator: {} {}",
            operator,
            right.as_ref().type_of()
        )),
    }
}

//bang : !
fn eval_bang_op_expr(right: &Box<object::TheObject>) -> Box<object::TheObject> {
    match right.as_ref() {
        TheObject::Boolean(b) => bool_obj(!*b),
        TheObject::NULL => bool_obj(true),
        _ => bool_obj(false),
    }
}

fn eval_minus_op_expr(right: &Box<object::TheObject>) -> Box<object::TheObject> {
    match right.as_ref() {
        TheObject::Integer(i) => new_int!(-(*i)),
        _ => new_error(format!("unknown operator: -{}", right.as_ref().type_of())),
    }
}

fn eval_program(program: &ast::Program, mut env: &mut Environment) -> Box<object::TheObject> {
    use crate::object::object::TheObject::*;
    let mut result = Box::new(object::TheObject::default());
    for i in &program.statements {
        result = eval(i, &mut env);
        //TODO
        match result.as_ref() {
            ReturnValue(ref value) => return value.clone(),
            Errors(ref value) => return new_error(value.clone()),
            _ => continue,
        }
    }
    result
}

fn eval_statements(
    stmts: &Vec<Box<ast::ASTNode>>,
    mut env: &mut Environment,
) -> Box<object::TheObject> {
    let mut result = Box::new(object::TheObject::default());
    for i in stmts {
        result = eval(i, &mut env);
        match result.as_ref().type_of() {
            object::RETURN_VALUE_OBJ | object::ERROR_OBJ => return result,
            _ => continue,
        }
    }
    result
}

fn eval_index_expression(
    left: &Box<object::TheObject>,
    index: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    if left.as_ref().type_of() == object::ARRAY_OBJ
        && index.as_ref().type_of() == object::INTEGER_OBJ
    {
        eval_array_index_expression(left, index)
    } else if left.as_ref().type_of() == object::MAP_OBJ {
        eval_hash_index_expression(left, index)
    } else {
        new_error(format!(
            "error: left:{:#?}, index:{:#?}",
            left.as_ref(),
            index.as_ref()
        ))
    }
}

fn eval_hash_index_expression(
    map: &Box<object::TheObject>,
    index: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    use crate::object::object::TheObject::*;
    match map.as_ref() {
        Map(ref map) => map
            .get(&format!("{:#?}", index))
            .unwrap_or(&Box::new(NULL))
            .clone(),
        _ => new_error(format!("not a map object")),
    }
}

fn eval_array_index_expression(
    array: &Box<object::TheObject>,
    index: &Box<object::TheObject>,
) -> Box<object::TheObject> {
    use crate::object::object::TheObject::*;
    match (array.as_ref(), index.as_ref()) {
        (Array(ref array), Integer(ref index)) => {
            if *index < 0 || *index > array.len() as i64 {
                return new_error(format!(
                    "index out of bounds length of array is {}, but have {}",
                    array.len(),
                    *index
                ));
            }

            array[*index as usize].clone()
        }
        _ => new_error(format!("there has some error ")),
    }
}

fn new_error<S: Into<String>>(s: S) -> Box<object::TheObject> {
    new_box!(object::TheObject::Errors(s.into()))
}

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub store: HashMap<String, Box<object::TheObject>>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }
    pub fn get(&self, k: &String) -> Option<Box<object::TheObject>> {
        if self.store.get(k).is_some() {
            return Some(self.store.get(k).unwrap().clone());
        } else {
            if self.outer.is_some() {
                return self.outer.clone().unwrap().as_ref().get(k);
            }
            return None;
        }
    }
}
