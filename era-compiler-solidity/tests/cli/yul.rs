use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_against_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_YUL_CONTRACT_PATH, "--yul", "--bin"];
    let solc_args = &[common::TEST_YUL_CONTRACT_PATH, "--strict-assembly"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let zksolc_status = result
        .success()
        .stdout(predicate::str::contains("Binary:\n"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_invalid_against_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--yul", "anyarg", "--bin"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains("Error"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_double_against_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_YUL_CONTRACT_PATH, "--yul", "--yul"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--yul' was provided more than once",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_invalid_input_file(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "--yul"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("Yul parsing"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_and_combined_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--combined-json",
        "anyarg",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Only one mode is allowed at the same time:",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_and_standard_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_YUL_CONTRACT_PATH, "--yul", "--standard-json"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time:",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_yul_and_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_and_solc_invalid_by_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        common::TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "DeclarationError: Function \\\"mdelete\\\" not found.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_invalid_by_zksolc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_and_solc_invalid_by_zksolc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        common::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}
