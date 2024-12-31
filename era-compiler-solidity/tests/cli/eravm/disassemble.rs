use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[common::TEST_DISASSEMBLER_BYTECODE_PATH, "--disassemble"];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    result.failure();

    Ok(())
}

#[test]
fn excess_arguments() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--disassemble",
        common::TEST_DISASSEMBLER_BYTECODE_PATH,
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options except input files and `--target` are allowed in disassembler mode.",
    ));

    Ok(())
}

#[test]
fn unimplemented_evm() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--disassemble", common::TEST_DISASSEMBLER_BYTECODE_PATH];

    let result = cli::execute_zksolc_with_target(args, Target::EVM)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support disassembling yet.",
    ));

    Ok(())
}
