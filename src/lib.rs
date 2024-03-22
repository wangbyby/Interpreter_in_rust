#[macro_use]
extern crate lazy_static;
pub mod ast;
pub mod evaluator;
pub mod mylexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

use std::{error, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FullError {
    #[error("parse int error")]
    IntErr(#[from] std::num::ParseIntError),
    #[error("parse group error")]
    GroupErr,
    #[error("parse if error")]
    IfErr,
    #[error("parse func error")]
    FuncErr,
    #[error("parse Index error")]
    IndexErr,
    #[error("parse Hash error")]
    HashErr,
    #[error("parse Assign error")]
    AssignErr,
    #[error("parse let error")]
    LetErr,
    #[error("parse return error")]
    RetErr,
    #[error("parse expr error")]
    ExpressionErr,
    #[error("unknown data store error")]
    Unknown,
}

type Result<T> = std::result::Result<T, FullError>;
