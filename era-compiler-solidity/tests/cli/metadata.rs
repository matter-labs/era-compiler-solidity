use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_metadata_combined_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--metadata",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "metadata",
    ];

    let result = cli::execute_zksolc(args)?;

    result.failure().stderr(predicate::str::contains(
        "Cannot output data outside of JSON in combined JSON mode.",
    ));

    Ok(())
}

#[test]
fn with_metadata_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata",
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stdout(predicate::str::contains(
        "Cannot output data outside of JSON in standard JSON mode.",
    ));

    Ok(())
}
