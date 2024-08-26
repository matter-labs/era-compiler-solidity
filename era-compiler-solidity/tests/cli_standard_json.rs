#![cfg(test)]

pub mod cli_tests;
pub mod common;

use predicates::prelude::*;

#[test]
fn run_zksolc_with_standard_json_contract() -> anyhow::Result<()> {
    let _ = common::setup();
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;
    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli_tests::TEST_JSON_CONTRACT_PATH,
    ];
    let args_solc = &["--standard-json", cli_tests::TEST_JSON_CONTRACT_PATH];

    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_exit_code = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args_solc)?;
    solc_result.code(zksolc_exit_code);

    Ok(())
}

#[test]
fn run_zksolc_with_standard_json_incompatible_input() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--standard-json", cli_tests::TEST_YUL_CONTRACT_PATH];

    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_exit_code = result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_exit_code);

    Ok(())
}
