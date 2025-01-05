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
fn no_arguments(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--combined-json"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "error: a value is required for \'--combined-json <COMBINED_JSON>\' but none was supplied",
        ))
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
fn warning_bin_omitted(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let selector = era_solc::CombinedJsonSelector::Assembly.to_string();
    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        selector.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stderr(predicate::str::contains(
        format!("The `{}` selector will become mandatory in future versions of `zksolc`. For now, bytecode is always emitted even if the selector is not provided.", era_solc::CombinedJsonSelector::Bytecode),
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
        format!("The `{}` selector does not make sense for the {} target, since there is only one bytecode segment.", era_solc::CombinedJsonSelector::BytecodeRuntime, era_compiler_common::Target::EraVM),
    ));

    Ok(())
}
