use crate::{cli, common};
use era_compiler_common::Target;
use era_solc::StandardJsonInputCodegen;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
// TODO: #[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn with_codegen(target: Target, codegen: StandardJsonInputCodegen) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--bin",
        common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
#[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn with_codegen_yul_mode(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--yul",
        "--bin",
        common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
#[test_case(Target::EVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EVM, StandardJsonInputCodegen::Yul)]
fn with_codegen_llvm_ir_mode(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--llvm-ir",
        "--bin",
        common::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
fn with_codegen_eravm_assembly_mode(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--eravm-assembly",
        "--bin",
        common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(Target::EraVM, StandardJsonInputCodegen::EVMLA)]
#[test_case(Target::EraVM, StandardJsonInputCodegen::Yul)]
fn with_codegen_standard_json_mode(
    target: Target,
    codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<()> {
    common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--standard-json",
        common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--codegen",
        codegen.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Codegen must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_codegen_invalid(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--codegen",
        "invalid",
        "--bin",
        common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: invalid value 'invalid' for '--codegen <CODEGEN>': Invalid codegen: `invalid`. Available options: evmla, yul"));

    Ok(())
}
