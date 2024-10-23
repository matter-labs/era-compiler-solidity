use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_disable_solc_optimizer() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Warning: The `solc` optimizer is not used by `zksolc` anymore.",
    ));

    Ok(())
}
