#![cfg(test)]

use crate::common;
use assert_cmd::prelude::*;
use era_compiler_solidity::SolcCompiler;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The solidity contract name
pub const TEST_SOLIDITY_CONTRACT_NAME: &'static str = "contract.sol";
/// The solidity contract full path
pub const TEST_SOLIDITY_CONTRACT_PATH: &'static str =
    "tests/examples/contracts/solidity/contract.sol";

/// The solidity binary artifact output name
pub const SOLIDITY_BIN_OUTPUT_NAME: &'static str = "C.zbin";

/// The solidity assembly artifact output name
pub const SOLIDITY_ASM_OUTPUT_NAME: &'static str = "C.zasm";

/// The yul contract name
pub const TEST_YUL_CONTRACT_NAME: &'static str = "contract.yul";

/// The yul contract for testing
pub const TEST_YUL_CONTRACT_PATH: &'static str = "tests/examples/contracts/yul/contract.yul";

/// The era assembly contract path
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_PATH: &'static str =
    "tests/examples/contracts/eravm/contract.zasm";

/// The LLVM contract path
pub const TEST_LLVM_CONTRACT_PATH: &'static str = "tests/examples/contracts/llvm/contract.ll";

/// The standard JSON contract path
pub const TEST_JSON_CONTRACT_PATH: &'static str = "tests/examples/contracts/json/contract.json";

/// The binary bytecode sample path
pub const TEST_BINARY_BYTECODE_PATH: &'static str = "tests/examples/bytecodes/bytecode.zbin";

/// The hexadecimal string bytecode sample path
pub const TEST_HEXADECIMAL_BYTECODE_PATH: &'static str = "tests/examples/bytecodes/bytecode.hex";

/// Shared library path and address
pub const LIBRARY_DEFAULT_PATH: &'static str = "tests/examples/contracts/solidity/MiniMath.sol:MiniMath=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

///
/// Execute zksolc with the given arguments and return the result
///
pub fn execute_zksolc(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin("zksolc")?;
    Ok(cmd
        .env(
            "PATH",
            fs::canonicalize(&PathBuf::from(common::SOLC_DOWNLOAD_DIR))?,
        )
        .args(args)
        .assert())
}

///
/// Execute solc with the given arguments and return the result
///
pub fn execute_solc(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let solc_compiler =
        common::get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)?.executable;
    let mut cmd = Command::new(solc_compiler);
    Ok(cmd.args(args).assert())
}

///
/// Check if the file at the given path is empty
///
pub fn is_file_empty(file_path: &str) -> anyhow::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() == 0)
}

///
/// Check if the output is the same as the file content
///
pub fn is_output_same_as_file(file_path: &str, output: &str) -> anyhow::Result<bool> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content.trim().contains(output.trim()) || output.trim().contains(file_content.trim()))
}
