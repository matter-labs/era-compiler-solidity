use crate::{cli, common};
use era_compiler_common::Target;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_threads_standard_json_mode(target: Target) -> anyhow::Result<()> {
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
