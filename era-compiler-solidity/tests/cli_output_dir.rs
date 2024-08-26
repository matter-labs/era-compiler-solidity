#![cfg(test)]

pub mod cli_tests;
pub mod common;

use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn run_zksolc_with_output_dir_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let zksolc_args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    // Execute zksolc command
    let result = cli_tests::execute_zksolc(zksolc_args)?;
    let zksolc_status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure the directory is created
    assert!(tmp_dir_zksolc.path().exists());

    // Compare with solc
    let solc_result = cli_tests::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_output_dir_invalid_arg_no_path() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
    ];

    // Execute invalid zksolc command
    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("error: The argument '--output-dir <output-directory>' requires a value but none was supplied"))
        .get_output().status.code().expect("No exit code.");

    // Compare with solc
    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_output_dir_invalid_args_no_source() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let zksolc_args = &[
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    // Execute zksolc with missing source
    let result = cli_tests::execute_zksolc(zksolc_args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with solc
    let solc_result = cli_tests::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_output_dir_specific_symbols() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir_zksolc = TempDir::with_prefix("File!and#$%-XXXXXX")?;
    let tmp_dir_solc = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let zksolc_args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    // Execute zksolc command
    let result = cli_tests::execute_zksolc(zksolc_args)?;
    let zksolc_status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Ensure the directory is created with specific symbols
    assert!(tmp_dir_zksolc.path().exists());

    // Compare with solc
    let solc_result = cli_tests::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    Ok(())
}
