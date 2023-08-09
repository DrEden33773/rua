//! # Lexical analysis
//!
//! Lexical analysis (front end) part of rua.
//!
//! ## Goal:
//!
//! Input -> TokenStream

use std::fs::File;

use crate::utils::TokenIterator;

#[derive(Debug)]
pub enum Token {
  Name(String),
  String(String),
  Eos,
}

#[derive(Debug)]
pub struct Lex {
  input: File,
}

impl Lex {
  pub fn new(input: File) -> Self {
    Self { input }
  }
}

impl TokenIterator for Lex {
  type Output = Token;
  fn next(&mut self) -> Self::Output {
    Token::Eos
  }
}
