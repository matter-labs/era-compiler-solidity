//!
//! Tracker of paths from the root of a YUL syntax tree.
//!

pub mod symbol_tracker;

use super::Path;

///
/// Tracker of paths from the root of a YUL syntax tree.
///
pub trait PathTracker {
    ///
    /// Currently constructed path from the root of YUL syntax tree.
    ///
    fn here(&self) -> &Path;

    ///
    /// Exit the last lexical block on the way from the root of YUL syntax tree.
    ///
    fn leave(&mut self);

    ///
    /// Enter a block of statements between curly braces on the way from the root of YUL syntax tree.
    ///
    fn enter_block(&mut self);

    ///
    /// Enter a function on the way from the root of YUL syntax tree.
    ///
    fn enter_function(&mut self, identifier: &str);

    ///
    /// Enter a code section on the way from the root of YUL syntax tree.
    ///
    fn enter_code(&mut self);

    ///
    /// Enter a YUL object section on the way from the root of YUL syntax tree.
    ///
    fn enter_object(&mut self, identifier: &str);

    ///
    /// Enter the condition of an "if" statement on the way from the root of YUL syntax tree.
    ///
    fn enter_if_cond(&mut self);

    ///
    /// Enter the "yes" branch of an "if" statement on the way from the root of YUL syntax tree.
    ///
    fn enter_if_then(&mut self);

    ///
    /// Enter the initializer of a "for" statement on the way from the root of YUL syntax tree.
    ///
    fn enter_for1(&mut self);

    ///
    /// Enter the condition of a "for" statement on the way from the root of YUL syntax tree.
    ///
    fn enter_for2(&mut self);

    ///
    /// Enter the finalizer of a "for" statement on the way from the root of YUL syntax tree.
    ///
    fn enter_for3(&mut self);
}
