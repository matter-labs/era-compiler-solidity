use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM, era_compiler_common::EXTENSION_ERAVM_BINARY)]
#[test_case(Target::EVM, era_compiler_common::EXTENSION_EVM_BINARY)]
fn with_output_dir_yul(target: Target, extension: &str) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;

    let input_path = PathBuf::from(common::TEST_YUL_CONTRACT_PATH);
    let input_file = input_path
        .file_name()
        .expect("Always exists")
        .to_str()
        .expect("Always valid");

    let args = &[
        input_path.to_str().expect("Always valid"),
        "--yul",
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    let output_file = tmp_dir_zksolc
        .path()
        .join(input_file)
        .join(format!("{input_file}.{extension}"));
    assert!(output_file.exists());

    Ok(())
}

#[test_case(Target::EraVM, common::SOLIDITY_ASM_OUTPUT_NAME_ERAVM)]
#[test_case(Target::EVM, common::SOLIDITY_ASM_OUTPUT_NAME_EVM)]
fn with_output_dir_with_asm_and_metadata(
    target: Target,
    asm_file_name: &str,
) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let mut asm_path = tmp_dir_zksolc.path().to_path_buf();
    asm_path.push(common::TEST_SOLIDITY_CONTRACT_NAME);
    asm_path.push(asm_file_name);

    let mut metadata_path = tmp_dir_zksolc.path().to_path_buf();
    metadata_path.push(common::TEST_SOLIDITY_CONTRACT_NAME);
    metadata_path.push("Test_meta.json");

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    assert!(asm_path.exists());
    assert!(metadata_path.exists());

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir_invalid_arg_no_path(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "--bin", "--output-dir"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains("error: The argument '--output-dir <output-directory>' requires a value but none was supplied"))
        .get_output().status.code().expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir_invalid_args_no_source(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir_specific_symbols(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("File!and#$%-XXXXXX")?;
    let tmp_dir_solc = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir_combined_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("File!and#$%-XXXXXX")?;
    let tmp_dir_solc = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_output_dir_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--output-dir",
        "output",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.success().stdout(predicate::str::contains(
        "Output directory cannot be used in standard JSON mode.",
    ));

    Ok(())
}
