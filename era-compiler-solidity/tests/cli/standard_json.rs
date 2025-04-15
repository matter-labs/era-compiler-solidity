//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
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

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input_yul(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json", crate::common::TEST_YUL_CONTRACT_PATH];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_input_solc_error(target: Target) -> anyhow::Result<()> {
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

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
fn invalid_input_zksolc_error(target: Target) -> anyhow::Result<()> {
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

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "You are using \'<address payable>.send/transfer(<X>)\' without providing the gas amount.",
    ));

    let solc_result = crate::cli::execute_solc(solc_args)?;
    solc_result.success();

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn suppressed_messages(target: Target) -> anyhow::Result<()> {
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

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn recursion(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_RECURSION_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_path(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_utf8(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn stdin_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json"];
    let solc_args = &["--standard-json"];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn empty_sources(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn missing_sources(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn yul(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn both_urls_and_content(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn neither_urls_nor_content(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];
    let solc_args = &[
        "--standard-json",
        crate::common::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn yul_solc(target: Target) -> anyhow::Result<()> {
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

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
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

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata_hash_ipfs_and_metadata(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_IPFS_AND_METADATA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("a264"))
        .stdout(predicate::str::contains("\"metadata\""));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata_hash_ipfs_no_metadata(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_IPFS_NO_METADATA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("a264"))
        .stdout(predicate::str::contains("\"metadata\"").not());

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata_hash_none_and_metadata(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_NONE_AND_METADATA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("a164"))
        .stdout(predicate::str::contains("\"metadata\""));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn metadata_hash_none_no_metadata(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_METADATA_HASH_NONE_NO_METADATA,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("a164"))
        .stdout(predicate::str::contains("\"metadata\"").not());

    Ok(())
}
