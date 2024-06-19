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
    /// Returns true if the signature return type is Unit.
    pub fn returns_unit(&self) -> bool {
        self.return_type == Type::Unit
    }

    /// Returns an instance of the arrow type matching the signature.
    pub fn get_type(&self) -> Type {
        Type::Arrow(
            Box::from(Type::Tuple(
                self.formal_parameters
                    .iter()
                    .map(|(_, typ)| typ)
                    .cloned()
                    .collect(),
            )),
            Box::from(self.return_type.clone()),
        )
    }
    /// A signature of a procedure with zero arguments.
    pub const UNIT_TO_UNIT: Signature = Signature {
        formal_parameters: vec![],
        return_type: Type::Unit,
        kind: SignatureKind::Proc,
    };
}
