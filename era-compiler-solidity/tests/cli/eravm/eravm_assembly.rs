use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_eravm_assembly() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        "--bin",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_eravm_assembly_duplicate_flag() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "error: the argument \'--eravm-assembly\' cannot be used multiple times",
    ));

    Ok(())
}

#[test]
fn with_eravm_assembly_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn with_wrong_input_format() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        "--eravm-assembly",
        "--bin",
        common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn with_incompatible_input_format_without_output() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--eravm-assembly", common::TEST_BROKEN_INPUT_PATH];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn with_incompatible_json_modes_combined_json() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--combined-json",
        "anyarg",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test]
fn with_incompatible_json_modes_standard_json() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--standard-json",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test]
fn with_incompatible_json_modes() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--eravm-assembly",
        "--combined-json",
        "wrong",
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Only one mode is allowed"));

    Ok(())
}

#[test]
fn with_target_evm() -> anyhow::Result<()> {
    common::setup()?;

    let target = era_compiler_common::Target::EVM.to_string();
    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--target",
        target.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: EraVM assembly cannot be compiled to EVM bytecode.",
    ));

    Ok(())
}

#[test]
fn with_optimization() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "-O",
        "3",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "LLVM optimizations are not supported in EraVM assembly mode.",
    ));

    Ok(())
}

#[test]
fn with_fallback_to_optimizing_for_size() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--fallback-Oz",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Falling back to -Oz is not supported in EraVM assembly mode.",
    ));

    Ok(())
}

#[test]
fn with_standard_json_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn with_standard_json_missing_file() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_MISSING_FILE_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Error: File \\\"tests/data/contracts/eravm_assembly/Missing.zasm\\\" reading:",
    ));

    Ok(())
}
