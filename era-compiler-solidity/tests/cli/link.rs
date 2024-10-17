use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_libraries() -> anyhow::Result<()> {
    common::setup()?;

    std::fs::copy(
        cli::TEST_LINKER_BYTECODE_PATH,
        cli::TEST_LINKER_BYTECODE_COPY_PATH,
    )?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_COPY_PATH,
        "--libraries",
        cli::LIBRARY_LINKER,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"linked\":{\"tests/data/bytecodes/linker_copy.hex\":",
    ));

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"ignored\":{\"tests/data/bytecodes/linker_copy.hex\":",
    ));

    std::fs::remove_file(cli::TEST_LINKER_BYTECODE_COPY_PATH)?;

    Ok(())
}

#[test]
fn without_libraries() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--link", cli::TEST_LINKER_BYTECODE_PATH];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"unlinked\":{\"tests/data/bytecodes/linker.hex\":[\"test.sol:GreaterHelper\"]}}",
    ));

    Ok(())
}

#[test]
fn with_libraries_and_extra_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_COPY_PATH,
        "--libraries",
        cli::LIBRARY_LINKER,
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: No other options except files with bytecode and `--libraries` are allowed in linker mode.",
    ));

    Ok(())
}
