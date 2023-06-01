//!
//! The contract source code.
//!

pub mod evmla;
pub mod llvm_ir;
pub mod yul;
pub mod zkasm;

use crate::evmla::assembly::Assembly;
use crate::yul::parser::statement::object::Object;

use self::evmla::EVMLA;
use self::llvm_ir::LLVMIR;
use self::yul::Yul;
use self::zkasm::ZKASM;

///
/// The contract source code.
///
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum IR {
    /// The Yul source code.
    Yul(Yul),
    /// The EVM legacy assembly source code.
    EVMLA(EVMLA),
    /// The LLVM IR source code.
    LLVMIR(LLVMIR),
    /// The zkEVM assembly source code.
    ZKASM(ZKASM),
}

impl IR {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_yul(source_code: String, object: Object) -> Self {
        Self::Yul(Yul::new(source_code, object))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_evmla(assembly: Assembly) -> Self {
        Self::EVMLA(EVMLA::new(assembly))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_llvm_ir(path: String, source: String) -> Self {
        Self::LLVMIR(LLVMIR::new(path, source))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_zkasm(path: String, source: String) -> Self {
        Self::ZKASM(ZKASM::new(path, source))
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for IR
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.declare(context),
            Self::EVMLA(inner) => inner.declare(context),
            Self::LLVMIR(_inner) => Ok(()),
            Self::ZKASM(_inner) => Ok(()),
        }
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.into_llvm(context),
            Self::EVMLA(inner) => inner.into_llvm(context),
            Self::LLVMIR(_inner) => Ok(()),
            Self::ZKASM(_inner) => Ok(()),
        }
    }
}
