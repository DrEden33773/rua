//! # Bytecode
//!
//! Definition of bytecode of rua (vm).

/// ## ByteCode
///
/// ByteCode of rua.
#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
  /// ### GetGlobal
  /// Get a global variable, push it to stack.
  /// ### format
  /// (target_stack_index, global_index)
  GetGlobal(u8, u8),
  /// ### LoadConst
  /// Load a constant from constants vec, push it to stack.
  /// ### format
  /// (target_stack_index, const_index)
  LoadConst(u8, u8),
  /// ### Call
  /// Call a function.
  /// ### format
  /// (target_stack_index, arg_count)
  Call(u8, u8),
}
