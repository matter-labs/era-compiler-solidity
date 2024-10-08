//!
//! The unit tests for standard JSON with different languages.
//!

use crate::common;
use era_compiler_solidity::solc::standard_json::input::Input as SolcStandardJsonInput;
use era_compiler_solidity::solc::Compiler as SolcCompiler;
use std::path::PathBuf;

#[test]
fn standard_json_yul_default() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_default.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_default_validated() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_default.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler = common::get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_default_urls() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_default_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_default_urls_validated() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_default_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler = common::get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_eravm() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_eravm.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_eravm_validated() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_eravm.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler = common::get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_eravm_urls() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_eravm_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_yul_eravm_urls_validated() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/yul_eravm_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_compiler = common::get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .get("EventWriter.yul")
        .expect("The `EventWriter.yul` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_llvm_ir_default_urls() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/llvm_ir_default_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_llvm_ir_standard_json(solc_input).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}

#[test]
fn standard_json_eravm_assembly_default_urls() {
    let solc_input = SolcStandardJsonInput::try_from(Some(
        PathBuf::from("tests/examples/standard_json_input/eravm_assembly_default_urls.json")
            .as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_eravm_assembly_standard_json(solc_input).expect("Test failure");

    assert!(!solc_output
        .contracts
        .expect("The `contracts` field is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .get("Test")
        .expect("The `Test` contract is missing")
        .evm
        .as_ref()
        .expect("The `evm` field is missing")
        .bytecode
        .as_ref()
        .expect("The `bytecode` field is missing")
        .object
        .is_empty())
}
