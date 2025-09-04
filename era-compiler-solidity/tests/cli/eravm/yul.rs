//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn solc_enable_eravm_extensions() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
        "--enable-eravm-extensions",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Yul validation cannot be done if EraVM extensions are enabled. Consider compiling without `solc`."));

    Ok(())
}
