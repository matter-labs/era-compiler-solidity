//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--enable-eravm-extensions",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--enable-eravm-extensions",
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EraVM extensions are only supported in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--enable-eravm-extensions",
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "EraVM extensions are only supported in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--enable-eravm-extensions",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "is deprecated in standard JSON mode and must be passed in JSON as",
    ));

    Ok(())
}

#[test]
fn deprecated_system_mode() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--system-mode",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Warning: `--system-mode` flag is deprecated: please use `--enable-eravm-extensions` instead"));

    Ok(())
}

#[test]
fn yul_literal_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        "--enable-eravm-extensions",
        "--asm",
        crate::common::TEST_YUL_CONTRACT_ERAVM_EXTENSION_LITERAL_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("meta"))
        .stdout(predicate::str::contains("@__staticcall").not());

    Ok(())
}

#[test]
fn yul_variable_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        "--enable-eravm-extensions",
        "--asm",
        crate::common::TEST_YUL_CONTRACT_ERAVM_EXTENSION_VARIABLE_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("@__staticcall"))
        .stdout(predicate::str::contains("65532"))
        .stdout(predicate::str::contains("meta").not());

    Ok(())
}

#[test]
fn yul_reassigned_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        "--enable-eravm-extensions",
        "--asm",
        crate::common::TEST_YUL_CONTRACT_ERAVM_EXTENSION_REASSIGNED_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("@__staticcall"))
        .stdout(predicate::str::contains("32770"))
        .stdout(predicate::str::contains("meta").not());

    Ok(())
}

#[test]
fn yul_sibling_scope_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        "--enable-eravm-extensions",
        "--asm",
        crate::common::TEST_YUL_CONTRACT_ERAVM_EXTENSION_SIBLING_SCOPE_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("@__staticcall"))
        .stdout(predicate::str::contains("meta").not());

    Ok(())
}
