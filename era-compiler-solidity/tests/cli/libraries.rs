use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_input(target: Target) -> anyhow::Result<()> {
    common::setup()?;
    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn without_input(target: Target) -> anyhow::Result<()> {
    common::setup()?;
    let args = &["--libraries"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "requires a value but none was supplied",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_llvm_ir_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--llvm-ir",
        common::TEST_LLVM_IR_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--eravm-assembly",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Libraries are only supported in Solidity, Yul, and linker modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--libraries",
        common::LIBRARY_DEFAULT,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.success().stdout(predicate::str::contains(
        "Libraries must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_missing_contract_name(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--yul",
        common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_CONTRACT_NAME_MISSING,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.failure().stderr(predicate::str::contains(
        "Library `tests/data/contracts/solidity/MiniMath.sol` contract name is missing.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_missing_address(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--yul",
        common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_ADDRESS_MISSING,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.failure().stderr(predicate::str::contains(
        "Error: Library `tests/data/contracts/solidity/MiniMath.sol:MiniMath` address is missing.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_libraries_invalid_address(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--yul",
        common::TEST_YUL_CONTRACT_PATH,
        "--libraries",
        common::LIBRARY_ADDRESS_INVALID,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;

    result.failure().stderr(predicate::str::contains(
        "Error: Invalid address `INVALID` of library `tests/data/contracts/solidity/MiniMath.sol:MiniMath`: Odd number of digits",
    ));

    Ok(())
}
