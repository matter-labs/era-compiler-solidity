use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_sol_detect_missing_libraries() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}

#[test]
fn run_zksolc_without_sol_detect_missing_libraries() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--detect-missing-libraries"];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}
