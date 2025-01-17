//!
//! CLI tests for the eponymous option.
//!

use era_solc::StandardJsonInputWarningType;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(StandardJsonInputWarningType::TxOrigin)]
#[test_case(StandardJsonInputWarningType::AssemblyCreate)]
fn default(warning_type: StandardJsonInputWarningType) -> anyhow::Result<()> {
    crate::common::setup()?;

    let warning_type = warning_type.to_string();
    let args = &[
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let warning_type = era_solc::StandardJsonInputWarningType::TxOrigin.to_string();
    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Suppressed warnings must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test]
fn invalid_variant() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--suppress-warnings",
        "mega-ultra-warning",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Invalid suppressed warning type: mega-ultra-warning",
    ));

    Ok(())
}

#[test]
fn standard_json_invalid_variant() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_JSON_CONTRACT_PATH_SUPPRESSED_WARNINGS_INVALID,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "unknown variant `INVALID_SUPPRESSED_WARNING_TYPE`",
    ));

    Ok(())
}
