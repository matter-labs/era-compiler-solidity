//!
//! The unit tests for combined JSON.
//!

use crate::common;

#[test]
fn combined_json_one_file() {
    let paths = [common::TEST_SOLIDITY_CONTRACT_PATH];
    let names = ["Test"];
    let sources = common::read_sources(paths.as_slice());

    let solc_compiler = common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");

    let combined_json = common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        &solc_compiler.version.default,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(combined_json.contracts.len(), paths.len());
    for (path, name) in paths.into_iter().zip(names.into_iter()) {
        let full_path = format!("{path}:{name}");
        assert!(!combined_json
            .contracts
            .get(full_path.as_str())
            .as_ref()
            .unwrap_or_else(|| panic!("The contract `{full_path}` is missing"))
            .bin
            .as_ref()
            .expect("The `bin` field is missing")
            .is_empty());
    }
}

#[test]
fn combined_json_multiple_files() {
    let paths = [
        common::TEST_SOLIDITY_CONTRACT_PATH,
        common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ];
    let names = ["Test", "Callable"];
    let sources = common::read_sources(paths.as_slice());

    let solc_compiler = common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");

    let combined_json = common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        &solc_compiler.version.default,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(combined_json.contracts.len(), paths.len());
    for (path, name) in paths.into_iter().zip(names.into_iter()) {
        let full_path = format!("{path}:{name}");
        assert!(!combined_json
            .contracts
            .get(full_path.as_str())
            .as_ref()
            .unwrap_or_else(|| panic!("The contract `{full_path}` is missing"))
            .bin
            .as_ref()
            .expect("The `bin` field is missing")
            .is_empty());
    }
}

#[test]
fn combined_json_multiple_files_with_dependencies() {
    let paths = [
        common::TEST_SOLIDITY_CONTRACT_PATH,
        common::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
    ];
    let full_paths = [
        format!("{}:Test", common::TEST_SOLIDITY_CONTRACT_PATH),
        format!("{}:Greeter", common::TEST_SOLIDITY_CONTRACT_GREETER_PATH),
        format!(
            "{}:GreeterHelper",
            common::TEST_SOLIDITY_CONTRACT_GREETER_PATH
        ),
        format!(
            "{}:SimpleContract",
            common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
        ),
        format!(
            "{}:SimpleLibrary",
            common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
        ),
    ];
    let sources = common::read_sources(paths.as_slice());

    let solc_compiler = common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
        .expect("`solc` initialization error");

    let combined_json = common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        &solc_compiler.version.default,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(combined_json.contracts.len(), full_paths.len());
    for full_path in full_paths.into_iter() {
        assert!(!combined_json
            .contracts
            .get(full_path.as_str())
            .as_ref()
            .unwrap_or_else(|| panic!("The contract `{full_path}` is missing"))
            .bin
            .as_ref()
            .expect("The `bin` field is missing")
            .is_empty());
    }
}
