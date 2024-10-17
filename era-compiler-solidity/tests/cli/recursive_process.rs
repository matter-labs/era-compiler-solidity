use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_recursive_process_without_target() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--recursive-process"];

    let result = cli::execute_zksolc(args)?;
    result.success();

    Ok(())
}

#[test]
fn with_recursive_process() -> anyhow::Result<()> {
    common::setup()?;

    let target = era_compiler_common::Target::EraVM.to_string();
    let args = &["--recursive-process", "--target", target.as_str()];

    let result = cli::execute_zksolc(args)?;
    result.success();

    Ok(())
}

#[test]
fn with_recursive_process_and_extra_args() -> anyhow::Result<()> {
    common::setup()?;

    let target = era_compiler_common::Target::EraVM.to_string();
    let args = &[
        "--recursive-process",
        "--target",
        target.as_str(),
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed in recursive mode.",
    ));

    Ok(())
}
