use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_incompatible_input(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_YUL_CONTRACT_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("parsing: expected value"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_invalid_by_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "ParserError: Expected identifier but got",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_invalid_by_zksolc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INVALID_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "You are using \'<address payable>.send/transfer(<X>)\' without providing the gas amount.",
    ));

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.success();

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_with_suppressed_messages(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_recursion(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_RECURSION_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_internal_function_pointers(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INTERNAL_FUNCTION_POINTERS_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_non_existent(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON file \\\"tests/data/standard_json_input/non_existent.json\\\" opening",
        ))
        .code(era_compiler_common::EXIT_CODE_SUCCESS);

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(era_compiler_common::EXIT_CODE_FAILURE);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_invalid_utf8(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: expected value",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_stdin_missing(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json"];
    let args_solc = &["--standard-json"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: EOF while parsing",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_empty_sources(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("No input sources specified."))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_missing_sources(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Standard JSON parsing: missing field `sources`",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_yul(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];
    let args_solc = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_both_urls_and_content(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Both `content` and `urls` cannot be set",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_neither_urls_nor_content(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains(
            "Either `content` or `urls` must be set.",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_standard_json_yul_with_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];
    let args_solc = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let status = result
        .success()
        .stdout(predicate::str::contains("bytecode"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.code(status);

    Ok(())
}
