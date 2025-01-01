//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn yul(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn llvm_ir(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn eravm_assembly(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--evm-version",
        evm_version.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "EVM version must be passed via standard JSON input.",
    ));

    Ok(())
}
