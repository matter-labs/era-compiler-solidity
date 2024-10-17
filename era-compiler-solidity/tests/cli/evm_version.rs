use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_evm_version() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_evm_version_yul_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_evm_version_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_evm_version_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--evm-version",
        evm_version.as_str(),
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EVM version is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_evm_version_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let evm_version = era_compiler_common::EVMVersion::Cancun.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--evm-version",
        evm_version.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stdout(predicate::str::contains(
        "EVM version must be passed via standard JSON input.",
    ));

    Ok(())
}
