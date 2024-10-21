use crate::{cli, common};

#[test]
fn with_threads_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--threads",
        "1",
    ];

    let result = cli::execute_zksolc(args)?;

    result.success();

    Ok(())
}
