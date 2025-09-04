//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn invalid_input_yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json", crate::common::TEST_YUL_CONTRACT_PATH];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn invalid_input_solc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "ParserError: Expected identifier but got",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn invalid_input_zksolc_error() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "You are using \'<address payable>.send/transfer(<X>)\' without providing the gas amount.",
    ));

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.success();

    Ok(())
}

#[test]
fn suppressed_messages() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_JSON_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_JSON_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn recursion() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_RECURSION_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn invalid_path() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON file \\\"tests/data/standard_json_input/non_existent.json\\\" reading",
        ))
        .code(era_compiler_common::EXIT_CODE_SUCCESS);

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(era_compiler_common::EXIT_CODE_FAILURE);

    Ok(())
}

#[test]
fn invalid_utf8() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: expected value",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn stdin_missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json"];
    let solc_args = &["--standard-json"];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: EOF while parsing",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn empty_sources() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("No input sources specified."))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn missing_sources() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: missing field `sources`",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn yul() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn both_urls_and_content() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Both `content` and `urls` cannot be set",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn neither_urls_nor_content() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Either `content` or `urls` must be set.",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn yul_solc() -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.code(status);

    Ok(())
}

#[test]
fn metadata_hash_ipfs_and_metadata() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_IPFS_AND_METADATA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264"))
        .stdout(predicate::str::contains("\"metadata\""));

    Ok(())
}

#[test]
fn metadata_hash_ipfs_no_metadata() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_IPFS_NO_METADATA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a264"))
        .stdout(predicate::str::contains("\"metadata\"").not());

    Ok(())
}

#[test]
fn metadata_hash_none_and_metadata() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_NONE_AND_METADATA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a164"))
        .stdout(predicate::str::contains("\"metadata\""));

    Ok(())
}

#[test]
fn metadata_hash_none_no_metadata() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_NONE_NO_METADATA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("a164"))
        .stdout(predicate::str::contains("\"metadata\"").not());

    Ok(())
}
