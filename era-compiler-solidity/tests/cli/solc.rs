use crate::{cli, common};
use assert_cmd::Command;
use era_compiler_common::Target;
use predicates::prelude::predicate;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        solc_compiler.as_str(),
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn without_solc(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let mut zksolc = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;

    let result = zksolc
        .arg(cli::TEST_SOLIDITY_CONTRACT_PATH)
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
fn with_solc_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn without_solc_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("bytecode"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_llvm_ir_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--llvm-ir",
        "--bin",
        cli::TEST_LLVM_IR_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_eravm_assembly_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)?.executable;

    let args = &[
        "--solc",
        solc_compiler.as_str(),
        "--eravm-assembly",
        "--bin",
        cli::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "Using `solc` is only allowed in Solidity and Yul modes.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_not_found(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let path = "solc-not-found";
    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--solc", path];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        format!("The `{path}` executable not found:").as_str(),
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_output_error(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_OUTPUT_ERROR_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("version getting:"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_too_old(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_TOO_OLD_PATH,
    ];

    let version_supported = era_solc::Compiler::FIRST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch -= 1;

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions older than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_too_new(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_TOO_NEW_PATH,
    ];

    let version_supported = era_solc::Compiler::LAST_SUPPORTED_VERSION;
    let mut version_not_supported = version_supported.clone();
    version_not_supported.patch += 1;

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains(format!("versions newer than {version_supported} are not supported, found {version_not_supported}.").as_str()));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_not_enough_lines(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_LINES_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough lines.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_not_enough_words_in_2nd_line(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_WORDS_IN_2ND_LINE_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.failure().stderr(predicate::str::contains(
        "version parsing: not enough words in the 2nd line.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_solc_version_parsing_error(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        cli::TEST_SOLIDITY_CONTRACT_PATH,
        "--solc",
        cli::TEST_SCRIPT_SOLC_VERSION_PARSING_ERROR_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .failure()
        .stderr(predicate::str::contains("version parsing:"));

    Ok(())
}
