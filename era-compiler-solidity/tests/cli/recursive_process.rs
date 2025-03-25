//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing_input(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--recursive-process"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("Error: Stdin parsing error"));

    Ok(())
}
