//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn with_libraries() -> anyhow::Result<()> {
    crate::common::setup()?;

    std::fs::create_dir_all(crate::common::TEST_TEMP_DIRECTORY)?;
    std::fs::copy(
        crate::common::TEST_LINKER_BYTECODE_PATH,
        crate::common::TEST_LINKER_BYTECODE_COPY_PATH,
    )?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_COPY_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"linked\":{\"tests/data/temp/linker_copy.zbin\":",
    ));

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"ignored\":{\"tests/data/temp/linker_copy.zbin\":",
    ));

    std::fs::remove_file(crate::common::TEST_LINKER_BYTECODE_COPY_PATH)?;

    Ok(())
}

#[test]
fn without_libraries() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--link", crate::common::TEST_LINKER_BYTECODE_PATH];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        r#""unlinked":{"tests/data/bytecodes/linker.zbin":{"linker_symbols":["Greeter.sol:GreeterHelper"],"factory_dependencies":[]}}"#,
    ));

    Ok(())
}

#[test]
fn without_libraries_linker_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--link", crate::common::TEST_LINKER_ERROR_BYTECODE_PATH];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "ld.lld: error: undefined symbol: foo",
    ));

    Ok(())
}

#[test]
fn with_libraries_excess_args() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER,
        "--bin",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: No other options except bytecode files, `--libraries`, `--standard-json` are allowed in linker mode.",
    ));

    Ok(())
}

#[test]
fn with_libraries_contract_name_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER_CONTRACT_NAME_MISSING,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `Greeter.sol` contract name is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER_ADDRESS_MISSING,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `Greeter.sol:GreeterHelper` address is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_invalid() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER_ADDRESS_INVALID,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Invalid address `XINVALID` of library `Greeter.sol:GreeterHelper`: Invalid character \'X\' at position 0.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_incorrect_size() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        crate::common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        crate::common::LIBRARY_LINKER_ADDRESS_INCORRECT_SIZE,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Incorrect size of address `0x12345678` of library `Greeter.sol:GreeterHelper`: expected 20, found 4.",
    ));

    Ok(())
}

#[test]
fn with_libraries_standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        crate::common::TEST_LINKER_STANDARD_JSON_INPUT_WITH_LIBRARIES_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"linked\":{\"tests/data/bytecodes/linker.zbin\":",
    ));

    Ok(())
}

#[test]
fn without_libraries_standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        crate::common::TEST_LINKER_STANDARD_JSON_INPUT_WITHOUT_LIBRARIES_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        r#""unlinked":{"tests/data/bytecodes/linker.zbin":{"linker_symbols":["Greeter.sol:GreeterHelper"],"factory_dependencies":[]}}"#,
    ));

    Ok(())
}

#[test]
fn standard_json_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Error: JSON file"));

    Ok(())
}

#[test]
fn standard_json_invalid_hexadecimal() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        crate::common::TEST_LINKER_STANDARD_JSON_INPUT_INVALID_HEXADECIMAL_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "hexadecimal string decoding: Invalid character \'I\' at position 0",
    ));

    Ok(())
}

#[test]
fn standard_json_linker_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        crate::common::TEST_LINKER_STANDARD_JSON_INPUT_LINKER_ERROR_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "ld.lld: error: undefined symbol: foo",
    ));

    Ok(())
}
