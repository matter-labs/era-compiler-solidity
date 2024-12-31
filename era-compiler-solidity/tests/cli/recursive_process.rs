use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing_input(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--recursive-process"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("Error: Stdin parsing error"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn excess_arguments(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--recursive-process", common::TEST_SOLIDITY_CONTRACT_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "No other options are allowed in recursive mode.",
    ));

    Ok(())
}
