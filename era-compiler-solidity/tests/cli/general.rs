//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test]
fn no_arguments() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args: &[&str] = &[];

    let result = crate::cli::execute_zksolc(args)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: No input sources specified",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(
    Target::EraVM,
    crate::common::SOLIDITY_BIN_OUTPUT_NAME_ERAVM,
    crate::common::SOLIDITY_ASM_OUTPUT_NAME_ERAVM
)]
#[test_case(
    Target::EVM,
    crate::common::SOLIDITY_BIN_OUTPUT_NAME_EVM,
    crate::common::SOLIDITY_ASM_OUTPUT_NAME_EVM
)]
fn multiple_output_options(
    target: Target,
    bin_output_file_name: &str,
    asm_output_file_name: &str,
) -> anyhow::Result<()> {
    crate::common::setup()?;
    let tmp_dir = TempDir::new()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--asm",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    assert!(tmp_dir.path().exists());

    let bin_output_file = tmp_dir
        .path()
        .join(crate::common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);
    let asm_output_file = tmp_dir
        .path()
        .join(crate::common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(asm_output_file_name);

    assert!(bin_output_file.exists());
    assert!(asm_output_file.exists());
    assert!(!crate::cli::is_file_empty(
        bin_output_file.to_str().unwrap()
    )?);
    assert!(!crate::cli::is_file_empty(
        asm_output_file.to_str().unwrap()
    )?);

    Ok(())
}

#[test_case(Target::EraVM, crate::common::SOLIDITY_BIN_OUTPUT_NAME_ERAVM)]
#[test_case(Target::EVM, crate::common::SOLIDITY_BIN_OUTPUT_NAME_EVM)]
fn same_output_directory_and_terminal(
    target: Target,
    bin_output_file_name: &str,
) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir = TempDir::new()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "-O3",
        "--bin",
        "--output-dir",
        tmp_dir.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful."));

    let bin_output_file = tmp_dir
        .path()
        .join(crate::common::TEST_SOLIDITY_CONTRACT_NAME)
        .join(bin_output_file_name);
    assert!(bin_output_file.exists());

    let cli_args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "-O3", "--bin"];
    let cli_result = crate::cli::execute_zksolc_with_target(cli_args, target)?;

    let stdout = String::from_utf8_lossy(cli_result.get_output().stdout.as_slice());
    assert!(crate::cli::is_output_same_as_file(
        bin_output_file.to_str().unwrap(),
        stdout.trim()
    )?);

    Ok(())
}
