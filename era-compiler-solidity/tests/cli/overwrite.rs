//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn bin(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn bin_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: Refusing to overwrite an existing file",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn asm(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn asm_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: Refusing to overwrite an existing file",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: Refusing to overwrite an existing file",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn all(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn all_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: Refusing to overwrite an existing file",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn combined_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn combined_json_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = crate::cli::execute_zksolc_with_target(args, target)?;
    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .failure()
        .stderr(predicate::str::contains(
            "Error: Refusing to overwrite an existing file",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = crate::cli::execute_solc(solc_args)?;
    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--output-dir",
        "output",
        "--overwrite",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Overwriting flag cannot be used in standard JSON mode.",
    ));

    Ok(())
}
