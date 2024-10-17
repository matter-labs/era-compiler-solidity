use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_suppressed_errors_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let error_type = era_compiler_solidity::MessageType::SendTransfer.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--suppress-errors",
        error_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Suppressed errors must be specified in standard JSON input settings.",
    ));

    Ok(())
}
