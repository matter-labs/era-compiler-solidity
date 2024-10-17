use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_eravm_extensions() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--enable-eravm-extensions",
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_system_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--system-mode", "--bin"];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Warning: The `--system-mode` flag is deprecated. Please use `--enable-eravm-extensions` instead."));

    Ok(())
}

#[test]
fn with_eravm_extensions_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--enable-eravm-extensions",
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EraVM extensions are only supported in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn with_eravm_extensions_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--enable-eravm-extensions",
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EraVM extensions are only supported in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn with_enable_eravm_extensions_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--enable-eravm-extensions",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}
