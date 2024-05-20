//!
//! The unit tests for standard JSON with different languages.
//!

#![cfg(test)]

use std::path::PathBuf;

use crate::solc::standard_json::input::Input as SolcStandardJsonInput;

#[test]
fn standard_json_yul_default() {
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/yul_default.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_yul_standard_json(solc_input, None).expect("Test failure");

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
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/yul_default_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_yul_standard_json(solc_input, None).expect("Test failure");

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
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/yul_eravm.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_yul_standard_json(solc_input, None).expect("Test failure");

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
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/yul_eravm_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_yul_standard_json(solc_input, None).expect("Test failure");

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
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/llvm_ir_default_urls.json").as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_llvm_ir_standard_json(solc_input).expect("Test failure");

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
    let solc_input = SolcStandardJsonInput::try_from_reader(Some(
        PathBuf::from("src/tests/examples/standard_json_input/eravm_assembly_default_urls.json")
            .as_path(),
    ))
    .expect("Standard JSON reading error");
    let solc_output = super::build_eravm_assembly_standard_json(solc_input).expect("Test failure");

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
