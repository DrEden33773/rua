//! # Utils
//!
//! Utility functions/traits will be used in implementing all components.

/// ## TokenIterator
///
/// Why not use [`std::iter::Iterator`]? That's because, `Token` is a enum with a `Eos` state,
/// which could act just as a `None` value in `Option`.
///
/// We hate to repeatedly matching Some(_) or None!
pub trait TokenIterator {
  type Output;
  fn next(&mut self) -> Self::Output;
}

/// ## New
///
/// A trait that provides a `new` method.
pub trait New {
  type Output;
  fn new() -> Self::Output;
}
