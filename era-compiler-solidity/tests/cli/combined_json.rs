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
fn run_zksolc_with_just_combined_json() -> anyhow::Result<()> {
    let _ = common::setup();
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
fn run_zksolc_with_sol_contract_and_combined_json() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--combined-json"];

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
fn run_zksolc_with_combined_json_and_valid_args() -> anyhow::Result<()> {
    let _ = common::setup();
    for &arg in JSON_ARGS {
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
fn run_zksolc_with_combined_json_and_invalid_args() -> anyhow::Result<()> {
    let _ = common::setup();
    for &arg in JSON_ARGS {
        let args = &[
            cli::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            &format!("--{}", arg),
        ];

        let result = cli::execute_zksolc(args)?;
        let status_code = result
            .failure()
            .stderr(
                predicate::str::contains("Invalid option").or(predicate::str::contains("error")),
            )
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
fn run_zksolc_with_combined_json_and_duplicate_args() -> anyhow::Result<()> {
    let _ = common::setup();
    for &arg in JSON_ARGS {
        let args = &[
            cli::TEST_SOLIDITY_CONTRACT_PATH,
            "--combined-json",
            arg,
            arg,
        ];

        let result = cli::execute_zksolc(args)?;
        let status_code = result
            .failure()
            .stderr(
                predicate::str::contains("No such file or directory")
                    .or(predicate::str::contains("cannot find the file specified")),
            )
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
fn run_zksolc_with_multiple_combined_json_flags() -> anyhow::Result<()> {
    let _ = common::setup();
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
fn run_zksolc_with_yul_and_combined_json() -> anyhow::Result<()> {
    let _ = common::setup();
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
