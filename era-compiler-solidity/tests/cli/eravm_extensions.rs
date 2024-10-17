use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_eravm_extensions() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--enable-eravm-extensions",
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test]
fn with_system_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--system-mode", "--bin"];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Warning: The `--system-mode` flag is deprecated. Please use `--enable-eravm-extensions` instead."));

    Ok(())
}
