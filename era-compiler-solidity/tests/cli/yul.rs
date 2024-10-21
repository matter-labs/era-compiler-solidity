use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_yul_against_solc() -> anyhow::Result<()> {
    common::setup()?;

    let zksolc_args = &[cli::TEST_YUL_CONTRACT_PATH, "--yul"];
    let solc_args = &[cli::TEST_YUL_CONTRACT_PATH, "--strict-assembly"];

    let result = cli::execute_zksolc(zksolc_args)?;
    let zksolc_status = result
        .success()
        .stderr(predicate::str::contains(
            "Compiler run successful. No output requested",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn with_yul_invalid_against_solc() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--yul", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains("Error"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_yul_double_against_solc() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--yul", "--yul"];

    let result = cli::execute_zksolc(args)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "The argument '--yul' was provided more than once",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_yul_invalid_input_file() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--yul"];

    let result = cli::execute_zksolc(args)?;
    let zksolc_status = result
        .failure()
        .stderr(predicate::str::contains("Yul parsing"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_status);

    Ok(())
}

#[test]
fn with_yul_and_combined_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--combined-json",
        "anyarg",
    ];

    let result = cli::execute_zksolc(args)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Only one mode is allowed at the same time:",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_yul_and_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_YUL_CONTRACT_PATH, "--yul", "--standard-json"];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Only one mode is allowed at the same time:",
    ));

    Ok(())
}

#[test]
fn with_yul_and_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        cli::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stderr(predicate::str::contains(
        "Compiler run successful. No output requested",
    ));

    Ok(())
}

#[test]
fn with_yul_and_solc_and_eravm_extensions() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        cli::TEST_YUL_CONTRACT_PATH,
        "--yul",
        "--solc",
        solc_compiler.as_str(),
        "--enable-eravm-extensions",
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Yul validation cannot be done if EraVM extensions are enabled. Consider compiling without `solc`."));

    Ok(())
}

#[test]
fn with_standard_json_and_solc_invalid_by_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "DeclarationError: Function \\\"mdelete\\\" not found.",
    ));

    Ok(())
}

#[test]
fn with_standard_json_invalid_by_zksolc() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}

#[test]
fn with_standard_json_and_solc_invalid_by_zksolc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The `SELFDESTRUCT` instruction is not supported",
    ));

    Ok(())
}
