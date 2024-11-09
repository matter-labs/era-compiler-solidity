use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_force_evmla(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--force-evmla", "--bin", cli::TEST_SOLIDITY_CONTRACT_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Warning: `--force-evmla` flag is deprecated: please use `--codegen 'evmla'` instead.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_force_evmla_yul_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_force_evmla_llvm_ir_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_force_evmla_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--force-evmla",
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_force_evmla_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--force-evmla",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
