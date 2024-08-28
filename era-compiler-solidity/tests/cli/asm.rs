use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_asm_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--asm"];
    let invalid_args = &["--asm"];

    // Valid command
    let result = cli::execute_zksolc(args)?;
    let result_status_code = result
        .success()
        .stdout(predicate::str::contains("__entry:"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // solc exit code == zksolc exit code
    let solc_result = cli::execute_solc(args)?;
    solc_result.code(result_status_code);

    // Run invalid: zksolc --asm
    let invalid_result = cli::execute_zksolc(invalid_args)?;
    let invalid_result_status_code = invalid_result
        .failure()
        .stderr(
            predicate::str::contains("No input sources specified")
                .or(predicate::str::contains("Compilation aborted")),
        )
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Invalid solc exit code == Invalid zksolc exit code
    let invalid_solc_result = cli::execute_solc(invalid_args)?;
    invalid_solc_result.code(invalid_result_status_code);

    Ok(())
}

#[test]
fn run_zksolc_with_two_same_flags_asm_asm() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--asm", "--asm"];

    let result = cli::execute_zksolc(args)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--asm' was provided more than once",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test]
fn run_zksolc_with_asm_with_wrong_input_format() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--asm"];

    let result = cli::execute_zksolc(args)?;
    let solc_result = cli::execute_solc(args)?;

    let result_exit_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Expected identifier but got 'StringLiteral'",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    solc_result.code(result_exit_code);

    Ok(())
}
