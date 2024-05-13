//!
//! EasyCrypt AST node containing function or procedure signature.
//!

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::r#type::Type;

/// Signature may belong to a function or to a procedure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Function,
    Proc,
}

/// EasyCrypt AST node containing function or procedure signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub formal_parameters: Vec<(Definition, Type)>,
    pub return_type: Type,
    pub kind: SignatureKind,
}

impl Signature {
    /// A signature of a procedure with zero arguments.
    pub const UNIT_TO_UNIT: Signature = Signature {
        formal_parameters: vec![],
        return_type: Type::Unit,
        kind: SignatureKind::Proc,
    };
}
