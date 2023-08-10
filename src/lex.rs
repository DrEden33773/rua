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

#[derive(Debug, PartialEq)]
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
  /* TODO: reserved identifiers */
  Async,
  Await,
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
  String(String),
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
  fn read_char(&mut self) -> char {
    match self.input.next() {
      Some(Ok(c)) => c as char,
      Some(_) => panic!("lex read error"),
      None => '\0',
    }
  }

  fn peek_char(&mut self) -> char {
    match self.input.peek() {
      Some(Ok(next)) => *next as char,
      Some(_) => panic!("lex read error"),
      None => '\0',
    }
  }

  fn next_char(&mut self) -> Option<char> {
    self.input.next().map(|r| r.unwrap().into())
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
    let c = self.read_char();
    match c {
      ' ' | '\r' | '\n' | '\t' => self.do_next(),
      '+' => Token::Add,
      '*' => Token::Mul,
      '%' => Token::Mod,
      '^' => Token::Pow,
      '#' => Token::Len,
      '&' => Token::BitAnd,
      '|' => Token::BitOr,
      '(' => Token::ParL,
      ')' => Token::ParR,
      '{' => Token::CurlyL,
      '}' => Token::CurlyR,
      '[' => Token::SqurL,
      ']' => Token::SqurR,
      ';' => Token::SemiColon,
      ',' => Token::Comma,
      '/' => self.check_ahead(vec!['/'], vec![Token::Idiv], Token::Div),
      '=' => self.check_ahead(vec!['='], vec![Token::Equal], Token::Assign),
      '~' => self.check_ahead(vec!['='], vec![Token::NotEq], Token::BitXor),
      ':' => self.check_ahead(vec![':'], vec![Token::DoubColon], Token::Colon),
      '<' => self.check_ahead(
        vec!['=', '<'],
        vec![Token::LesEq, Token::ShiftL],
        Token::Less,
      ),
      '>' => self.check_ahead(
        vec!['=', '>'],
        vec![Token::GreEq, Token::ShiftR],
        Token::Less,
      ),
      '-' => {
        if self.peek_char() == '-' {
          self.next_char();
          self.lex_comment();
          self.do_next()
        } else {
          Token::Sub
        }
      }
      '\'' | '"' => self.lex_string(c),
      '.' => match self.peek_char() {
        '.' => {
          self.next_char();
          if self.peek_char() == '.' {
            self.next_char();
            Token::Dots
          } else {
            Token::Concat
          }
        }
        '0'..='9' => self.lex_number_fraction(0),
        _ => Token::Dot,
      },
      '0'..='9' => self.lex_number(c),
      'A'..='Z' | 'a'..='z' | '_' => self.lex_name(c),
      '\0' => Token::Eos,
      c => panic!("unexpected char: {}", c),
    }
  }
}
