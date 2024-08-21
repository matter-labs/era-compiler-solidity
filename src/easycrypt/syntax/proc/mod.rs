//!
//! EasyCrypt AST node containing a definition of a new procedure.
//!

pub mod name;

use self::name::ProcName;

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::statement::block::Block;
use crate::yul::path::Path;

///
/// EasyCrypt AST node containing a definition of a new procedure.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    /// Name of the procedure.
    pub name: ProcName,
    /// Optionally, a location of the original function in the YUL source code.
    pub location: Option<Path>,
    /// Signature of the procedure.
    pub signature: Signature,
    /// Definitions of the local variables.
    pub locals: Vec<Definition>,
    /// Body of the procedure.
    pub body: Block,
}
