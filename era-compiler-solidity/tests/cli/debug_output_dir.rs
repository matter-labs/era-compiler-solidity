//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use tempfile::TempDir;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_debug = TempDir::with_prefix("debug_output")?;

    let args = &[
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_debug = TempDir::with_prefix("debug_output")?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success();

    Ok(())
}
