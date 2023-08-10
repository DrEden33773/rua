//! # Lexical analysis
//!
//! Lexical analysis (front end) part of rua.
//!
//! ## Goal:
//!
//! Input -> TokenStream

use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
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
pub struct Lex {
  /// source file
  input: File,
  /// token which is lexed from input file but shouldn't get returned
  ahead: Token,
}

impl Lex {
  pub fn new(input: File) -> Self {
    Self {
      input,
      ahead: Token::Eos,
    }
  }
}

impl Lex {
  fn read_char(&mut self) -> char {
    let mut buffer = [0];
    if self.input.read(&mut buffer).unwrap() == 1 {
      buffer[0] as char
    } else {
      '\0'
    }
  }

  fn move_back(&mut self) {
    self.input.seek(SeekFrom::Current(-1)).unwrap();
  }
}

impl TokenIterator for Lex {
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
        if self.read_char() == '-' {
          self.lex_comment();
          self.do_next()
        } else {
          self.move_back();
          Token::Sub
        }
      }
      '\'' | '"' => self.lex_string(c),
      '.' => match self.read_char() {
        '.' => {
          if self.read_char() == '.' {
            Token::Dots
          } else {
            self.move_back();
            Token::Concat
          }
        }
        '0'..='9' => {
          self.move_back();
          self.lex_number_fraction(0)
        }
        _ => {
          self.move_back();
          Token::Dot
        }
      },
      '0'..='9' => self.lex_number(c),
      'A'..='Z' | 'a'..='z' | '_' => self.lex_name(c),
      '\0' => Token::Eos,
      c => panic!("unexpected char: {}", c),
    }
  }
}
