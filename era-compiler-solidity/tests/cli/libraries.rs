use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_solidity() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn without_solidity() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--libraries"];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "requires a value but none was supplied",
    ));

    Ok(())
}

#[test]
fn with_libraries_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_STANDARD_JSON_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stdout(predicate::str::contains(
        "Libraries must be passed via standard JSON input.",
    ));

    Ok(())
}
