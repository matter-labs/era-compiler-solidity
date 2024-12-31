use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
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
fn invalid_input_text() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--eravm-assembly", common::TEST_BROKEN_INPUT_PATH];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: cannot parse operand"));

    Ok(())
}

#[test]
fn invalid_input_solidity() -> anyhow::Result<()> {
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
fn invalid_input_assembly() -> anyhow::Result<()> {
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
fn combined_json() -> anyhow::Result<()> {
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
fn standard_json() -> anyhow::Result<()> {
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
fn unsupported_evm() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, era_compiler_common::Target::EVM)?;
    result.failure().stderr(predicate::str::contains(
        "Error: EraVM assembly cannot be compiled to EVM bytecode.",
    ));

    Ok(())
}

#[test]
fn optimization() -> anyhow::Result<()> {
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
fn fallback_to_optimizing_for_size() -> anyhow::Result<()> {
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
fn standard_json_missing_file() -> anyhow::Result<()> {
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

#[test]
fn standard_json_invalid_assembly() -> anyhow::Result<()> {
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
