use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn run_zksolc_with_libraries() -> anyhow::Result<()> {
    let _ = common::setup();

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
        "\"linked\":{\"tests/examples/bytecodes/linker_copy.hex\":",
    ));

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"ignored\":{\"tests/examples/bytecodes/linker_copy.hex\":",
    ));

    std::fs::remove_file(cli::TEST_LINKER_BYTECODE_COPY_PATH)?;

    Ok(())
}

#[test]
fn run_zksolc_without_libraries() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--link", cli::TEST_LINKER_BYTECODE_PATH];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"unlinked\":{\"tests/examples/bytecodes/linker.hex\":[\"test.sol:GreaterHelper\"]}}",
    ));

    Ok(())
}
