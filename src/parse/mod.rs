//! # Parse
//!
//! Parse the bytecode to a AST.
//!
//! ## Goal:
//!
//! ByteCode -> AST

use crate::{
  bytecode::ByteCode,
  lex::{Lex, Token},
  utils::TokenIterator,
  value::Value,
};
use std::fs::File;

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
}

impl ParseProto {
  pub fn load(input: File) -> Self {
    let mut constants = Vec::new();
    let mut bytecodes = Vec::new();
    let mut lex = Lex::new(input);
    'l: loop {
      match lex.next() {
        /* `Literal.Name` as functions call */
        Token::Name(name) => {
          constants.push(Value::String(name));
          bytecodes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

          if let Token::String(s) = lex.next() {
            constants.push(Value::String(s));
            bytecodes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8))
          } else {
            panic!("expected `Literal.String`")
          }
        }
        /* No new token could be found, end */
        Token::Eos => break 'l,
        t => panic!("unexpected token: {:?}", t),
      }
    }
    #[cfg(feature = "debug")]
    {
      dbg!(&constants);
      dbg!(&bytecodes);
    }
    Self {
      constants,
      bytecodes,
    }
  }
}
