use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_version() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--version"];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Solidity compiler for ZKsync"));

    Ok(())
}

#[test]
fn with_version_and_extra_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--version", common::TEST_SOLIDITY_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed while getting the compiler version.",
    ));

    Ok(())
}
