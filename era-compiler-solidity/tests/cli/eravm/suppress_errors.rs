use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_suppressed_errors() -> anyhow::Result<()> {
    common::setup()?;

    let error_type = era_solc::StandardJsonInputErrorType::SendTransfer.to_string();
    let args = &[
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-errors",
        error_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_suppressed_errors_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let error_type = era_solc::StandardJsonInputErrorType::SendTransfer.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--suppress-errors",
        error_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Suppressed errors must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test]
fn with_suppressed_errors_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-errors",
        "mega-ultra-error",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Invalid suppressed error type: mega-ultra-error",
    ));

    Ok(())
}

#[test]
fn with_suppressed_warnings_invalid_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_WARNINGS_INVALID,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "unknown variant `INVALID_SUPPRESSED_WARNING_TYPE`",
    ));

    Ok(())
}

#[test]
fn with_suppressed_errors_invalid_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_INVALID,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "unknown variant `INVALID_SUPPRESSED_ERROR_TYPE`",
    ));

    Ok(())
}
