use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_llvm_ir_by_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir"];
    let invalid_args = &["--llvm-ir", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    let invalid_result = cli::execute_zksolc(invalid_args)?;

    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested. Use --asm and --bin flags.",
    ));
    invalid_result.failure();

    Ok(())
}

#[test]
fn run_zksolc_with_same_llvm_ir_flags() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_LLVM_CONTRACT_PATH, "--llvm-ir", "--llvm-ir"];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The argument '--llvm-ir' was provided more than once",
    ));

    Ok(())
}

#[test]
fn run_zksolc_with_wrong_input_format() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--llvm-ir", "--bin"];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}

#[test]
fn run_zksolc_with_incompatible_json_modes_combined_json() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_LLVM_CONTRACT_PATH,
        "--llvm-ir",
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
fn run_zksolc_with_incompatible_json_modes_standard_json() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--llvm-ir", "--standard-json"];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}
