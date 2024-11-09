use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
// TODO: #[test_case(Target::EVM)]
fn with_optimization_levels(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let levels = ["0", "1", "2", "3", "s", "z"];

    for level in levels.into_iter() {
        let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, &format!("-O{}", level)];

        let result = cli::execute_zksolc_with_target(args, target)?;
        result
            .success()
            .stderr(predicate::str::contains("Compiler run successful"));
    }

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_optimization_no_input_file(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["-O", "0"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_invalid_optimization_option(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "-O", "99"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(
        predicate::str::contains("Unexpected optimization option")
            .or(predicate::str::contains("Invalid value for")),
    );

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_optimization_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "-O",
        "3",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "LLVM optimizations must be specified in standard JSON input settings.",
    ));

    Ok(())
}
