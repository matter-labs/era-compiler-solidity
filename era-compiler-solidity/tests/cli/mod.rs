//!
//! The CLI/e2e tests entry module.
//!

use std::path::PathBuf;
use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;

mod allow_paths;
mod asm;
mod base_path;
mod basic;
mod bin;
mod codegen;
mod combined_json;
mod debug_output_dir;
mod disable_solc_optimizer;
mod eravm;
mod evm_version;
mod fallback_oz;
mod force_evmla;
mod include_path;
mod libraries;
mod llvm_ir;
mod llvm_options;
mod metadata;
mod metadata_hash;
mod metadata_literal;
mod optimization;
mod output_dir;
mod overwrite;
mod recursive_process;
mod remappings;
mod solc;
mod standard_json;
mod threads;
mod version;
mod yul;

///
/// Execute zksolc with the given arguments and return the result.
///
pub fn execute_zksolc(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    Ok(cmd
        .env(
            "PATH",
            std::fs::canonicalize(PathBuf::from(crate::common::SOLC_DOWNLOAD_DIRECTORY))?,
        )
        .args(args)
        .assert())
}

///
/// Execute zksolc with the given arguments including target, and return the result.
///
pub fn execute_zksolc_with_target(
    args: &[&str],
    target: era_compiler_common::Target,
) -> anyhow::Result<assert_cmd::assert::Assert> {
    let mut cmd = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    Ok(cmd
        .env(
            "PATH",
            std::fs::canonicalize(PathBuf::from(crate::common::SOLC_DOWNLOAD_DIRECTORY))?,
        )
        .args(args)
        .args(&["--target", target.to_string().as_str()])
        .assert())
}

///
/// Execute solc with the given arguments and return the result.
///
pub fn execute_solc(args: &[&str]) -> anyhow::Result<assert_cmd::assert::Assert> {
    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;
    let mut cmd = Command::new(solc_compiler);
    Ok(cmd.args(args).assert())
}

///
/// Check if the file at the given path is empty.
///
pub fn is_file_empty(file_path: &str) -> anyhow::Result<bool> {
    let metadata = std::fs::metadata(file_path)?;
    Ok(metadata.len() == 0)
}

///
/// Check if the output is the same as the file content.
///
pub fn is_output_same_as_file(file_path: &str, output: &str) -> anyhow::Result<bool> {
    let file_content = std::fs::read_to_string(file_path)?;
    Ok(file_content.trim().contains(output.trim()) || output.trim().contains(file_content.trim()))
}
