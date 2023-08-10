//! # Value
//!
//! Definition of `Value` type of rua.

use crate::vm::ExeState;
use core::fmt;
use std::{fmt::Debug, rc::Rc};

/// sizeof(Value) - 1(tag) - 1(len)
const SHORT_STR_MAX: usize = 16 - 1 - 1;
/// 64 - sizeof(value)
const MID_STR_MAX: usize = 48 - 1;

#[derive(Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(String),
  ShortStr(u8, [u8; SHORT_STR_MAX]),
  MidStr(Rc<(u8, [u8; MID_STR_MAX])>),
  LongStr(Rc<Vec<u8>>),
  Function(fn(&mut ExeState) -> i32),
}

impl From<()> for Value {
  fn from(_: ()) -> Self {
    Self::Nil
  }
}
impl From<String> for Value {
  fn from(v: String) -> Self {
    Self::String(v)
  }
}
impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}
impl From<i64> for Value {
  fn from(value: i64) -> Self {
    Self::Integer(value)
  }
}
impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Self::Float(value)
  }
}
impl From<fn(&mut ExeState) -> i32> for Value {
  fn from(value: fn(&mut ExeState) -> i32) -> Self {
    Self::Function(value)
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Boolean(l0), Self::Boolean(r0)) => *l0 == *r0,
      (Self::Integer(l0), Self::Integer(r0)) => *l0 == *r0,
      (Self::Float(l0), Self::Float(r0)) => *l0 == *r0,
      (Self::Function(l0), Self::Function(r0)) => std::ptr::eq(l0, r0),
      (Self::String(l0), Self::String(r0)) => *l0 == *r0,
      (Self::ShortStr(len0, s0), Self::ShortStr(len1, s1)) => {
        s0[..*len0 as usize] == s1[..*len1 as usize]
      }
      (Self::MidStr(s0), Self::MidStr(s1)) => s0.1[..s0.0 as usize] == s1.1[..s1.0 as usize],
      (Self::LongStr(s0), Self::LongStr(s1)) => *s0 == *s1,
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
      Value::ShortStr(len, buf) => {
        write!(
          f,
          "'{s}'",
          s = String::from_utf8_lossy(&buf[..*len as usize])
        )
      }
      Value::MidStr(s) => write!(
        f,
        "'{s}'",
        s = String::from_utf8_lossy(&s.1[..s.0 as usize])
      ),
      Value::LongStr(s) => write!(f, "'{s}'", s = String::from_utf8_lossy(&s[..])),
      Value::Function(_) => write!(f, "<function>"),
    }
  }
}
