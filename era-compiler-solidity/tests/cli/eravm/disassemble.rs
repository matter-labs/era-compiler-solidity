//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_DISASSEMBLER_BYTECODE_PATH,
        "--disassemble",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--disassemble", "anyarg"];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure();

    Ok(())
}

#[test]
fn excess_arguments() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--disassemble",
        crate::common::TEST_DISASSEMBLER_BYTECODE_PATH,
        "--bin",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "No other options except input files and `--target` are allowed in disassembler mode.",
    ));

    Ok(())
}

#[test]
fn unimplemented_evm() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--disassemble",
        crate::common::TEST_DISASSEMBLER_BYTECODE_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, Target::EVM)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support disassembling yet.",
    ));

    Ok(())
}
