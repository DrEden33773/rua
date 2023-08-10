//! # VM (stack-based)
//!
//! The virtual machine implementation of rua.
//!
//! ## Note
//!
//! Original C-lua implementation adapt a `register-based` vm.
//!
//! But in rua, we use a `stack-based` vm instead.

use crate::{bytecode::ByteCode, parse::ParseProto, table::Table, utils::New, value::Value};
use std::{cell::RefCell, collections::HashMap, io::Read, rc::Rc};

pub mod lib;

use self::lib::io::lib_print;

pub struct ExeState {
  /// A hashtable of global variables
  globals: HashMap<String, Value>,
  /// Stack of values => core component of vm
  stack: Vec<Value>,
  /// The index of called func (in stack)
  func_index: usize,
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
  pub fn execute<R: Read>(&mut self, proto: &ParseProto<R>) {
    for code in proto.bytecodes.iter() {
      match *code {
        ByteCode::GetGlobal(dst, name) => {
          let name: &str = (&proto.constants[name as usize]).into();
          let value = self.globals.get(name).unwrap_or(&Value::Nil).to_owned();
          self.set_stack(dst, value);
        }
        ByteCode::SetGlobal(name, src) => {
          let name = &proto.constants[name as usize];
          let value = self.stack[src as usize].to_owned();
          self.globals.insert(name.into(), value);
        }
        ByteCode::SetGlobalConst(name, src) => {
          let name = &proto.constants[name as usize];
          let value = proto.constants[src as usize].to_owned();
          self.globals.insert(name.into(), value);
        }
        ByteCode::SetGlobalGlobal(name, src) => {
          let name = &proto.constants[name as usize];
          let src: &str = (&proto.constants[src as usize]).into();
          let value = self.globals.get(src).unwrap_or(&Value::Nil).to_owned();
          self.globals.insert(name.into(), value);
        }
        ByteCode::Call(func, _) => {
          self.func_index = func as usize;
          let func = &self.stack[self.func_index];
          if let Value::Function(func) = func {
            func(self);
          } else {
            panic!("invalid function: {:?}", func);
          }
        }
        ByteCode::Move(dst, src) => {
          let value = self.stack[src as usize].to_owned();
          self.set_stack(dst, value);
        }
        ByteCode::LoadConst(dst, index) => {
          let value = proto.constants[index as usize].to_owned();
          self.set_stack(dst, value);
        }
        ByteCode::LoadNil(dst) => self.set_stack(dst, Value::Nil),
        ByteCode::LoadBool(dst, b) => self.set_stack(dst, Value::Boolean(b)),
        ByteCode::LoadInt(dst, i_16) => self.set_stack(dst, Value::Integer(i_16 as i64)),
        ByteCode::NewTable(dst, array_size, map_size) => {
          let table = Table::new(array_size as usize, map_size as usize);
          self.set_stack(dst, Value::Table(Rc::new(RefCell::new(table))));
        }
        ByteCode::SetTable(table, k, v) => {
          // key is a variable
          let k = self.stack[k as usize].clone();
          let v = self.stack[v as usize].clone();
          if let Value::Table(table) = &self.stack[table as usize] {
            table.borrow_mut().map.insert(k, v);
          } else {
            panic!("not table");
          }
        }
        ByteCode::SetField(table, k, v) => {
          // key is a constant
          let k = proto.constants[k as usize].clone();
          let v = self.stack[v as usize].clone();
          if let Value::Table(table) = &self.stack[table as usize] {
            table.borrow_mut().map.insert(k, v);
          } else {
            panic!("not table");
          }
        }
        ByteCode::SetList(table, n) => {
          let value_index = table as usize + 1;
          if let Value::Table(table) = self.stack[table as usize].clone() {
            // `drain + extend` could be much better than `for + push`
            let values = self.stack.drain(value_index..value_index + n as usize);
            table.borrow_mut().array.extend(values);
          } else {
            panic!("not table");
          }
        }
      }
    }
  }
}

impl New for ExeState {
  type Output = Self;
  fn new() -> Self::Output {
    let globals_vec = vec![("print", Value::Function(lib_print))];
    let mut globals = HashMap::new();
    for (k, v) in globals_vec {
      globals.insert(k.to_owned(), v.to_owned());
    }
    Self {
      globals,
      stack: Vec::new(),
      func_index: 0,
    }
  }
}

impl Default for ExeState {
  fn default() -> Self {
    Self::new()
  }
}
