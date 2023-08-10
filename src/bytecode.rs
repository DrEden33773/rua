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
  /// ### format
  /// (table.index, table.array.len, table.hash_map.len)
  NewTable(u8, u8, u8),
  /// ### format
  /// (table.index, key<on_stack>.index, value.index)
  SetTable(u8, u8, u8),
  /// ### format
  /// (table.index, key<literal>.index, value.index)
  SetField(u8, u8, u8),
  /// ### format
  /// (table.index, item.count)
  SetList(u8, u8),
}

impl Debug for ByteCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const TO: &str = "To.Stack.Index";
    const FROM: &str = "From.Constants.Index";
    const FUNC: &str = "Func.Index";
    const ARG: &str = "Func.Arg.Count";
    const TABLE: &str = "Table.Index";
    const NARRAY: &str = "Table.Array.Len";
    const NMAP: &str = "Table.Map.Len";
    const CKEY: &str = "Key<Constants>.Index";
    const VKEY: &str = "Key<Stack>.Index";
    const VALUE: &str = "Value.Index";
    const ITEM: &str = "Inserted.Item.Count";
    match self {
      Self::GetGlobal(arg0, arg1) => f
        .debug_struct("GetGlobal")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::SetGlobal(arg0, arg1) => f
        .debug_struct("SetGlobal")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::SetGlobalConst(arg0, arg1) => f
        .debug_struct("SetGlobalConst")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::SetGlobalGlobal(arg0, arg1) => f
        .debug_struct("SetGlobalGlobal")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::LoadConst(arg0, arg1) => f
        .debug_struct("LoadConst")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::LoadNil(arg0) => f.debug_struct("LoadNil").field(TO, arg0).finish(),
      Self::LoadBool(arg0, arg1) => f
        .debug_struct("LoadBool")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::LoadInt(arg0, arg1) => f
        .debug_struct("LoadInt")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::Call(arg0, arg1) => f
        .debug_struct("Call")
        .field(FUNC, arg0)
        .field(ARG, arg1)
        .finish(),
      Self::Move(arg0, arg1) => f
        .debug_struct("Move")
        .field(TO, arg0)
        .field(FROM, arg1)
        .finish(),
      Self::NewTable(arg0, arg1, arg2) => f
        .debug_struct("NewTable")
        .field(TABLE, arg0)
        .field(NARRAY, arg1)
        .field(NMAP, arg2)
        .finish(),
      Self::SetTable(arg0, arg1, arg2) => f
        .debug_struct("SetTable")
        .field(TABLE, arg0)
        .field(VKEY, arg1)
        .field(VALUE, arg2)
        .finish(),
      Self::SetField(arg0, arg1, arg2) => f
        .debug_struct("SetField")
        .field(TABLE, arg0)
        .field(CKEY, arg1)
        .field(VALUE, arg2)
        .finish(),
      Self::SetList(arg0, arg1) => f
        .debug_struct("SetList")
        .field(TABLE, arg0)
        .field(ITEM, arg1)
        .finish(),
    }
  }
}
