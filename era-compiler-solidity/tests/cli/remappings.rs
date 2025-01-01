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
        "./path/to/1.sol=./path/to/2.sol",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn excess_equals_sign(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "./path/to/1.sol=./path/to/2.sol=./path/to/3.sol",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "expected two parts separated by '='",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "./path/to/1.sol=./path/to/2.sol",
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Input files must be passed via standard JSON input.",
    ));

    Ok(())
}
