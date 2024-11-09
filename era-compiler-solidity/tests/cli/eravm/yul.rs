use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn with_yul_and_solc_and_eravm_extensions() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        common::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
        "--enable-eravm-extensions",
    ];

    let result = cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result
        .failure()
        .stderr(predicate::str::contains("Yul validation cannot be done if EraVM extensions are enabled. Consider compiling without `solc`."));

    Ok(())
}
