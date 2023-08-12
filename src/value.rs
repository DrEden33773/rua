//! # Value
//!
//! Definition of `Value` type of rua.

use crate::{table::Table, vm::ExeState};
use core::fmt;
use gc::Gc;
use std::{
  cell::RefCell,
  fmt::{Debug, Display},
  hash::Hash,
  rc::Rc,
  str,
};

/// sizeof(Value) - 1(tag) - 1(len)
const SHORT_STR_MAX: usize = 16 - 1 - 1;
/// 64 - sizeof(value)
const MID_STR_MAX: usize = 48 - 1;

fn slice_to_short_mid_str(v: &[u8]) -> Option<Value> {
  let len = v.len();
  match len {
    l if l <= SHORT_STR_MAX => {
      let mut buf = [0; SHORT_STR_MAX];
      buf[..len].copy_from_slice(v);
      Some(Value::ShortStr(l as u8, buf))
    }
    l if l <= MID_STR_MAX => {
      let mut buf = [0; MID_STR_MAX];
      buf[..len].copy_from_slice(v);
      Some(Value::MidStr(Gc::new((l as u8, buf))))
    }
    _ => None,
  }
}

#[derive(Clone)]
pub enum Value {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  ShortStr(u8, [u8; SHORT_STR_MAX]),
  MidStr(Gc<(u8, [u8; MID_STR_MAX])>),
  LongStr(Gc<Vec<u8>>),
  Function(fn(&mut ExeState) -> i32),
  Table(Rc<RefCell<Table>>),
}

impl Hash for Value {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Value::Nil => (),
      Value::Boolean(b) => b.hash(state),
      Value::Integer(i) => i.hash(state),
      Value::Float(f) => {
        // TODO: try to convert to integer
        (f).to_bits().hash(state)
      }
      Value::ShortStr(len, buf) => buf[..*len as usize].hash(state),
      Value::MidStr(s) => s.1[..s.0 as usize].hash(state),
      Value::LongStr(s) => s.hash(state),
      Value::Function(f) => (*f as *const usize).hash(state),
      Value::Table(t) => Rc::as_ptr(t).hash(state),
    }
  }
}

impl From<&[u8]> for Value {
  fn from(v: &[u8]) -> Self {
    slice_to_short_mid_str(v).unwrap_or(Value::LongStr(Gc::new(v.to_vec())))
  }
}
impl From<&str> for Value {
  fn from(s: &str) -> Self {
    s.as_bytes().into()
  }
}
impl From<Vec<u8>> for Value {
  fn from(v: Vec<u8>) -> Self {
    slice_to_short_mid_str(&v).unwrap_or(Value::LongStr(Gc::new(v.to_vec())))
  }
}
impl From<String> for Value {
  fn from(s: String) -> Self {
    s.into_bytes().into()
  }
}

impl<'a> From<&'a Value> for &'a [u8] {
  fn from(v: &'a Value) -> Self {
    match v {
      Value::ShortStr(len, buf) => &buf[..*len as usize],
      Value::MidStr(s) => &s.1[..s.0 as usize],
      Value::LongStr(s) => s,
      _ => panic!("invalid string Value"),
    }
  }
}
impl<'a> From<&'a Value> for &'a str {
  fn from(v: &'a Value) -> Self {
    str::from_utf8(v.into()).unwrap()
  }
}
impl From<&Value> for String {
  fn from(v: &Value) -> Self {
    String::from_utf8_lossy(v.into()).to_string()
  }
}

impl From<()> for Value {
  fn from(_: ()) -> Self {
    Self::Nil
  }
}
impl From<bool> for Value {
  fn from(b: bool) -> Self {
    Self::Boolean(b)
  }
}
impl From<i64> for Value {
  fn from(i: i64) -> Self {
    Self::Integer(i)
  }
}
impl From<f64> for Value {
  fn from(f: f64) -> Self {
    Self::Float(f)
  }
}
impl From<fn(&mut ExeState) -> i32> for Value {
  fn from(func: fn(&mut ExeState) -> i32) -> Self {
    Self::Function(func)
  }
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Nil, Value::Nil) => true,
      (Self::Boolean(l0), Self::Boolean(r0)) => *l0 == *r0,
      (Self::Integer(l0), Self::Integer(r0)) => *l0 == *r0,
      (Self::Float(l0), Self::Float(r0)) => *l0 == *r0,
      (Self::ShortStr(len0, s0), Self::ShortStr(len1, s1)) => {
        s0[..*len0 as usize] == s1[..*len1 as usize]
      }
      (Self::MidStr(s0), Self::MidStr(s1)) => s0.1[..s0.0 as usize] == s1.1[..s1.0 as usize],
      (Self::LongStr(s0), Self::LongStr(s1)) => *s0 == *s1,
      (Self::Function(l0), Self::Function(r0)) => std::ptr::eq(l0, r0),
      // TODO: detailed logic of comparing two `Table` objects
      (Self::Table(l0), Self::Table(r0)) => std::ptr::eq(l0, r0),
      _ => false,
    }
  }
}

impl Eq for Value {}

impl Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Integer(i) => write!(f, "{i}"),
      Value::Float(n) => write!(f, "{:?}", n),
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
      Value::Table(t) => {
        let t = t.borrow();
        write!(f, "{}", t)
      }
    }
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Table(t) => write!(f, "{}", t.borrow()),
      _ => write!(f, "{:?}", self),
    }
  }
}
