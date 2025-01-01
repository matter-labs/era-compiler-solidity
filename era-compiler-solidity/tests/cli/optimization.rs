//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
// TODO: #[test_case(Target::EVM)]
fn all(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let levels = ["0", "1", "2", "3", "s", "z"];

    for level in levels.into_iter() {
        let args = &[
            crate::common::TEST_SOLIDITY_CONTRACT_PATH,
            &format!("-O{}", level),
        ];

        let result = crate::cli::execute_zksolc_with_target(args, target)?;
        result
            .success()
            .stderr(predicate::str::contains("Compiler run successful"));
    }

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
