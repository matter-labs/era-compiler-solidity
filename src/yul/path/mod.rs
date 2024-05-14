//!
//! Path from the root of YUL syntax tree to some location in it.
//!
//! In YUL code it is possible to have variables with the same name located in different lexical scopes, for example:
//! ```
//! function sample() {
//! let x
//! {
//!    let y := add(1, x)
//! }
//! {
//!    let y := add(1, x)
//! }
//! }
//! ```
//! In this example, `y` refers to different variables. If a code analysis
//! needs information about both of them, we can differentiate between them
//! based on their lexical scopes; each scope is then identified by a path from
//! the root of YUL syntax tree to its block.
//!

pub mod builder;

mod step;
pub mod tracker;

pub use builder::Builder;

use self::step::LexicalBlock;

/// Path from the root of YUL syntax tree to a specific lexical block in it,
/// including all the blocks on the way from root.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    stack: Vec<LexicalBlock>,
}

impl Path {
    /// Transforms [`crate::yul::path::Path`] into a prefix for a variable name.
    /// Each block on the way from root will contribute to the prefix.
    pub fn full(&self) -> String {
        self.stack
            .iter()
            .fold(String::from(""), |acc, step| -> String {
                let contribution = LexicalBlock::full_name_contribution(step);
                format!("{acc}_{contribution}")
            })
    }
}
