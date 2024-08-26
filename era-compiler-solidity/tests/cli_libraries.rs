#![cfg(test)]

pub mod cli_tests;
pub mod common;
use predicates::prelude::*;

#[test]
fn run_zksolc_with_sol_and_libraries() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli_tests::LIBRARY_DEFAULT_PATH,
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn run_zksolc_without_sol_and_with_libraries() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--libraries"];

    let result = cli_tests::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "requires a value but none was supplied",
    ));

    Ok(())
}
