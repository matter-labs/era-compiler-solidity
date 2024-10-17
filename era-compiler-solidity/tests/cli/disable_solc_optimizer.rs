use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_disable_solc_optimizer() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
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
fn with_disable_solc_optimizer_yul_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Disabling the solc optimizer is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_disable_solc_optimizer_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Disabling the solc optimizer is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_disable_solc_optimizer_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Disabling the solc optimizer is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_disable_solc_optimizer_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--disable-solc-optimizer",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Disabling the solc optimizer must be specified in standard JSON input settings.",
    ));

    Ok(())
}
