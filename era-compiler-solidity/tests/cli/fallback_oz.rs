use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_fallback_oz() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--fallback-Oz", "--bin"];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_fallback_oz_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--fallback-Oz",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Falling back to -Oz must be specified in standard JSON input settings.",
    ));

    Ok(())
}
