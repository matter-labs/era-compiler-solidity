use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_metadata_hash_default() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata-hash", "none"];

    let result = cli::execute_zksolc(args)?;
    let zksolc_result = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test]
fn with_metadata_hash_no_argument() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata-hash"];

    let result = cli::execute_zksolc(args)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains(
            "requires a value but none was supplied",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test]
fn with_metadata_hash_no_input_file() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--metadata-hash", "none"];

    let result = cli::execute_zksolc(args)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test]
fn with_metadata_hash_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--metadata-hash",
        "none",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}
