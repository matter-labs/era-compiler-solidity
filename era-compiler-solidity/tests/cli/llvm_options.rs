use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_llvm_options() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));

    Ok(())
}

#[test]
fn with_llvm_options_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM options must be specified in standard JSON input settings.",
    ));

    Ok(())
}
