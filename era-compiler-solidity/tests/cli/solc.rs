use assert_cmd::Command;

use crate::{cli, common};
use predicates::prelude::predicate;

#[test]
fn call_zksolc_with_solc_argument() -> anyhow::Result<()> {
    let _ = common::setup();

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let assert = zksolc
        .arg(cli::TEST_SOLIDITY_CONTRACT_PATH)
        .arg("--solc")
        .arg(solc_compiler)
        .assert();

    assert
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn call_zksolc_with_solc_in_standard_json_mode() -> anyhow::Result<()> {
    let _ = common::setup();

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let assert = zksolc
        .arg("--solc")
        .arg(solc_compiler)
        .arg("--standard-json")
        .arg(cli::TEST_JSON_CONTRACT_PATH)
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn call_zksolc_without_solc_in_standard_json_mode() -> anyhow::Result<()> {
    let _ = common::setup();

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;

    let assert = zksolc
        .arg("--standard-json")
        .arg(cli::TEST_JSON_CONTRACT_PATH)
        .assert();

    assert.failure().stderr(predicate::str::contains("Error"));

    Ok(())
}

#[test]
fn call_zksolc_without_solc_argument() -> anyhow::Result<()> {
    let _ = common::setup();

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;

    let assert = zksolc
        .arg(cli::TEST_SOLIDITY_CONTRACT_PATH)
        .env("PATH", "./solc-bin")
        .assert();

    assert
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}
