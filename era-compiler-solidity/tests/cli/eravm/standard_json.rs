use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", common::TEST_LLVM_IR_STANDARD_JSON_PATH];

    let result = cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn llvm_ir_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--standard-json",
        common::TEST_LLVM_IR_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result.success().stdout(predicate::str::contains(
        "LLVM IR projects cannot be compiled with `solc`",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn eravm_assembly_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--standard-json",
        common::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result.success().stdout(predicate::str::contains(
        "EraVM assembly projects cannot be compiled with `solc`",
    ));

    Ok(())
}
