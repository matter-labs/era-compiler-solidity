use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_allow_paths() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
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
fn with_allow_paths_yul_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_allow_paths_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test]
fn with_allow_paths_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}
