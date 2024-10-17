use assert_cmd::Command;

use crate::{cli, common};
use predicates::prelude::predicate;

#[test]
fn with_solc() -> anyhow::Result<()> {
    common::setup()?;

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
fn without_solc() -> anyhow::Result<()> {
    common::setup()?;

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

#[test]
fn with_solc_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let assert = zksolc
        .arg("--solc")
        .arg(solc_compiler)
        .arg("--standard-json")
        .arg(cli::TEST_STANDARD_JSON_PATH)
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn without_solc_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;

    let assert = zksolc
        .arg("--standard-json")
        .arg(cli::TEST_STANDARD_JSON_PATH)
        .assert();

    assert.success().stdout(predicate::str::contains(
        "The `solc` executable not found in ${PATH}",
    ));

    Ok(())
}

#[test]
fn with_solc_llvm_ir_mode() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test]
fn with_solc_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}
