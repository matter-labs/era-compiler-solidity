use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_PATH,
    ];
    let args_solc = &["--standard-json", cli::TEST_SOLIDITY_STANDARD_JSON_PATH];

    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_standard_json_incompatible_input() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_YUL_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_standard_json_with_suppressed_messages() -> anyhow::Result<()> {
    common::setup()?;

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
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_standard_json_with_suppressed_messages_invalid() -> anyhow::Result<()> {
    common::setup()?;

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
        "unknown variant `INVALID_SUPPRESSED_MESSAGE_TYPE`",
    ));

    Ok(())
}
