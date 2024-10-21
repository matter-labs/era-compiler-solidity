use crate::{cli, common};
use era_compiler_common::Target;
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
        cli::TEST_LINKER_BYTECODE_PATH,
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
fn with_libraries_contract_name_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        cli::LIBRARY_LINKER_CONTRACT_NAME_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "{\"ignored\":{},\"linked\":{},\"unlinked\":{\"tests/data/bytecodes/linker.hex\":[\"test.sol:GreaterHelper\"]}}",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        cli::LIBRARY_LINKER_ADDRESS_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Address of the library `test.sol:GreaterHelper` is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        cli::LIBRARY_LINKER_ADDRESS_INVALID,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Invalid address of library `test.sol:GreaterHelper`: Odd number of digits.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_incorrect_size() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        cli::LIBRARY_LINKER_ADDRESS_INCORRECT_SIZE,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Invalid address size of library `test.sol:GreaterHelper`: expected 20, found 4.",
    ));

    Ok(())
}

#[test]
fn with_target_evm() -> anyhow::Result<()> {
    common::setup()?;

    let target = Target::EVM.to_string();
    let args = &[
        "--link",
        cli::TEST_LINKER_BYTECODE_PATH,
        "--target",
        target.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support linking yet.",
    ));

    Ok(())
}
