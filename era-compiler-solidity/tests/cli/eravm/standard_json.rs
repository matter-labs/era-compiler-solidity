//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_LLVM_IR_STANDARD_JSON_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn llvm_ir_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--standard-json",
        crate::common::TEST_LLVM_IR_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM IR projects cannot be compiled with `solc`",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn eravm_assembly_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--standard-json",
        crate::common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "EraVM assembly projects cannot be compiled with `solc`",
    ));

    Ok(())
}
