//!
//! The unit tests for standard JSON with different languages.
//!

use std::path::PathBuf;

use crate::common;

#[test]
fn standard_json_yul_solc() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_solc.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_solc_validated() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_solc.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)
            .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_solc_urls() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_solc_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_solc_urls_validated() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_solc_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)
            .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_zksolc() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_zksolc.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_zksolc_validated() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_zksolc.json").as_path(),
    ))
    .expect("Standard JSON reading error");

    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)
            .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_zksolc_urls() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_zksolc_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_yul_standard_json(solc_input, None).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_yul_zksolc_urls_validated() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/yul_zksolc_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_compiler =
        common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION, false)
            .expect("`solc` initialization error");
    let solc_output =
        common::build_yul_standard_json(solc_input, Some(&solc_compiler)).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_llvm_ir_urls() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/llvm_ir_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_llvm_ir_standard_json(solc_input).expect("Test failure");

    assert!(!solc_output
        .contracts
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
fn standard_json_eravm_assembly_urls() {
    let solc_input = era_solc::StandardJsonInput::try_from(Some(
        PathBuf::from("tests/data/standard_json_input/eravm_assembly_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = common::build_eravm_assembly_standard_json(solc_input).expect("Test failure");

    assert!(!solc_output
        .contracts
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
