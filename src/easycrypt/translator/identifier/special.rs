//!
//! Transpilation of special YUL identifiers that look like functions but affect the control flow.
//!

pub enum YulSpecial {
    Return,
    Revert,
    Stop,
    Invalid,
}
