use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_llvm_ir() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[cli::TEST_LLVM_IR_CONTRACT_PATH, "--llvm-ir"];
    let invalid_args = &["--llvm-ir", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    let invalid_result = cli::execute_zksolc(invalid_args)?;

    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));
    invalid_result.failure();

    Ok(())
}

#[test]
fn with_llvm_ir_duplicate_flag() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[cli::TEST_LLVM_IR_CONTRACT_PATH, "--llvm-ir", "--llvm-ir"];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The argument '--llvm-ir' was provided more than once",
    ));

    Ok(())
}

#[test]
fn with_wrong_input_format() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--llvm-ir", "--bin"];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}

#[test]
fn with_incompatible_json_modes_combined_json() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        cli::TEST_LLVM_IR_CONTRACT_PATH,
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
fn with_incompatible_json_modes_standard_json() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--llvm-ir", "--standard-json"];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test]
fn with_standard_json_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_LLVM_IR_STANDARD_JSON_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("error: use of undefined value"));

    Ok(())
}
