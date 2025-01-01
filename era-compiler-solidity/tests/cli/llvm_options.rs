//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "LLVM options must be specified in standard JSON input settings.",
    ));

    Ok(())
}
