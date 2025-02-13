//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, crate::common::TEST_LLVM_IR_CONTRACT_PATH)]
fn default(target: Target, path: &str) -> anyhow::Result<()> {
    crate::common::setup()?;
    let args = &[path, "--llvm-ir"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));

    Ok(())
}
