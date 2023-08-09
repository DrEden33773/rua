//! # Value
//!
//! Definition of `Value` type of rua.

use crate::vm::ExeState;
use core::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  Function(fn(&mut ExeState) -> i32),
}

impl Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Integer(i) => write!(f, "{i}"),
      Value::Float(n) => write!(f, "{:?}", n),
      Value::String(s) => write!(f, "{s}"),
      Value::Function(_) => write!(f, "function"),
    }
  }
}
