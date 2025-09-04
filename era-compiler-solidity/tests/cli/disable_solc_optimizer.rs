//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn deprecated() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Warning: `--disable-solc-optimizer` flag is deprecated: the `solc` optimizer is not used by `zksolc` anymore.",
    ));

    Ok(())
}
