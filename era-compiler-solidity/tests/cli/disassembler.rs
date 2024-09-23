use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_disassemble_binary() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_DISASSEMBLER_BINARY_BYTECODE_PATH, "--disassemble"];
    let invalid_args = &["--disassemble", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    let invalid_result = cli::execute_zksolc(invalid_args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));
    invalid_result.failure();

    Ok(())
}

#[test]
fn run_zksolc_with_disassemble_hexadecimal() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli::TEST_DISASSEMBLER_HEXADECIMAL_BYTECODE_PATH,
        "--disassemble",
    ];
    let invalid_args = &["--disassemble", "anyarg"];

    let result = cli::execute_zksolc(args)?;
    let invalid_result = cli::execute_zksolc(invalid_args)?;

    result
        .success()
        .stderr(predicate::str::contains("disassembly:"));
    invalid_result.failure();

    Ok(())
}
