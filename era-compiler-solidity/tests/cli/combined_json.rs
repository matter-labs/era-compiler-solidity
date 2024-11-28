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
fn with_combined_json_loop_args(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    for arg in JSON_ARGS {
        let args = &[common::TEST_SOLIDITY_CONTRACT_PATH, "--combined-json", arg];

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
fn with_combined_json_two_files(target: Target) -> anyhow::Result<()> {
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
            predicate::str::is_match(r#""bin":"[0-9a-z]*""#)
                .expect("Always valid")
                .count(2),
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
fn with_combined_json_no_argument(target: Target) -> anyhow::Result<()> {
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
fn with_combined_json_and_invalid_arg(target: Target) -> anyhow::Result<()> {
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_multiple_combined_json_flags(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    for &arg in JSON_ARGS {
        let args = &[
            common::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            arg,
            "--combined-json",
            arg,
        ];

        let result = cli::execute_zksolc_with_target(args, target)?;
        let status_code = result
            .failure()
            .stderr(predicate::str::contains("cannot be used multiple times"))
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
fn with_combined_json_and_yul_input(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    for &arg in JSON_ARGS {
        let args = &[common::TEST_YUL_CONTRACT_PATH, "--combined-json", arg];

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
