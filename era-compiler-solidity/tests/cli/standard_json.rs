use crate::{cli, common};
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
        cli::TEST_JSON_CONTRACT_PATH,
    ];
    let args_solc = &["--standard-json", cli::TEST_JSON_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
    let zksolc_exit_code = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(zksolc_exit_code);

    Ok(())
}

#[test]
fn run_zksolc_with_standard_json_incompatible_input() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--standard-json", cli::TEST_YUL_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
    let zksolc_exit_code = result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_exit_code);

    Ok(())
}

#[test]
fn run_zksolc_with_standard_json_suppressed_errors_and_warnings_deserialization(
) -> anyhow::Result<()> {
    let _ = common::setup();
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;
    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];

    let result = cli::execute_zksolc(args)?;
    let zksolc_exit_code = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(zksolc_exit_code);

    Ok(())
}

#[test]
fn run_zksolc_with_incorrect_standard_json_suppressed_errors_and_warnings_deserialization(
) -> anyhow::Result<()> {
    let _ = common::setup();
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;
    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_INCORRECT_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "unknown variant `INVALID_SUPPRESSED_ERROR_TYPE`",
    ));

    Ok(())
}
