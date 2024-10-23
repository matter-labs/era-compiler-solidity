use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
/// TODO: EVM
fn with_metadata(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata"];
    let invalid_args = &["--metadata"];

    // Valid command
    let result = cli::execute_zksolc_with_target(args, target)?;
    let result_status_code = result
        .success()
        .stdout(predicate::str::contains("Metadata:\n"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // solc exit code == zksolc exit code
    let solc_result = cli::execute_solc(args)?;
    solc_result.code(result_status_code);

    // Run invalid: zksolc --metadata
    let invalid_result = cli::execute_zksolc_with_target(invalid_args, target)?;
    let invalid_result_status_code = invalid_result
        .failure()
        .stderr(
            predicate::str::contains("No input sources specified")
                .or(predicate::str::contains("Compilation aborted")),
        )
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    // Invalid solc exit code == Invalid zksolc exit code
    let invalid_solc_result = cli::execute_solc(invalid_args)?;
    invalid_solc_result.code(invalid_result_status_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_duplicate_flag(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata", "--metadata"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--metadata' was provided more than once",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_with_wrong_input_format(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--metadata"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let solc_result = cli::execute_solc(args)?;

    let result_exit_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Expected identifier but got 'StringLiteral'",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    solc_result.code(result_exit_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_combined_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--metadata",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "metadata",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.failure().stderr(predicate::str::contains(
        "Cannot output data outside of JSON in combined JSON mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.success().stdout(predicate::str::contains(
        "Cannot output data outside of JSON in standard JSON mode.",
    ));

    Ok(())
}
