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

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Boolean(l0), Self::Boolean(r0)) => *l0 == *r0,
      (Self::Integer(l0), Self::Integer(r0)) => *l0 == *r0,
      (Self::Float(l0), Self::Float(r0)) => *l0 == *r0,
      (Self::String(l0), Self::String(r0)) => *l0 == *r0,
      (Self::Function(l0), Self::Function(r0)) => std::ptr::eq(l0, r0),
      (Value::Nil, Value::Nil) => true,
      _ => false,
    }
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Integer(i) => write!(f, "{i}"),
      Value::Float(n) => write!(f, "{:?}", n),
      Value::String(s) => write!(f, "'{s}'"),
      Value::Function(_) => write!(f, "<function>"),
    }
  }
}
