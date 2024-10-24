use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn with_bytecode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_DISASSEMBLER_BINARY_BYTECODE_PATH, "--disassemble"];

    let result = cli::execute_zksolc(args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));

    Ok(())
}

#[test]
fn with_bytecode_hexadecimal() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_DISASSEMBLER_HEXADECIMAL_BYTECODE_PATH,
        "--disassemble",
    ];

    let result = cli::execute_zksolc(args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));

    Ok(())
}

#[test]
fn with_bytecode_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", "anyarg"];

    let result = cli::execute_zksolc(args)?;

    result.failure();

    Ok(())
}

#[test]
fn with_bytecode_and_extra_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disassemble",
        cli::TEST_DISASSEMBLER_BINARY_BYTECODE_PATH,
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;

    result.failure().stderr(predicate::str::contains(
        "No other options except input files and `--target` are allowed in disassembler mode.",
    ));

    Ok(())
}

#[test]
fn with_invalid_file_extensions() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", cli::TEST_SOLIDITY_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;

    result
        .failure()
        .stderr(predicate::str::contains("Invalid file extension"));

    Ok(())
}

#[test]
fn with_target_evm() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", cli::TEST_DISASSEMBLER_BINARY_BYTECODE_PATH];

    let result = cli::execute_zksolc_with_target(args, Target::EVM)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support disassembling yet.",
    ));

    Ok(())
}
