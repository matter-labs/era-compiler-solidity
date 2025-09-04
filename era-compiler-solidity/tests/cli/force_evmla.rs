//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn deprecated() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Warning: `--force-evmla` flag is deprecated: please use `--codegen 'evmla'` instead.",
    ));

    Ok(())
}

#[test]
fn deprecated_standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_FORCE_EVMLA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The `forceEVMLA` setting is deprecated. Please use `codegen: 'evmla'` instead.",
    ));

    Ok(())
}

#[test]
fn yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--force-evmla",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
