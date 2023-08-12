//! Expression description.
//!
//! Implementation of `ExpDesc`.
//!
//! ExpDesc => Act as an intermediate state/storage/register of
//! the expression (with it's `constant/variable` composition) that
//! will be used to generate the bytecode.

#[allow(unused_imports)]
use super::*;

#[allow(dead_code)]
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
