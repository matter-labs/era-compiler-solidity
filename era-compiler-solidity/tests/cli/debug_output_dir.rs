use crate::{cli, common};
use tempfile::TempDir;

#[test]
fn with_debug_output_dir_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_debug = TempDir::with_prefix("debug_output")?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc(args)?;

    result.success();

    Ok(())
}
