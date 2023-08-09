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
    let to = "To.Stack.Index: ";
    let from = "From.Constants.Index: ";
    let func = "Function.Index: ";
    let arg = "Function.Arg.Count: ";
    match self {
      Self::GetGlobal(arg0, arg1) => f
        .debug_tuple("GetGlobal")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::SetGlobal(arg0, arg1) => f
        .debug_tuple("SetGlobal")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::SetGlobalConst(arg0, arg1) => f
        .debug_tuple("SetGlobalConst")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::SetGlobalGlobal(arg0, arg1) => f
        .debug_tuple("SetGlobalGlobal")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::LoadConst(arg0, arg1) => f
        .debug_tuple("LoadConst")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::LoadNil(arg0) => f
        .debug_tuple("LoadNil")
        .field(&format!("{to}{arg0}"))
        .finish(),
      Self::LoadBool(arg0, arg1) => f
        .debug_tuple("LoadBool")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::LoadInt(arg0, arg1) => f
        .debug_tuple("LoadInt")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
      Self::Call(arg0, arg1) => f
        .debug_tuple("Call")
        .field(&format!("{func}{arg0}"))
        .field(&format!("{arg}{arg1}"))
        .finish(),
      Self::Move(arg0, arg1) => f
        .debug_tuple("Move")
        .field(&format!("{to}{arg0}"))
        .field(&format!("{from}{arg1}"))
        .finish(),
    }
  }
}
