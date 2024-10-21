//!
//! The contract source code.
//!

pub mod eravm_assembly;
pub mod evmla;
pub mod llvm_ir;
pub mod yul;

use std::collections::HashSet;

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

impl From<Yul> for IR {
    fn from(inner: Yul) -> Self {
        Self::Yul(inner)
    }
}

impl From<EVMLA> for IR {
    fn from(inner: EVMLA) -> Self {
        Self::EVMLA(inner)
    }
}

impl From<LLVMIR> for IR {
    fn from(inner: LLVMIR) -> Self {
        Self::LLVMIR(inner)
    }
}

impl From<EraVMAssembly> for IR {
    fn from(inner: EraVMAssembly) -> Self {
        Self::EraVMAssembly(inner)
    }
}
