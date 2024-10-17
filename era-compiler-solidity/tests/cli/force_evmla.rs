use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_force_evmla() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--force-evmla", "--bin", cli::TEST_SOLIDITY_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_force_evmla_yul_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: EVM legacy assembly codegen is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_force_evmla_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: EVM legacy assembly codegen is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_force_evmla_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: EVM legacy assembly codegen is only available in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_force_evmla_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_PATH,
        "--force-evmla",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
