#![cfg(test)]

pub mod cli_tests;
pub mod common;
use predicates::prelude::*;

#[test]
fn run_zksolc_with_disassemble_binary() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_BINARY_BYTECODE_PATH, "--disassemble"];
    let invalid_args = &["--disassemble", "anyarg"];

    let result = cli_tests::execute_zksolc(args)?;
    let invalid_result = cli_tests::execute_zksolc(invalid_args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));
    invalid_result.failure();

    Ok(())
}

#[test]
fn run_zksolc_with_disassemble_hexadecimal() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_HEXADECIMAL_BYTECODE_PATH, "--disassemble"];
    let invalid_args = &["--disassemble", "anyarg"];

    let result = cli_tests::execute_zksolc(args)?;
    let invalid_result = cli_tests::execute_zksolc(invalid_args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));
    invalid_result.failure();

    Ok(())
}
