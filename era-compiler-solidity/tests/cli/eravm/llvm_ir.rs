//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result.success().stdout(predicate::str::contains("Binary"));

    Ok(())
}
