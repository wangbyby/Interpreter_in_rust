#[macro_use]
extern crate lazy_static;
pub mod ast;
pub mod evaluator;
pub mod mylexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FullError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

type Result<T> = std::result::Result<T, FullError>;
