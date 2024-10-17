use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_base_path() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        cli::TEST_CONTRACTS_PATH,
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
fn with_base_path_yul_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        cli::TEST_CONTRACTS_PATH,
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_base_path_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        cli::TEST_CONTRACTS_PATH,
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_base_path_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        cli::TEST_CONTRACTS_PATH,
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}
