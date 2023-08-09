//! # VM (stack-based)
//!
//! The virtual machine implementation of rua.
//!
//! ## Note
//!
//! Original C-lua implementation adapt a `register-based` vm.
//!
//! But in rua, we use a `stack-based` vm instead.

use crate::{bytecode::ByteCode, parse::ParseProto, utils::New, value::Value};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub mod lib;

use self::lib::io::lib_print;

static GLOBALS_VEC: Lazy<Vec<(String, Value)>> =
  Lazy::new(|| vec![("print".into(), Value::Function(lib_print))]);

pub struct ExeState {
  /// A hashtable of global variables
  globals: HashMap<String, Value>,
  /// Stack of values => core component of vm
  stack: Vec<Value>,
}

impl ExeState {
  fn set_stack(&mut self, dst: u8, value: Value) {
    let dst = dst as usize;
    match dst.cmp(&self.stack.len()) {
      std::cmp::Ordering::Less => self.stack[dst] = value,
      std::cmp::Ordering::Equal => self.stack.push(value),
      std::cmp::Ordering::Greater => panic!("fail in set_stack, for stack is full"),
    }
  }
}

impl ExeState {
  pub fn execute(&mut self, proto: &ParseProto) {
    for code in proto.bytecodes.iter() {
      match *code {
        ByteCode::GetGlobal(dst, name) => {
          let name = &proto.constants[name as usize];
          if let Value::String(key) = name {
            let value = self.globals.get(key).unwrap_or(&Value::Nil).to_owned();
            self.set_stack(dst, value);
          } else {
            panic!("invalid global key: {:?}", name);
          }
        }
        ByteCode::LoadConst(dst, index) => {
          let value = proto.constants[index as usize].to_owned();
          self.set_stack(dst, value);
        }
        ByteCode::Call(func, _) => {
          let func = &self.stack[func as usize];
          if let Value::Function(func) = func {
            func(self);
          } else {
            panic!("invalid function: {:?}", func);
          }
        }
      }
    }
  }
}

impl New for ExeState {
  type Output = Self;
  fn new() -> Self::Output {
    let mut globals = HashMap::new();
    for (k, v) in GLOBALS_VEC.iter() {
      globals.insert(k.to_owned(), v.to_owned());
    }
    Self {
      globals,
      stack: Vec::new(),
    }
  }
}

impl Default for ExeState {
  fn default() -> Self {
    Self::new()
  }
}
