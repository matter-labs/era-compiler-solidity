use crate::{cli, common};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn with_overwrite_bin() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn without_overwrite_bin() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
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

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_overwrite_asm() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn without_overwrite_asm() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--asm",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
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

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_overwrite_metadata() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn without_overwrite_metadata() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
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

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_overwrite_all() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn without_overwrite_all() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
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

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_overwrite_combined_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
        "--overwrite",
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
        "--overwrite",
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    assert!(tmp_dir_zksolc.path().exists());

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn without_overwrite_combined_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;
    let tmp_dir_solc = TempDir::with_prefix("solc_output")?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];
    let solc_args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "bin,asm,metadata",
        "--output-dir",
        tmp_dir_solc.path().to_str().unwrap(),
    ];

    let _ = cli::execute_zksolc(args)?;
    let result = cli::execute_zksolc(args)?;
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

    let _ = cli::execute_solc(solc_args)?;
    let solc_result = cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn with_overwrite_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--output-dir",
        "output",
        "--overwrite",
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stdout(predicate::str::contains(
        "Overwriting flag cannot be used in standard JSON mode.",
    ));

    Ok(())
}
