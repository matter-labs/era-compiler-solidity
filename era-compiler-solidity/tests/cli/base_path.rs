use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_base_path(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        common::TEST_CONTRACTS_PATH,
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
fn with_base_path_yul_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        common::TEST_CONTRACTS_PATH,
        "--yul",
        "--bin",
        common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_base_path_llvm_ir_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        common::TEST_CONTRACTS_PATH,
        "--llvm-ir",
        "--bin",
        common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_base_path_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--base-path",
        common::TEST_CONTRACTS_PATH,
        "--eravm-assembly",
        "--bin",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "`base-path` is only allowed in Solidity mode",
    ));

    Ok(())
}
