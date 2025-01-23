//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, '0')]
#[test_case(Target::EraVM, '1')]
#[test_case(Target::EraVM, '2')]
#[test_case(Target::EraVM, '3')]
#[test_case(Target::EraVM, 's')]
#[test_case(Target::EraVM, 'z')]
// TODO: #[test_case(Target::EVM, '0')]
// TODO: #[test_case(Target::EVM, '1')]
// TODO: #[test_case(Target::EVM, '2')]
#[test_case(Target::EVM, '3')]
#[test_case(Target::EVM, 's')]
#[test_case(Target::EVM, 'z')]
fn all(target: Target, level: char) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        &format!("-O{level}"),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "-O", "99"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(
        predicate::str::contains("Unexpected optimization option")
            .or(predicate::str::contains("error: invalid value \'99\' for \'--optimization <OPTIMIZATION>\': too many characters in string")),
    );

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "-O",
        "3",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "LLVM optimizations must be specified in standard JSON input settings.",
    ));

    Ok(())
}
