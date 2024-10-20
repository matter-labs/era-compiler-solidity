use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_suppressed_warnings() -> anyhow::Result<()> {
    common::setup()?;

    let warning_type = era_compiler_solidity::WarningType::TxOrigin.to_string();
    let args = &[
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_suppressed_warnings_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let warning_type = era_compiler_solidity::WarningType::TxOrigin.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Suppressed warnings must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test]
fn with_suppressed_warnings_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-warnings",
        "mega-ultra-warning",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Invalid suppressed warning type: mega-ultra-warning",
    ));

    Ok(())
}
