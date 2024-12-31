use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}

#[test]
fn deprecated_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--detect-missing-libraries",
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("`--detect-missing-libraries` is deprecated in standard JSON mode and must be passed in JSON"));

    Ok(())
}

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--detect-missing-libraries",
        "--llvm-ir",
        "--bin",
        common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--detect-missing-libraries",
        "--eravm-assembly",
        "--bin",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}

#[test]
fn standard_json_missing_sources() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}

#[test]
fn standard_json_llvm_ir() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_LLVM_IR_STANDARD_JSON_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}

#[test]
fn standard_json_eravm_assembly() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
