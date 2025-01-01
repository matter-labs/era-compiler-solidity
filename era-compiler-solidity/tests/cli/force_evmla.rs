//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn deprecated(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Warning: `--force-evmla` flag is deprecated: please use `--codegen 'evmla'` instead.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn deprecated_standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_FORCE_EVMLA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "The `forceEVMLA` setting is deprecated. Please use `codegen: 'evmla'` instead.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn yul(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn llvm_ir(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn eravm_assembly(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--force-evmla",
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--force-evmla",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
