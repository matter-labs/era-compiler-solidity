use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_input() -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn without_input() -> anyhow::Result<()> {
    common::setup()?;
    let args = &["--libraries"];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "requires a value but none was supplied",
    ));

    Ok(())
}

#[test]
fn with_libraries_llvm_ir_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--llvm-ir",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test]
fn with_libraries_eravm_assembly_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test]
fn with_libraries_standard_json_mode() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--libraries",
        cli::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc(args)?;

    result.success().stdout(predicate::str::contains(
        "Libraries must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test]
fn with_libraries_missing_contract_name() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_CONTRACT_NAME_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;

    result.failure().stderr(predicate::str::contains(
        "The library `tests/data/contracts/solidity/MiniMath.sol` contract name is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_missing_address() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_ADDRESS_MISSING,
    ];

    let result = cli::execute_zksolc(args)?;

    result.failure().stderr(predicate::str::contains(
        "The library `tests/data/contracts/solidity/MiniMath.sol:MiniMath` address is missing.",
    ));

    Ok(())
}

#[test]
fn with_libraries_invalid_address() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        cli::LIBRARY_ADDRESS_INVALID,
    ];

    let result = cli::execute_zksolc(args)?;

    result.failure().stderr(predicate::str::contains(
        "Error: Library address is not prefixed with \"0x\"",
    ));

    Ok(())
}
