//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;

    let input_path = PathBuf::from(crate::common::TEST_LLVM_IR_CONTRACT_PATH);
    let input_file = input_path
        .file_name()
        .expect("Always exists")
        .to_str()
        .expect("Always valid");

    let args = &[
        input_path.to_str().expect("Always valid"),
        "--llvm-ir",
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    let output_file = tmp_dir_zksolc.path().join(input_file).join(format!(
        "{input_file}.{}",
        era_compiler_common::EXTENSION_ERAVM_BINARY
    ));
    assert!(output_file.exists());

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_zksolc = TempDir::with_prefix("zksolc_output")?;

    let input_path = PathBuf::from(crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH);
    let input_file = input_path
        .file_name()
        .expect("Always exists")
        .to_str()
        .expect("Always valid");

    let args = &[
        input_path.to_str().expect("Always valid"),
        "--eravm-assembly",
        "--bin",
        "--output-dir",
        tmp_dir_zksolc.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, Target::EraVM)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    let output_file = tmp_dir_zksolc.path().join(input_file).join(format!(
        "{input_file}.{}",
        era_compiler_common::EXTENSION_ERAVM_BINARY
    ));
    assert!(output_file.exists());

    Ok(())
}
