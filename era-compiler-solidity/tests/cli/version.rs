//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--version"];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Solidity compiler for ZKsync"));

    Ok(())
}

#[test]
fn excess_args() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--version", crate::common::TEST_SOLIDITY_CONTRACT_PATH];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed while getting the compiler version.",
    ));

    Ok(())
}
