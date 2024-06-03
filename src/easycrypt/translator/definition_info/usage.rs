//!
//! How is memory or storage affected by a procedure.
//!

#[allow(dead_code)]
/// How is memory or storage affected by a procedure.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Usage {
    #[default]
    NoUse,
    Read,
    Meta,
    Write,
    ReadWrite,
}
