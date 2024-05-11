use crate::yul::path::Path;

use self::name::ProcName;

use super::definition::Definition;
use super::signature::Signature;
use super::statement::block::Block;

pub mod name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    pub name: ProcName,
    pub location: Option<Path>,
    pub signature: Signature,
    pub locals: Vec<Definition>,
    pub body: Block,
}
