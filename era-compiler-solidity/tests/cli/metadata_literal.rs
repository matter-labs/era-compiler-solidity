use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_metadata_literal_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-literal",
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Metadata literal content flag must be specified in standard JSON input settings.",
    ));

    Ok(())
}
