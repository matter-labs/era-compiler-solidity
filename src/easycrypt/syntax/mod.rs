//!
//! EasyCrypt syntax tree.
//!

pub mod definition;
pub mod expression;
pub mod function;
pub mod literal;
pub mod module;
pub mod proc;
pub mod reference;
pub mod signature;
pub mod statement;
pub mod r#type;

///
/// A name of a custom identifier.
///
pub type Name = String;
