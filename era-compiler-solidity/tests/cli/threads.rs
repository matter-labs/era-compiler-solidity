//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--threads", "1"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--threads",
        "1",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}
