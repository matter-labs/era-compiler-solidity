use crate::{cli, common};
use predicates::prelude::*;
use era_compiler_common::Target;

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
        "Error: No other options except bytecode files, `--libraries`, and `--target` are allowed in linker mode.",
    ));

    Ok(())
}

#[test]
fn with_target_evm() -> anyhow::Result<()> {
    common::setup()?;

    let target = Target::EVM.to_string();
    let args = &["--link", cli::TEST_LINKER_BYTECODE_PATH, "--target", target.as_str()];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support linking yet.",
    ));

    Ok(())
}
