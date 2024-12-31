use crate::{cli, common};
use era_compiler_common::Target;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "--threads", "1"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--threads",
        "1",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}
