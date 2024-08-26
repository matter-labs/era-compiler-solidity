#![cfg(test)]

pub mod cli_tests;
pub mod common;

use predicates::prelude::*;

#[test]
fn run_zksolc_with_yul_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let zksolc_args = &[cli_tests::TEST_YUL_CONTRACT_PATH, "--yul"];
    let solc_args = &[cli_tests::TEST_YUL_CONTRACT_PATH, "--strict-assembly"];
    let invalid_args = &["--yul", "anyarg"];

    // Valid command
    let result = cli_tests::execute_zksolc(zksolc_args)?;
    let zksolc_status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .stderr(predicate::str::contains("No output requested"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // solc exit code comparison
    let solc_result = cli_tests::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    // Invalid command
    let invalid_result = cli_tests::execute_zksolc(invalid_args)?;
    let invalid_status = invalid_result
        .failure()
        .stderr(predicate::str::contains("Error"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Invalid solc vs zksolc exit code comparison
    let solc_invalid_result = cli_tests::execute_solc(invalid_args)?;
    solc_invalid_result.code(invalid_status);

    Ok(())
}

#[test]
fn run_zksolc_with_double_yul_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_YUL_CONTRACT_PATH, "--yul", "--yul"];

    // Execute zksolc with duplicate --yul
    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--yul' was provided more than once",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with solc
    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_input_format_solidity_contract() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_SOLIDITY_CONTRACT_PATH, "--yul"];

    // Execute zksolc with incompatible Solidity contract and --yul flag
    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("Yul parsing"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with solc
    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_json_modes_combined_json() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--combined-json",
        "anyarg",
    ];

    // Execute zksolc with incompatible --yul and --combined-json flags
    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains(
            "Only one mode is allowed at the same time:",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Compare with solc
    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_json_modes_standard_json() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--standard-json",
    ];

    // Execute zksolc with incompatible --yul and --standard-json flags
    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains(
            "Only one mode is allowed at the same time:",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    Ok(())
}
