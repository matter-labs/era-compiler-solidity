//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn llvm_ir(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn eravm_assembly(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--eravm-assembly",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--libraries",
        crate::common::LIBRARY_DEFAULT,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Libraries must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing_contract_name(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_CONTRACT_NAME_MISSING,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Library `tests/data/contracts/solidity/MiniMath.sol` contract name is missing.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing_address(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_ADDRESS_MISSING,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Library `tests/data/contracts/solidity/MiniMath.sol:MiniMath` address is missing.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_address(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        crate::common::LIBRARY_ADDRESS_INVALID,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Invalid address `INVALID` of library `tests/data/contracts/solidity/MiniMath.sol:MiniMath`: Odd number of digits",
    ));

    Ok(())
}
