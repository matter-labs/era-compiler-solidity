pub mod cli_tests;
#[cfg(test)]
pub mod common;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn run_zksolc_without_any_args() -> anyhow::Result<()> {
    let _ = common::setup();
    let args: &[&str] = &[];

    let result = cli_tests::execute_zksolc(args)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Compiles the provided Solidity input files",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test]
fn default_run_of_zksolc_from_the_help() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir = TempDir::new()?;
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(cli_tests::TEST_SOLIDITY_CONTRACT_NAME)
        .join(cli_tests::SOLIDITY_BIN_OUTPUT_NAME);

    assert!(bin_output_file.exists());
    assert!(!cli_tests::is_file_empty(
        &bin_output_file.to_str().unwrap()
    )?);

    Ok(())
}

#[test]
fn run_zksolc_with_multiple_output_options() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir = TempDir::new()?;
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--asm",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(cli_tests::TEST_SOLIDITY_CONTRACT_NAME)
        .join(cli_tests::SOLIDITY_BIN_OUTPUT_NAME);
    let asm_output_file = tmp_dir
        .path()
        .join(cli_tests::TEST_SOLIDITY_CONTRACT_NAME)
        .join(cli_tests::SOLIDITY_ASM_OUTPUT_NAME);

    assert!(bin_output_file.exists());
    assert!(asm_output_file.exists());
    assert!(!cli_tests::is_file_empty(
        &bin_output_file.to_str().unwrap()
    )?);
    assert!(!cli_tests::is_file_empty(
        &asm_output_file.to_str().unwrap()
    )?);

    Ok(())
}

#[test]
fn bin_output_is_the_same_in_file_and_cli() -> anyhow::Result<()> {
    let _ = common::setup();
    let tmp_dir = TempDir::new()?;
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli_tests::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    let bin_output_file = tmp_dir
        .path()
        .join(cli_tests::TEST_SOLIDITY_CONTRACT_NAME)
        .join(cli_tests::SOLIDITY_BIN_OUTPUT_NAME);
    assert!(bin_output_file.exists());

    let cli_args = &[cli_tests::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];
    let cli_result = cli_tests::execute_zksolc(cli_args)?;

    let stderr =
        String::from_utf8(cli_result.get_output().clone().stdout).expect("Invalid UTF-8 sequence");

    assert!(cli_tests::is_output_same_as_file(
        &bin_output_file.to_str().unwrap(),
        &stderr.as_str()
    )?);

    Ok(())
}
