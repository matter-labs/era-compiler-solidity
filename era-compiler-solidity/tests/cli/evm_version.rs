use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_evm_version(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--bin",
        common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_evm_version_yul_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--yul",
        "--bin",
        common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_evm_version_llvm_ir_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--llvm-ir",
        "--bin",
        common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_evm_version_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--eravm-assembly",
        "--bin",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_evm_version_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--evm-version",
        evm_version.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.success().stdout(predicate::str::contains(
        "EVM version must be passed via standard JSON input.",
    ));

    Ok(())
}
