//! # Parse
//!
//! Parse the bytecode to a AST.
//!
//! ## Goal:
//!
//! TokenStream -> Bytecode

use crate::{
  bytecode::ByteCode,
  lex::{Lex, Token},
  utils::TokenIterator,
  value::Value,
};
use std::{fs::File, vec};

/// ## ParseProto
///
/// A struct that contains all the information of a proto.
///
/// Proto <=> Instructions + Constants (Intermediate)
#[derive(Debug)]
pub struct ParseProto {
  /// Constants vec
  pub constants: Vec<Value>,
  /// Bytecodes/Instructions vec
  pub bytecodes: Vec<ByteCode>,
  /// Local variable pool
  locals: Vec<String>,
  /// Lexing Machine
  lexer: Lex,
}

impl ParseProto {
  pub fn new(input: File) -> Self {
    Self {
      constants: vec![],
      bytecodes: vec![],
      locals: vec![],
      lexer: Lex::new(input),
    }
  }
}

impl ParseProto {
  /// Add a constant into const_table only if the table doesn't contains it.
  ///
  /// Return the index.
  pub fn add_const(&mut self, constant: Value) -> usize {
    let constants = &mut self.constants;
    constants
      .iter()
      .position(|v| *v == constant)
      .unwrap_or_else(|| {
        constants.push(constant);
        constants.len() - 1
      })
  }

  /// Generate bytecode of `LoadConst`, with given `dst` and `correct const_table index`
  /// leading by calling `add_const` with `constant` argument.
  pub fn load_const(&mut self, dst: usize, c: Value) -> ByteCode {
    ByteCode::LoadConst(dst as u8, self.add_const(c) as u8)
  }

  pub fn load_expression(&mut self, dst: usize) {
    let code = match self.lexer.next() {
      Token::Nil => ByteCode::LoadNil(dst as u8),
      Token::True => ByteCode::LoadBool(dst as u8, true),
      Token::False => ByteCode::LoadBool(dst as u8, false),
      Token::Integer(i) => {
        if let Ok(i_16) = i16::try_from(i) {
          // do not need to add current integer into the const table,
          // just move it into the bytecode itself.
          ByteCode::LoadInt(dst as u8, i_16)
        } else {
          self.load_const(dst, Value::Integer(i))
        }
      }
      Token::Float(f) => self.load_const(dst, Value::Float(f)),
      Token::String(s) => self.load_const(dst, Value::String(s)),
      Token::Name(var) => self.load_var(dst, var),
      _ => panic!("invalid argument"),
    };
    self.bytecodes.push(code);
  }

  pub fn load_var(&mut self, dst: usize, name: String) -> ByteCode {
    if let Some(i) = self.locals.iter().rposition(|v| *v == name) {
      // it's a local var
      ByteCode::Move(dst as u8, i as u8)
    } else {
      // it's a global var
      let i = self.add_const(Value::String(name));
      ByteCode::GetGlobal(dst as u8, i as u8)
    }
  }

  fn get_local(&self, name: &str) -> Option<usize> {
    self.locals.iter().rposition(|v| v == name)
  }
}

impl ParseProto {
  /// FuncName(expression) / FuncName LiteralString
  fn function_call(&mut self, name: String) {
    let func_index = self.locals.len();
    let arg_index = func_index + 1;

    // function, var
    let code = self.load_var(func_index, name);
    self.bytecodes.push(code);

    // argument, (expression) or "literal_string"
    match self.lexer.next() {
      // '('
      Token::ParL => {
        // expression
        self.load_expression(arg_index);
        // ')'
        if self.lexer.next() != Token::ParR {
          panic!("expected `)`");
        }
      }
      // "literal_string"
      Token::String(s) => {
        let code = self.load_const(arg_index, Value::String(s));
        self.bytecodes.push(code);
      }
      _ => panic!("expected 'string' or (expression)"),
    }

    self.bytecodes.push(ByteCode::Call(func_index as u8, 1));
  }

  // local name = expression
  fn local_bind(&mut self) {
    let var = if let Token::Name(var) = self.lexer.next() {
      var
    } else {
      panic!("expected variable_name after `local` keyword")
    };

    if self.lexer.next() != Token::Assign {
      panic!("expected `=` after variable_name")
    }

    self.load_expression(self.locals.len());

    // add to locals after load_expression
    self.locals.push(var);
  }

  /// var = var | const
  fn assignment(&mut self, l_var: String) {
    // consume `=`
    self.lexer.next();
    // expression
    if let Some(dst) = self.get_local(&l_var) {
      // l_var := local var
      self.load_expression(dst);
    } else {
      // l_var := global var
      let dst = self.add_const(Value::String(l_var)) as u8;
      let code = match self.lexer.next() {
        // from const values
        Token::Nil => ByteCode::SetGlobalConst(dst, self.add_const(Value::Nil) as u8),
        Token::True => ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(true)) as u8),
        Token::False => ByteCode::SetGlobalConst(dst, self.add_const(Value::Boolean(false)) as u8),
        Token::Integer(i) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Integer(i)) as u8),
        Token::Float(f) => ByteCode::SetGlobalConst(dst, self.add_const(Value::Float(f)) as u8),
        Token::String(s) => ByteCode::SetGlobalConst(dst, self.add_const(Value::String(s)) as u8),
        // from variable
        Token::Name(r_var) => {
          if let Some(i) = self.get_local(&r_var) {
            // from local variable
            ByteCode::SetGlobal(dst, i as u8)
          } else {
            // from global variable
            ByteCode::SetGlobalGlobal(dst, self.add_const(Value::String(r_var)) as u8)
          }
        }
        _ => panic!("invalid argument"),
      };
      self.bytecodes.push(code);
    }
  }

  fn chunk(&mut self) {
    loop {
      match self.lexer.next() {
        Token::Name(name) => {
          if self.lexer.peek() == &Token::Assign {
            self.assignment(name);
          } else {
            self.function_call(name);
          }
        }
        Token::Local => self.local_bind(),
        Token::Eos => break,
        t => panic!("unexpected token: {:?}", t),
      }
    }
  }

  pub fn load(input: File) -> Self {
    let mut proto = Self::new(input);

    proto.chunk();

    #[cfg(feature = "debug")]
    {
      let title = "Disassembler";
      println!("{} {} {}", "=".repeat(8), title, "=".repeat(8));
      println!("constants: {:#?}\n", &proto.constants);
      println!("bytecodes: [");
      for c in proto.bytecodes.iter() {
        println!("    {:?},", c);
      }
      println!("]");
      println!(
        "{}={}={}",
        "=".repeat(8),
        "=".repeat(title.len()),
        "=".repeat(8)
      );
    }
    println!();

    proto
  }
}
