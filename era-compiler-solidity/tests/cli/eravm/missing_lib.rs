use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn detect_missing_libraries() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--detect-missing-libraries",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Missing deployable libraries detection mode is only supported in standard JSON mode.",
    ));

    Ok(())
}
