//!
//! Unit tests for combined JSON.
//!

#[test]
fn one_file() {
    let paths = [crate::common::TEST_SOLIDITY_CONTRACT_PATH];
    let names = ["Test"];
    let sources = crate::common::read_sources(paths.as_slice());

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
            .expect("`solc` initialization error");

    let combined_json = crate::common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        vec![era_solc::CombinedJsonSelector::Bytecode],
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
fn multiple_files() {
    let paths = [
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ];
    let names = ["Test", "Callable"];
    let sources = crate::common::read_sources(paths.as_slice());

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
            .expect("`solc` initialization error");

    let combined_json = crate::common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        vec![era_solc::CombinedJsonSelector::Bytecode],
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
fn multiple_files_with_dependencies() {
    let paths = [
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_GREETER_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH,
    ];
    let full_paths = [
        format!("{}:Test", crate::common::TEST_SOLIDITY_CONTRACT_PATH),
        format!(
            "{}:Greeter",
            crate::common::TEST_SOLIDITY_CONTRACT_GREETER_PATH
        ),
        format!(
            "{}:GreeterHelper",
            crate::common::TEST_SOLIDITY_CONTRACT_GREETER_PATH
        ),
        format!(
            "{}:SimpleContract",
            crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
        ),
        format!(
            "{}:SimpleLibrary",
            crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
        ),
    ];
    let sources = crate::common::read_sources(paths.as_slice());

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
            .expect("`solc` initialization error");

    let combined_json = crate::common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        vec![era_solc::CombinedJsonSelector::Bytecode],
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

#[test]
fn eravm_assembly_requested() {
    let paths = [crate::common::TEST_SOLIDITY_CONTRACT_PATH];
    let names = ["Test"];
    let sources = crate::common::read_sources(paths.as_slice());

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
            .expect("`solc` initialization error");

    let combined_json = crate::common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        vec![era_solc::CombinedJsonSelector::Assembly],
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
            .assembly
            .as_ref()
            .expect("The `assembly` field is missing")
            .is_empty());
    }
}

#[test]
fn eravm_assembly_not_requested() {
    let paths = [crate::common::TEST_SOLIDITY_CONTRACT_PATH];
    let names = ["Test"];
    let sources = crate::common::read_sources(paths.as_slice());

    let solc_compiler =
        crate::common::get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)
            .expect("`solc` initialization error");

    let combined_json = crate::common::build_solidity_combined_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        vec![],
        era_compiler_common::HashType::Ipfs,
        &solc_compiler.version.default,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(combined_json.contracts.len(), paths.len());
    for (path, name) in paths.into_iter().zip(names.into_iter()) {
        let full_path = format!("{path}:{name}");
        assert!(combined_json
            .contracts
            .get(full_path.as_str())
            .as_ref()
            .unwrap_or_else(|| panic!("The contract `{full_path}` is missing"))
            .assembly
            .is_none());
    }
}
