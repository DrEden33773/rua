//! # Bytecode
//!
//! Definition of bytecode of rua (vm).

use std::fmt::Debug;

/// ## ByteCode
///
/// ByteCode of rua.
#[derive(Clone, Copy)]
pub enum ByteCode {
  /// ### format
  /// (target stack index, global index)
  GetGlobal(u8, u8),
  /// ### format
  /// (target stack index, global index)
  SetGlobal(u8, u8),
  /// ### format
  /// (target stack index, const index)
  SetGlobalConst(u8, u8),
  /// ### format
  /// (target stack index, global index)
  SetGlobalGlobal(u8, u8),
  /// ### format
  /// (target stack index, const index)
  LoadConst(u8, u8),
  /// ### format
  /// (target stack index)
  LoadNil(u8),
  /// ### format
  /// (target stack index, the boolean)
  LoadBool(u8, bool),
  /// ### format
  /// (target stack index, int if in range of i16)
  LoadInt(u8, i16),
  /// ### format
  /// (target stack index, arg count)
  Call(u8, u8),
  /// ### format
  /// (destination index, source index)
  Move(u8, u8),
}

impl Debug for ByteCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let to = "To.Stack.Index";
    let from = "From.Constants.Index";
    let func = "Func.Index";
    let arg = "Func.Arg.Count";
    match self {
      Self::GetGlobal(arg0, arg1) => f
        .debug_struct("GetGlobal")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::SetGlobal(arg0, arg1) => f
        .debug_struct("SetGlobal")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::SetGlobalConst(arg0, arg1) => f
        .debug_struct("SetGlobalConst")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::SetGlobalGlobal(arg0, arg1) => f
        .debug_struct("SetGlobalGlobal")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::LoadConst(arg0, arg1) => f
        .debug_struct("LoadConst")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::LoadNil(arg0) => f.debug_struct("LoadNil").field(to, arg0).finish(),
      Self::LoadBool(arg0, arg1) => f
        .debug_struct("LoadBool")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::LoadInt(arg0, arg1) => f
        .debug_struct("LoadInt")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
      Self::Call(arg0, arg1) => f
        .debug_struct("Call")
        .field(func, arg0)
        .field(arg, arg1)
        .finish(),
      Self::Move(arg0, arg1) => f
        .debug_struct("Move")
        .field(to, arg0)
        .field(from, arg1)
        .finish(),
    }
  }
}
