//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

const JSON_ARGS: &[&str] = &[
    "abi",
    "hashes",
    "metadata",
    "devdoc",
    "userdoc",
    "storage-layout",
    "transient-storage-layout",
    "ast",
    "asm",
    "bin",
    "bin-runtime",
];

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn all(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[
            crate::common::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            selector,
        ];

        let result = crate::cli::execute_zksolc_with_target(args, target)?;
        let status_code = result
            .success()
            .stdout(predicate::str::contains("contracts"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = crate::cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn all_yul(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[
            crate::common::TEST_YUL_CONTRACT_PATH,
            "--combined-json",
            selector,
        ];

        let result = crate::cli::execute_zksolc_with_target(args, target)?;
        let status_code = result
            .failure()
            .stderr(predicate::str::contains("Expected identifier"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = crate::cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn two_files(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        "--combined-json",
        "bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .success()
        .stdout(
            predicate::str::is_match([r#""bin":"[0-9a-f]*""#; 2].join(".*")).expect("Always valid"),
        )
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_path(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "unknown",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        "The selector `unknown` is not supported, and therefore ignored.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let selector = era_solc::CombinedJsonSelector::Bytecode.to_string();
    let args = &[
        "--solc",
        solc_compiler.as_str(),
        crate::common::TEST_BROKEN_INPUT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess failed with exit code"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_output(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = era_solc::CombinedJsonSelector::Bytecode.to_string();
    let args = &[
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_INVALID_OUTPUT_JSON,
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess stdout parsing:"));

    Ok(())
}

#[test_case(Target::EVM)]
fn warning_evm_assembly_unsupported_yet(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = era_solc::CombinedJsonSelector::Assembly.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(format!(
        "The `{}` selector is not supported for the {} target yet, and therefore ignored.",
        era_solc::CombinedJsonSelector::Assembly,
        era_compiler_common::Target::EVM,
    )));

    Ok(())
}

#[test_case(Target::EraVM)]
fn warning_bin_omitted(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = era_solc::CombinedJsonSelector::ASM.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        format!("The `{}` selector will become mandatory in future releases of `zksolc`. For now, bytecode is always emitted even if the selector is not provided.", era_solc::CombinedJsonSelector::Bytecode),
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
fn warning_bin_runtime_excess(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = era_solc::CombinedJsonSelector::BytecodeRuntime.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        format!("The `{}` selector does not make sense for the {} target, since there is only one bytecode segment. The eponymous output field will be removed in future releases of `zksolc`.", era_solc::CombinedJsonSelector::BytecodeRuntime, era_compiler_common::Target::EraVM),
    ));

    Ok(())
}
