use crate::{cli, common};
use predicates::prelude::*;

const JSON_ARGS: &[&str] = &[
    "abi",
    "hashes",
    "metadata",
    "devdoc",
    "userdoc",
    "storage-layout",
    "ast",
    "asm",
    "bin",
    "bin-runtime",
];

#[test]
fn with_combined_json_loop_args() -> anyhow::Result<()> {
    common::setup()?;

    for arg in JSON_ARGS {
        let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--combined-json", arg];

        let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_combined_json_two_files() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        cli::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        "--combined-json",
        "bin",
    ];

    let result = cli::execute_zksolc(args)?;
    let status_code = result
        .success()
        .stdout(predicate::str::contains("contracts"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test]
fn with_combined_json_no_args() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--combined-json"];

    let result = cli::execute_zksolc(args)?;
    let status_code = result
        .failure()
        .stderr(predicate::str::contains(
            "requires a value but none was supplied",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status_code);

    Ok(())
}

#[test]
fn with_combined_json_and_invalid_arg() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--combined-json",
        "unknown",
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_multiple_combined_json_flags() -> anyhow::Result<()> {
    common::setup()?;

    for &arg in JSON_ARGS {
        let args = &[
            cli::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            arg,
            "--combined-json",
            arg,
        ];

        let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_combined_json_and_yul_input() -> anyhow::Result<()> {
    common::setup()?;

    for &arg in JSON_ARGS {
        let args = &[cli::TEST_YUL_CONTRACT_PATH, "--combined-json", arg];

        let result = cli::execute_zksolc(args)?;
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
