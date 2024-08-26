#![cfg(test)]

pub mod cli_tests;
pub mod common;

use predicates::prelude::*;

#[test]
fn run_zksolc_with_optimization_levels() -> anyhow::Result<()> {
    let _ = common::setup();
    let optimization_args = ["0", "1", "2", "3", "s", "z"];

    for opt in &optimization_args {
        let args = &[
            cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
            &format!("-O{}", opt),
        ];

        let result = cli_tests::execute_zksolc(args)?;
        result
            .success()
            .stderr(predicate::str::contains("Compiler run successful"));
    }

    Ok(())
}

#[test]
fn run_zksolc_with_optimization_no_input_file() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["-O0"];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"));

    Ok(())
}

#[test]
fn run_zksolc_with_invalid_optimization_option() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_SOLIDITY_CONTRACT_PATH, "-O99"];

    let result = cli_tests::execute_zksolc(args)?;
    result.failure().stderr(
        predicate::str::contains("Unexpected optimization option")
            .or(predicate::str::contains("Invalid value for")),
    );

    Ok(())
}
