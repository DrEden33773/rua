//! # Table
//!
//! Implementation of `Table` type in lua.

use std::{
  collections::HashMap,
  fmt::{Debug, Display},
};

use crate::value::Value;

#[derive(Debug)]
pub struct Table {
  pub array: Vec<Value>,
  pub map: HashMap<Value, Value>,
}

impl Table {
  pub fn new(array_size: usize, map_size: usize) -> Self {
    Self {
      array: Vec::with_capacity(array_size),
      map: HashMap::with_capacity(map_size),
    }
  }
}

impl Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{table}}: {:p}", self)
  }
}
