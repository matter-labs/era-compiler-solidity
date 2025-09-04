//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_YUL_CONTRACT_PATH, "--yul"];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested",
    ));

    Ok(())
}

#[test]
fn solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested",
    ));

    Ok(())
}

#[test]
fn invalid_input() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--yul"];

    let result = crate::cli::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("Yul parsing"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn combined_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--combined-json",
        "anyarg",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Only one mode is allowed at the same time:",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--standard-json",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time:",
    ));

    Ok(())
}

#[test]
fn invalid_solc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "DeclarationError: Function \\\"mdelete\\\" not found.",
    ));

    Ok(())
}

#[test]
fn invalid_zksolc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}

#[test]
fn through_solc_invalid_zksolc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}
