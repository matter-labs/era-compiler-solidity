use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_suppressed_warnings_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let warning_type = era_compiler_solidity::MessageType::TxOrigin.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--suppress-warnings",
        warning_type.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Suppressed warnings must be specified in standard JSON input settings.",
    ));

    Ok(())
}
