//! # Bytecode
//!
//! Definition of bytecode of rua (vm).

/// ## ByteCode
///
/// ByteCode of rua.
#[derive(Debug, Clone, Copy)]
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
