use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test]
fn without_any_args() -> anyhow::Result<()> {
    common::setup()?;

    let args: &[&str] = &[];

    let result = cli::execute_zksolc(args)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: No input sources specified",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(Target::EraVM, common::SOLIDITY_BIN_OUTPUT_NAME_ERAVM)]
#[test_case(Target::EVM, common::SOLIDITY_BIN_OUTPUT_NAME_EVM)]
fn with_args_from_help_example(target: Target, bin_output_file_name: &str) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir = TempDir::new()?;
    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);

    assert!(bin_output_file.exists());
    assert!(!cli::is_file_empty(bin_output_file.to_str().unwrap())?);

    Ok(())
}

#[test_case(
    Target::EraVM,
    common::SOLIDITY_BIN_OUTPUT_NAME_ERAVM,
    common::SOLIDITY_ASM_OUTPUT_NAME_ERAVM
)]
#[test_case(
    Target::EVM,
    common::SOLIDITY_BIN_OUTPUT_NAME_EVM,
    common::SOLIDITY_ASM_OUTPUT_NAME_EVM
)]
fn with_multiple_output_options(
    target: Target,
    bin_output_file_name: &str,
    asm_output_file_name: &str,
) -> anyhow::Result<()> {
    common::setup()?;
    let tmp_dir = TempDir::new()?;
    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--asm",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);
    let asm_output_file = tmp_dir
        .path()
        .join(common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(asm_output_file_name);

    assert!(bin_output_file.exists());
    assert!(asm_output_file.exists());
    assert!(!cli::is_file_empty(bin_output_file.to_str().unwrap())?);
    assert!(!cli::is_file_empty(asm_output_file.to_str().unwrap())?);

    Ok(())
}

#[test_case(Target::EraVM, common::SOLIDITY_BIN_OUTPUT_NAME_ERAVM)]
#[test_case(Target::EVM, common::SOLIDITY_BIN_OUTPUT_NAME_EVM)]
fn with_bin_output_same_file_and_cli(
    target: Target,
    bin_output_file_name: &str,
) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir = TempDir::new()?;
    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    let bin_output_file = tmp_dir
        .path()
        .join(common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);
    assert!(bin_output_file.exists());

    let cli_args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];
    let cli_result = cli::execute_zksolc_with_target(cli_args, target)?;

    let stdout = String::from_utf8_lossy(cli_result.get_output().stdout.as_slice());

    assert!(cli::is_output_same_as_file(
        bin_output_file.to_str().unwrap(),
        stdout.trim()
    )?);

    Ok(())
}
