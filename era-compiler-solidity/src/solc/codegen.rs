//!
//! The Solidity compiler codegen.
//!

use std::str::FromStr;

use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;

///
/// The Solidity compiler codegen.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Codegen {
    /// The Yul IR.
    Yul,
    /// The EVM legacy assembly IR.
    EVMLA,
}

impl Codegen {
    ///
    /// We always use EVMLA for Solidity <=0.7, or if the user does not want to compile via Yul.
    ///
    pub fn new(solc_version: &SolcVersion, explicit: Option<Self>) -> Self {
        if solc_version.default < SolcCompiler::FIRST_YUL_VERSION || explicit == Some(Self::EVMLA) {
            Self::EVMLA
        } else {
            Self::Yul
        }
    }
}

impl FromStr for Codegen {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "yul" => Ok(Self::Yul),
            "evmla" => Ok(Self::EVMLA),
            string => anyhow::bail!(
                "Invalid codegen: {string}. Available options: {}",
                [Self::EVMLA, Self::Yul]
                    .into_iter()
                    .map(|codegen| codegen.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl std::fmt::Display for Codegen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Yul => write!(f, "yul"),
            Self::EVMLA => write!(f, "evmla"),
        }
    }
}
