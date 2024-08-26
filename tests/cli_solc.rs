#![cfg(test)]

pub mod cli_tests;
pub mod common;
use assert_cmd::Command;

use predicates::prelude::predicate;

#[test]
fn call_zksolc_with_solc_argument() -> anyhow::Result<()> {
    let _ = common::setup();

    let mut zksolc = Command::cargo_bin("zksolc")?;
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let assert = zksolc
        .arg(cli_tests::TEST_SOLIDITY_CONTRACT_PATH)
        .arg("--solc")
        .arg(solc_compiler)
        .assert();

    assert
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}
