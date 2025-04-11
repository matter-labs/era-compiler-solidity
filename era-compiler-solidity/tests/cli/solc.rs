//!
//! CLI tests for the eponymous option.
//!

use assert_cmd::Command;
use era_compiler_common::Target;
use predicates::prelude::predicate;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn default(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn no_solc(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;

    let result = zksolc
        .arg(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
        .arg("--target")
        .arg(target.to_string())
        .env("PATH", "./solc-bin")
        .assert();
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json_no_solc(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn llvm_ir(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn eravm_assembly(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_path(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let path = "solc-not-found";
    let args = &[crate::common::TEST_SOLIDITY_CONTRACT_PATH, "--solc", path];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        format!("The `{path}` executable not found:").as_str(),
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_missing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_OUTPUT_ERROR_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("version getting:"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_too_old(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_TOO_OLD_PATH,
    ];

    let version_supported = era_solc::Compiler::FIRST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch -= 1;

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions older than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_too_recent(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_TOO_NEW_PATH,
    ];

    let version_supported = era_solc::Compiler::LAST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch += 1;

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions newer than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_not_enough_lines(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_LINES_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough lines.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_not_enough_words_in_2nd_line(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_WORDS_IN_2ND_LINE_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough words in the 2nd line.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn error_version_parsing(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_PARSING_ERROR_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("version parsing:"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn zksync_revision_missing_version(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_MISSING_VERSION,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "ZKsync revision parsing: missing version.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn zksync_revision_missing_revision(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_MISSING_REVISION,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "ZKsync revision parsing: missing revision.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn zksync_revision_parsing_revision_error(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_VERSION_ZKSYNC_REVISION_PARSING_REVISION_ERROR,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("ZKsync revision parsing:"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn exit_code_failed(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_EXIT_CODE_FAILED,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess failed with exit code"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn invalid_output_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        crate::common::TEST_SCRIPT_SOLC_INVALID_OUTPUT_JSON,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("subprocess stdout parsing"));

    Ok(())
}
