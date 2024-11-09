use crate::{cli, common};
use era_compiler_common::Target;
use tempfile::TempDir;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_debug_output_dir_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_debug = TempDir::with_prefix("debug_output")?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.success();

    Ok(())
}
