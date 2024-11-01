use crate::{cli, common};
use predicates::prelude::*;
use test_case::test_case;

#[test_case(era_solc::StandardJsonInputCodegen::EVMLA)]
#[test_case(era_solc::StandardJsonInputCodegen::Yul)]
fn with_codegen(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(era_solc::StandardJsonInputCodegen::EVMLA)]
#[test_case(era_solc::StandardJsonInputCodegen::Yul)]
fn with_codegen_yul_mode(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--yul",
        "--bin",
        cli::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(era_solc::StandardJsonInputCodegen::EVMLA)]
#[test_case(era_solc::StandardJsonInputCodegen::Yul)]
fn with_codegen_llvm_ir_mode(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(era_solc::StandardJsonInputCodegen::EVMLA)]
#[test_case(era_solc::StandardJsonInputCodegen::Yul)]
fn with_codegen_eravm_assembly_mode(
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(era_solc::StandardJsonInputCodegen::EVMLA)]
#[test_case(era_solc::StandardJsonInputCodegen::Yul)]
fn with_codegen_standard_json_mode(
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--codegen",
        codegen.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Codegen must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test]
fn with_codegen_invalid() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--codegen",
        "invalid",
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("Invalid value for \'--codegen <codegen>\': Invalid codegen: `invalid`. Available options: evmla, yul"));

    Ok(())
}
