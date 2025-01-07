//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use era_solc::StandardJsonInputCodegen;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
#[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn default(target: Target, codegen: StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
#[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn yul(target: Target, codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
#[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn llvm_ir(target: Target, codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
fn eravm_assembly(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
fn standard_json(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--codegen",
        codegen.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Codegen must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--codegen",
        "invalid",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: invalid value 'invalid' for '--codegen <CODEGEN>': Invalid codegen: `invalid`. Available options: evmla, yul"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_FORCE_EVMLA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "The `codegen` setting will become mandatory in future versions of zksolc. Please set it to either `evmla` or `yul`.",
    ));

    Ok(())
}
