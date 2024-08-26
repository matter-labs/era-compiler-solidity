pub mod cli_tests;
#[cfg(test)]
pub mod common;
use predicates::prelude::*;

#[test]
fn run_zksolc_with_eravm_assembly_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--bin",
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn run_zksolc_with_double_eravm_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--eravm-assembly",
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The argument '--eravm-assembly' was provided more than once",
    ));

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_input_format() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--eravm-assembly",
        "--bin",
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_json_modes() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--combined-json",
        "wrong",
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Only one mode is allowed"));

    Ok(())
}
