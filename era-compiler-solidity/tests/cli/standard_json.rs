use crate::{cli, common};
use predicates::prelude::*;

#[test]
fn with_standard_json() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

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

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_incompatible_input() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_YUL_CONTRACT_PATH];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_invalid_by_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

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

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_invalid_by_zksolc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_BY_ZKSOLC_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_BY_ZKSOLC_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "You are using \'<address payable>.send/transfer(<X>)\' without providing the gas amount.",
    ));

    let solc_result = cli::execute_solc(args_solc)?;
    solc_result.success();

    Ok(())
}

#[test]
fn with_standard_json_with_suppressed_messages() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

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

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_recursion() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_RECURSION_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn with_standard_json_non_existent() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_invalid_utf8() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_stdin_missing() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json"];
    let args_solc = &["--standard-json"];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_empty_sources() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_missing_sources() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_yul() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];
    let args_solc = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_both_urls_and_content() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_neither_urls_nor_content() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];
    let args_solc = &[
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_yul_with_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_YUL_STANDARD_JSON_SOLC_PATH,
    ];
    let args_solc = &["--standard-json", cli::TEST_YUL_STANDARD_JSON_SOLC_PATH];

    let result = cli::execute_zksolc(args)?;
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

#[test]
fn with_standard_json_llvm_ir() -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--standard-json", cli::TEST_LLVM_IR_STANDARD_JSON_PATH];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn with_standard_json_llvm_ir_with_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--standard-json",
        cli::TEST_LLVM_IR_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "LLVM IR projects cannot be compiled with `solc`",
    ));

    Ok(())
}

#[test]
fn with_standard_json_eravm_assembly() -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
    ];

    let result = cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test]
fn with_standard_json_eravm_assembly_with_solc() -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_compiler_solidity::SolcCompiler::LAST_SUPPORTED_VERSION)?
            .executable;

    let args = &[
        "--standard-json",
        cli::TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "EraVM assembly projects cannot be compiled with `solc`",
    ));

    Ok(())
}
