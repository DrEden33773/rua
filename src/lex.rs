//! # Lexical analysis
//!
//! Lexical analysis (front end) part of rua.
//!
//! ## Goal:
//!
//! Input -> TokenStream

use std::{
  io::{Bytes, Read},
  iter::Peekable,
  mem,
};

use crate::utils::TokenIterator;

pub mod lexing_methods;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  /* keywords */
  And,
  Break,
  Do,
  Else,
  Elseif,
  End,
  False,
  For,
  Function,
  Goto,
  If,
  In,
  Local,
  Nil,
  Not,
  Or,
  Repeat,
  Return,
  Then,
  True,
  Until,
  While,
  /* symbols */
  Add,       // +
  Sub,       // -
  Mul,       // *
  Div,       // /
  Mod,       // %
  Pow,       // ^
  Len,       // #
  BitAnd,    // &
  BitXor,    // ~
  BitOr,     // |
  ShiftL,    // <<
  ShiftR,    // >>
  Idiv,      // //
  Equal,     // ==
  NotEq,     // ~=
  LesEq,     // <=
  GreEq,     // >=
  Less,      // <
  Greater,   // >
  Assign,    // =
  ParL,      // (
  ParR,      // )
  CurlyL,    // {
  CurlyR,    // }
  SqurL,     // [
  SqurR,     // ]
  DoubColon, // ::
  SemiColon, // ;
  Colon,     // :
  Comma,     // ,
  Dot,       // .
  Concat,    // ..
  Dots,      // ...
  /* constant values */
  Integer(i64),
  Float(f64),
  String(Vec<u8>),
  /* name of vars or table_keys */
  Name(String),
  /* end */
  Eos,
}

impl Default for Token {
  fn default() -> Self {
    Self::Eos
  }
}

#[derive(Debug)]
pub struct Lex<R: Read> {
  /// source file
  input: Peekable<Bytes<R>>,
  /// token which is lexed from input file but shouldn't get returned
  ahead: Token,
}

impl<R: Read> Lex<R> {
  pub fn new(input: R) -> Self {
    Self {
      input: input.bytes().peekable(),
      ahead: Token::Eos,
    }
  }
}

impl<R: Read> Lex<R> {
  pub fn expect(&mut self, t: Token) {
    assert_eq!(self.next(), t);
  }

  fn read_byte(&mut self) -> u8 {
    match self.input.next() {
      Some(Ok(c)) => c,
      Some(_) => panic!("lex read error"),
      None => b'\0',
    }
  }

  fn peek_byte(&mut self) -> u8 {
    match self.input.peek() {
      Some(Ok(next)) => *next,
      Some(_) => panic!("lex read error"),
      None => b'\0',
    }
  }

  fn next_byte(&mut self) -> Option<u8> {
    self.input.next().map(|r| r.unwrap())
  }
}

impl<R: Read> TokenIterator for Lex<R> {
  type Output = Token;

  /// Take out the next token. (with updating `ahead`)
  fn next(&mut self) -> Self::Output {
    if self.ahead == Token::Eos {
      self.do_next()
    } else {
      mem::replace(&mut self.ahead, Token::Eos)
    }
  }

  /// Observe the next token(emplace).
  fn peek(&mut self) -> &Self::Output {
    if self.ahead == Token::Eos {
      self.ahead = self.do_next();
    }
    &self.ahead
  }

  /// Take out the next token.
  fn do_next(&mut self) -> Self::Output {
    if let Some(c) = self.next_byte() {
      match c {
        b' ' | b'\r' | b'\n' | b'\t' => self.do_next(),
        b'+' => Token::Add,
        b'*' => Token::Mul,
        b'%' => Token::Mod,
        b'^' => Token::Pow,
        b'#' => Token::Len,
        b'&' => Token::BitAnd,
        b'|' => Token::BitOr,
        b'(' => Token::ParL,
        b')' => Token::ParR,
        b'{' => Token::CurlyL,
        b'}' => Token::CurlyR,
        b'[' => Token::SqurL,
        b']' => Token::SqurR,
        b';' => Token::SemiColon,
        b',' => Token::Comma,
        b'/' => self.check_ahead(vec![b'/'], vec![Token::Idiv], Token::Div),
        b'=' => self.check_ahead(vec![b'='], vec![Token::Equal], Token::Assign),
        b'~' => self.check_ahead(vec![b'='], vec![Token::NotEq], Token::BitXor),
        b':' => self.check_ahead(vec![b':'], vec![Token::DoubColon], Token::Colon),
        b'<' => self.check_ahead(
          vec![b'=', b'<'],
          vec![Token::LesEq, Token::ShiftL],
          Token::Less,
        ),
        b'>' => self.check_ahead(
          vec![b'=', b'>'],
          vec![Token::GreEq, Token::ShiftR],
          Token::Less,
        ),
        b'-' => {
          if self.peek_byte() == b'-' {
            self.next_byte();
            self.lex_comment();
            self.do_next()
          } else {
            Token::Sub
          }
        }
        b'\'' | b'"' => self.lex_string(c),
        b'.' => match self.peek_byte() {
          b'.' => {
            self.next_byte();
            if self.peek_byte() == b'.' {
              self.next_byte();
              Token::Dots
            } else {
              Token::Concat
            }
          }
          b'0'..=b'9' => self.lex_number_fraction(0),
          _ => Token::Dot,
        },
        b'0'..=b'9' => self.lex_number(c),
        b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.lex_name(c),
        _ => panic!("unexpected char: {}", c),
      }
    } else {
      Token::Eos
    }
  }
}
