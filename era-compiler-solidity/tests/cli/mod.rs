use crate::common;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use era_compiler_solidity::SolcCompiler;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod asm;
mod basic;
mod bin;
mod combined_json;
mod disassembler;
mod eravm_assembly;
mod libraries;
mod linker;
mod llvm_ir;
mod metadata_hash;
mod missing_lib;
mod optimization;
mod output_dir;
mod solc;
mod standard_json;
mod yul;

/// The Solidity contract name.
pub const TEST_SOLIDITY_CONTRACT_NAME: &str = "contract.sol";

/// The Solidity contract full path.
pub const TEST_SOLIDITY_CONTRACT_PATH: &str = "tests/examples/contracts/solidity/contract.sol";

/// The Solidity binary artifact output name.
pub const SOLIDITY_BIN_OUTPUT_NAME: &str = "C.zbin";

/// The Solidity assembly artifact output name.
pub const SOLIDITY_ASM_OUTPUT_NAME: &str = "C.zasm";

/// The Yul contract for testing.
pub const TEST_YUL_CONTRACT_PATH: &str = "tests/examples/contracts/yul/contract.yul";

/// The LLVM IR contract path.
pub const TEST_LLVM_IR_CONTRACT_PATH: &str = "tests/examples/contracts/llvm/contract.ll";

/// The EraVM assembly contract path.
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_PATH: &str = "tests/examples/contracts/eravm/contract.zasm";

/// The standard JSON contract path.
pub const TEST_JSON_CONTRACT_PATH: &str = "tests/examples/contracts/json/contract.json";

/// The standard JSON contract path with suppressed errors and warnings.
pub const TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS: &str =
    "tests/examples/standard_json_input/contract_suppressed_warnings_and_errors.json";

/// The standard JSON contract path with incorrect suppressed errors and warnings.
pub const TEST_JSON_CONTRACT_PATH_INCORRECT_SUPPRESSED_ERRORS_AND_WARNINGS: &str =
    "tests/examples/standard_json_input/contract_incorrect_suppressed_warnings_and_errors.json";

/// The disassembler hexadecimal string bytecode sample path.
pub const TEST_DISASSEMBLER_HEXADECIMAL_BYTECODE_PATH: &str =
    "tests/examples/bytecodes/disassembler.hex";

/// The disassembler binary bytecode sample path.
pub const TEST_DISASSEMBLER_BINARY_BYTECODE_PATH: &str =
    "tests/examples/bytecodes/disassembler.zbin";

/// The linker hexadecimal string bytecode sample path.
pub const TEST_LINKER_BYTECODE_PATH: &str = "tests/examples/bytecodes/linker.hex";

/// The linker hexadecimal string bytecode sample path.
/// This file must be copied from `TEST_LINKER_BYTECODE_PATH` before linking and removed afterwards
pub const TEST_LINKER_BYTECODE_COPY_PATH: &str = "tests/examples/bytecodes/linker_copy.hex";

/// The broken input file path.
pub const TEST_BROKEN_INPUT_PATH: &str = "tests/examples/contracts/broken.bad";

/// Default library path and address.
pub const LIBRARY_DEFAULT: &str = "tests/examples/contracts/solidity/MiniMath.sol:MiniMath=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

/// Linker library path and address.
pub const LIBRARY_LINKER: &str =
    "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678";

///
/// Execute zksolc with the given arguments and return the result
///
pub fn execute_zksolc(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin("zksolc")?;
    Ok(cmd
        .env(
            "PATH",
            fs::canonicalize(PathBuf::from(common::SOLC_DOWNLOAD_DIR))?,
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
