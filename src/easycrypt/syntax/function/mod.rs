use crate::yul::path::Path;

use self::name::FunctionName;

use super::expression::Expression;
use super::signature::Signature;

pub mod name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: FunctionName,
    pub location: Option<Path>,
    pub signature: Signature,
    pub body: Expression,
}
