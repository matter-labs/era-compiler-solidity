use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn warning_evm() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];

    let result = cli::execute_zksolc_with_target(args, era_compiler_common::Target::EVM)?;
    result.success().stderr(predicate::str::contains(
        "EVM target is under development and not fully functional yet.",
    ));

    Ok(())
}
