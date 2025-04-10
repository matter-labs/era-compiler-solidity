//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM, era_compiler_common::EXTENSION_ERAVM_BINARY)]
#[test_case(Target::EVM, era_compiler_common::EXTENSION_EVM_BINARY)]
fn yul(target: Target, extension: &str) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;

    let input_path = PathBuf::from(crate::common::TEST_YUL_CONTRACT_PATH);
    let input_file = input_path
        .file_name()
        .expect("Always exists")
        .to_str()
        .expect("Always valid");

    let args = &[
        input_path.to_str().expect("Always valid"),
        "--yul",
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    let output_file = tmp_dir_zksolc
        .path()
        .join(input_file)
        .join(format!("Test.{extension}"));
    assert!(output_file.exists());

    Ok(())
}

#[test_case(Target::EraVM, crate::common::SOLIDITY_ASM_OUTPUT_NAME_ERAVM)]
fn asm_and_metadata(target: Target, asm_file_name: &str) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let mut asm_path = tmp_dir_zksolc.path().to_path_buf();
    asm_path.push(crate::common::TEST_SOLIDITY_CONTRACT_NAME);
    asm_path.push(asm_file_name);

    let mut metadata_path = tmp_dir_zksolc.path().to_path_buf();
    metadata_path.push(crate::common::TEST_SOLIDITY_CONTRACT_NAME);
    metadata_path.push("Test_meta.json");

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    assert!(asm_path.exists());
    assert!(metadata_path.exists());

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn weird_path_characters(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("File!and#$%-XXXXXX")?;
    let tmp_dir_solc = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn combined_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("File!and#$%-XXXXXX")?;
    let tmp_dir_solc = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--output-dir",
        "output",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Output directory cannot be used in standard JSON mode.",
    ));

    Ok(())
}
