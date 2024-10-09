use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_llvm_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--llvm-options='-eravm-disable-system-request-memoization 10'",
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested.",
    ));

    Ok(())
}
