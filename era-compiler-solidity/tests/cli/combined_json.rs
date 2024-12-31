use crate::{cli, common};
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
    common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[
            common::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            selector,
        ];

        let result = cli::execute_zksolc_with_target(args, target)?;
        let status_code = result
            .success()
            .stdout(predicate::str::contains("contracts"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn all_yul(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    for selector in JSON_ARGS.into_iter() {
        let args = &[common::TEST_YUL_CONTRACT_PATH, "--combined-json", selector];

        let result = cli::execute_zksolc_with_target(args, target)?;
        let status_code = result
            .failure()
            .stderr(predicate::str::contains("Expected identifier"))
            .get_output()
            .status
            .code()
            .expect("No exit code.");

        let solc_result = cli::execute_solc(args)?;
        solc_result.code(status_code);
    }

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn two_files(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        common::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        "--combined-json",
        "bin",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .success()
        .stdout(
            predicate::str::is_match([r#""bin":"[0-9a-f]*""#; 2].join(".*")).expect("Always valid"),
        )
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn no_arguments(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--combined-json"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "error: a value is required for \'--combined-json <COMBINED_JSON>\' but none was supplied",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_path(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        common::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "unknown",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains("Invalid option").or(predicate::str::contains("error")))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}
