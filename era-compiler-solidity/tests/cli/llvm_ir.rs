use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, common::TEST_LLVM_IR_CONTRACT_PATH)]
#[test_case(Target::EVM, common::TEST_LLVM_IR_CONTRACT_EVM_PATH)]
fn default(target: Target, path: &str) -> anyhow::Result<()> {
    common::setup()?;
    let args = &[path, "--llvm-ir"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input_text(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--llvm-ir", common::TEST_BROKEN_INPUT_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: expected top-level entity"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input_solidity(target: Target) -> anyhow::Result<()> {
    common::setup()?;
    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "--llvm-ir", "--bin"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("expected top-level entity"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input_llvm_ir(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--llvm-ir", common::TEST_LLVM_IR_CONTRACT_INVALID_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "error: use of undefined value \'%runtime\'",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
fn linker_error(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--llvm-ir", common::TEST_LLVM_IR_CONTRACT_LINKER_ERROR_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "ld.lld: error: undefined symbol: foo",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_incompatible_json_modes_combined_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_LLVM_IR_CONTRACT_PATH,
        "--llvm-ir",
        "--combined-json",
        "anyarg",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_incompatible_json_modes_standard_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_LLVM_IR_CONTRACT_PATH,
        "--llvm-ir",
        "--standard-json",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_invalid(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_LLVM_IR_STANDARD_JSON_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("error: use of undefined value"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_missing_file(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_LLVM_IR_STANDARD_JSON_MISSING_FILE_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Error: File \\\"tests/data/contracts/llvm_ir/Missing.ll\\\" reading:",
    ));

    Ok(())
}
