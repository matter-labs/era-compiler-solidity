use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;

#[test]
fn with_libraries() -> anyhow::Result<()> {
    common::setup()?;

    std::fs::create_dir_all(common::TEST_TEMP_DIRECTORY)?;
    std::fs::copy(
        common::TEST_LINKER_BYTECODE_PATH,
        common::TEST_LINKER_BYTECODE_COPY_PATH,
    )?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_COPY_PATH,
        "--libraries",
        common::LIBRARY_LINKER,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"linked\":{\"tests/data/temp/linker_copy.zbin\":",
    ));

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"ignored\":{\"tests/data/temp/linker_copy.zbin\":",
    ));

    std::fs::remove_file(common::TEST_LINKER_BYTECODE_COPY_PATH)?;

    Ok(())
}

#[test]
fn without_libraries() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--link", common::TEST_LINKER_BYTECODE_PATH];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        r#""unlinked":{"tests/data/bytecodes/linker.zbin":{"linker_symbols":["Greeter.sol:GreeterHelper"],"factory_dependencies":[]}}"#,
    ));

    Ok(())
}

#[test]
fn with_libraries_and_extra_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        common::LIBRARY_LINKER,
        "--bin",
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: No other options except bytecode files, `--libraries`, `--standard-json`, `--target` are allowed in linker mode.",
    ));

    Ok(())
}

#[test]
fn with_libraries_contract_name_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        common::LIBRARY_LINKER_CONTRACT_NAME_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `Greeter.sol` contract name is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        common::LIBRARY_LINKER_ADDRESS_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `Greeter.sol:GreeterHelper` address is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        common::LIBRARY_LINKER_ADDRESS_INVALID,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Invalid address `XINVALID` of library `Greeter.sol:GreeterHelper`: Invalid character \'X\' at position 0.",
    ));

    Ok(())
}

#[test]
fn with_libraries_address_incorrect_size() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--libraries",
        common::LIBRARY_LINKER_ADDRESS_INCORRECT_SIZE,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Incorrect size of address `0x12345678` of library `Greeter.sol:GreeterHelper`: expected 20, found 4.",
    ));

    Ok(())
}

#[test]
fn with_libraries_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        common::TEST_LINKER_STANDARD_JSON_INPUT_WITH_LIBRARIES_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "\"linked\":{\"tests/data/bytecodes/linker.zbin\":",
    ));

    Ok(())
}

#[test]
fn without_libraries_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        common::TEST_LINKER_STANDARD_JSON_INPUT_WITHOUT_LIBRARIES_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        r#""unlinked":{"tests/data/bytecodes/linker.zbin":{"linker_symbols":["Greeter.sol:GreeterHelper"],"factory_dependencies":[]}}"#,
    ));

    Ok(())
}

#[test]
fn with_standard_json_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--link",
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Error: JSON file"));

    Ok(())
}

#[test]
fn with_target_evm() -> anyhow::Result<()> {
    common::setup()?;

    let target = Target::EVM.to_string();
    let args = &[
        "--link",
        common::TEST_LINKER_BYTECODE_PATH,
        "--target",
        target.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "The EVM target does not support linking yet.",
    ));

    Ok(())
}
