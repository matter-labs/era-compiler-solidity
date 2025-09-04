//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test]
fn llvm_ir() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test]
fn eravm_assembly() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--eravm-assembly",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Libraries must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test]
fn missing_contract_name() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_CONTRACT_NAME_MISSING,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Library `tests/data/contracts/solidity/MiniMath.sol` contract name is missing.",
    ));

    Ok(())
}

#[test]
fn missing_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_ADDRESS_MISSING,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `tests/data/contracts/solidity/MiniMath.sol:MiniMath` address is missing.",
    ));

    Ok(())
}

#[test]
fn invalid_address() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_ADDRESS_INVALID,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Invalid address `INVALID` of library `tests/data/contracts/solidity/MiniMath.sol:MiniMath`: Odd number of digits",
    ));

    Ok(())
}
