//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn warning_evm() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];

    let result = crate::cli::execute_zksolc_with_target(args, era_compiler_common::Target::EVM)?;
    result.success().stderr(predicate::str::contains(
        "EVM target is under development and not fully functional yet.",
    ));

    Ok(())
}
