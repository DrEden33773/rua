//! Expression description.
//!
//! Implementation of `ExpDesc`.
//!
//! ExpDesc => Act as an intermediate state/storage/register of
//! the expression (with it's `constant/variable` composition) that
//! will be used to generate the bytecode.

// TODO: remove this dangerous lint configuration (after finishing this module)
#![allow(unused)]

use super::*;

#[derive(Debug, PartialEq)]
pub(super) enum ExpDesc {
  Nil,
  Boolean(bool),
  Integer(i64),
  Float(f64),
  String(Vec<u8>),
  Local(usize),  // on stack, including local/temporary variables
  Global(usize), // global variable
  Index(usize, usize),
  IndexField(usize, usize),
  IndexInt(usize, u8),
  Call,
}

impl<R: Read> ParseProto<R> {
  pub(super) fn exp(&mut self) -> ExpDesc {
    match self.lexer.next() {
      Token::Nil => ExpDesc::Nil,
      Token::True => ExpDesc::Boolean(true),
      Token::False => ExpDesc::Boolean(false),
      Token::Integer(i) => ExpDesc::Integer(i),
      Token::Float(f) => ExpDesc::Float(f),
      Token::String(s) => ExpDesc::String(s),
      Token::Name(var) => self.simple_name(var),
      Token::CurlyL => self.table_constructor(),
      t => panic!("invalid expr: {:?}", t),
    }
  }

  /// TODO: finish this f**king piece of sh*t
  fn table_constructor(&mut self) -> ExpDesc {
    let table = self.sp;
    self.sp += 1;

    let new_index = self.bytecodes.len();
    self.bytecodes.push(ByteCode::NewTable(table as u8, 0, 0));

    let mut array_size = 0;
    let mut map_size = 0;

    loop {
      match self.lexer.peek() {
        Token::CurlyL => {
          // `}`
          self.lexer.next();
          break;
        }
        Token::SqurL => {
          // `[` ~> exp] = exp => general form
          map_size += 1;
          self.lexer.next(); // `[`
          self.load_exp(self.sp); // `exp`
          self.lexer.expect(Token::SqurR); // `]`
          self.lexer.expect(Token::Assign); // `=`
          self.load_exp(self.sp + 1); // `r_value`
          self.bytecodes.push(ByteCode::SetTable(
            table as u8,
            self.sp as u8,
            self.sp as u8 + 1,
          ));
        }
        Token::Name(_) => {
          // Name `=` exp | Name
          map_size += 1;
          let key = if let Token::Name(key) = self.lexer.next() {
            self.add_const(key)
          } else {
            panic!("invalid table constructor");
          };
          if self.lexer.peek() == &Token::Assign {
            // Name `=` exp => record form
            self.lexer.next();
            self.load_exp(self.sp);
            self
              .bytecodes
              .push(ByteCode::SetField(table as u8, key as u8, self.sp as u8));
          } else {
            // => list form
            array_size += 1;
          }
        }
        _ => panic!("invalid table constructor"),
      }
    }

    self.sp = table + 1;
    ExpDesc::Local(table)
  }

  fn simple_name(&mut self, name: String) -> ExpDesc {
    // search reversely => new var covers old one with same name
    if let Some(index) = self.locals.iter().rposition(|v| v == &name) {
      ExpDesc::Local(index)
    } else {
      ExpDesc::Global(self.add_const(name))
    }
  }
}
