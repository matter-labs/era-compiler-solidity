use super::{definition::Definition, r#type::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Function,
    Proc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub formal_parameters: Vec<(Definition, Type)>,
    pub return_type: Type,
    pub kind: SignatureKind,
}

impl Signature {
    pub const UNIT_TO_UNIT: Signature = Signature {
        formal_parameters: vec![],
        return_type: Type::Unit,
        kind: SignatureKind::Proc,
    };
}
