//!
//! CLI tests for the eponymous option.
//!

use era_solc::StandardJsonInputCodegen;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn default(codegen: StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn yul(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn llvm_ir(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn eravm_assembly(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn standard_json(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--codegen",
        codegen.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Codegen must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test]
fn invalid() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--codegen",
        "invalid",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: invalid value 'invalid' for '--codegen <CODEGEN>': Invalid codegen: `invalid`. Available options: evmla, yul"));

    Ok(())
}

#[test]
fn missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_FORCE_EVMLA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The default codegen of `zksolc` does not match that of `solc` for historical reasons.",
    ));

    Ok(())
}
