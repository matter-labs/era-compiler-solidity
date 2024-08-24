//!
//! The contract source code.
//!

pub mod eravm_assembly;
pub mod evmla;
pub mod llvm_ir;
pub mod yul;

use std::collections::HashSet;

use crate::evmla::assembly::Assembly;
use crate::solc::standard_json::output::contract::evm::extra_metadata::ExtraMetadata;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::statement::object::Object;

use self::eravm_assembly::EraVMAssembly;
use self::evmla::EVMLA;
use self::llvm_ir::LLVMIR;
use self::yul::Yul;

///
/// The contract source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IR {
    /// The Yul source code.
    Yul(Yul),
    /// The EVM legacy assembly source code.
    EVMLA(EVMLA),
    /// The LLVM IR source code.
    LLVMIR(LLVMIR),
    /// The EraVM assembly source code.
    EraVMAssembly(EraVMAssembly),
}

impl IR {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_yul(object: Object<LLVMDialect>) -> Self {
        Self::Yul(Yul::new(object))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_evmla(assembly: Assembly, extra_metadata: ExtraMetadata) -> Self {
        Self::EVMLA(EVMLA::new(assembly, extra_metadata))
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
    pub fn new_eravm_assembly(path: String, source: String) -> Self {
        Self::EraVMAssembly(EraVMAssembly::new(path, source))
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        match self {
            Self::Yul(inner) => inner.get_missing_libraries(),
            Self::EVMLA(inner) => inner.get_missing_libraries(),
            Self::LLVMIR(_inner) => HashSet::new(),
            Self::EraVMAssembly(_inner) => HashSet::new(),
        }
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for IR
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.declare(context),
            Self::EVMLA(inner) => inner.declare(context),
            Self::LLVMIR(_inner) => Ok(()),
            Self::EraVMAssembly(_inner) => Ok(()),
        }
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.into_llvm(context),
            Self::EVMLA(inner) => inner.into_llvm(context),
            Self::LLVMIR(_inner) => Ok(()),
            Self::EraVMAssembly(_inner) => Ok(()),
        }
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for IR
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.declare(context),
            Self::EVMLA(_inner) => todo!(),
            Self::LLVMIR(_inner) => Ok(()),
            Self::EraVMAssembly(_inner) => {
                anyhow::bail!("EraVM assembly cannot be compiled to the EVM target")
            }
        }
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        match self {
            Self::Yul(inner) => inner.into_llvm(context),
            Self::EVMLA(_inner) => todo!(),
            Self::LLVMIR(_inner) => Ok(()),
            Self::EraVMAssembly(_inner) => {
                anyhow::bail!("EraVM assembly cannot be compiled to the EVM target")
            }
        }
    }
}
