//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn deprecated(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--disable-solc-optimizer",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Warning: `--disable-solc-optimizer` flag is deprecated: the `solc` optimizer is not used by `zksolc` anymore.",
    ));

    Ok(())
}
