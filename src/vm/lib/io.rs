//! # IO Library
//!
//! IO library for Lua VM.

use super::*;

// "print" function in Lua's std-lib.
// It supports only 1 argument and assumes the argument is at `index: 1` on stack.
pub(crate) fn lib_print(state: &mut ExeState) -> i32 {
  let expression_index = state.func_index + 1;
  println!("{:?}", state.stack[expression_index]);
  0
}
