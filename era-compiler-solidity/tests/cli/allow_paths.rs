use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_allow_paths(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_allow_paths_yul_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_allow_paths_llvm_ir_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_allow_paths_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--allow-paths",
        cli::TEST_CONTRACTS_PATH,
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`allow-paths` is only allowed in Solidity mode",
    ));

    Ok(())
}
