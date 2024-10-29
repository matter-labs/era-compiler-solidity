//!
//! The Solidity compiler unit tests for the linker.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_solidity::solc::standard_json::input::settings::codegen::Codegen as SolcStandardJsonInputSettingsCodegen;
use era_compiler_solidity::solc::standard_json::input::settings::libraries::Libraries;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
fn library_not_passed_compile_time_08_evmla() {
    library_not_passed_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_not_passed_compile_time_08_yul() {
    library_not_passed_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_not_passed_post_compile_time_08_evmla() {
    library_not_passed_post_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_not_passed_post_compile_time_08_yul() {
    library_not_passed_post_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_passed_compile_time_08_evmla() {
    library_passed_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_passed_compile_time_08_yul() {
    library_passed_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_passed_post_compile_time_08_evmla() {
    library_passed_post_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_passed_post_compile_time_08_yul() {
    library_passed_post_compile_time(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_passed_post_compile_time_second_call_08_evmla() {
    library_passed_post_compile_time_second_call(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_passed_post_compile_time_second_call_08_yul() {
    library_passed_post_compile_time_second_call(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_passed_post_compile_time_redundant_args_08_evmla() {
    library_passed_post_compile_time_redundant_args(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_passed_post_compile_time_redundant_args_08_yul() {
    library_passed_post_compile_time_redundant_args(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf_08_evmla() {
    library_passed_post_compile_time_non_elf(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf_08_yul() {
    library_passed_post_compile_time_non_elf(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}
#[test]
fn library_produce_equal_bytecode_in_both_cases_08_evmla() {
    library_produce_equal_bytecode_in_both_cases(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn library_produce_equal_bytecode_in_both_cases_08_yul() {
    library_produce_equal_bytecode_in_both_cases(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}

pub const SOURCE_CODE: &str = r#"
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

library GreaterHelper {
    function addPrefix(Greeter greeter, string memory great) public view returns (string memory) {
        return string.concat(greeter.prefix(),great);
    }
}

contract Greeter {
    string private greeting;
    string private _prefix;

    constructor(string memory _greeting) {
        greeting = _greeting;
        _prefix = "The greating is:";
    }

    function prefix() public view returns (string memory) {
        return _prefix;
    }

    function greet() public view returns (string memory) {
        return GreaterHelper.addPrefix(this, greeting);
    }

    function setGreeting(string memory _greeting) public {
        greeting = _greeting;
    }
}
"#;

fn get_bytecode(
    libraries: Libraries,
    version: &semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) -> Vec<u8> {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let build = common::build_solidity(
        sources,
        libraries,
        era_compiler_common::HashType::None,
        BTreeSet::new(),
        version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let bytecode_hexadecimal = build
        .contracts
        .get("test.sol")
        .expect("Missing file `test.sol`")
        .get("Greeter")
        .expect("Missing contract `test.sol:Greeter`")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .as_str();
    hex::decode(bytecode_hexadecimal).expect("Invalid bytecode")
}

fn library_not_passed_compile_time(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(
        memory_buffer.is_elf_eravm(),
        "The bytecode is not an ELF file"
    );
}

fn library_not_passed_post_compile_time(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&BTreeMap::new())
        .expect("Link failure");
    assert!(
        memory_buffer_linked.is_elf_eravm(),
        "The bytecode is not an ELF file"
    );
}

fn library_passed_compile_time(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments =
        vec!["test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = Libraries::try_from(library_arguments.as_slice()).expect("Always valid");

    let bytecode = get_bytecode(libraries, &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(!memory_buffer.is_elf_eravm(), "The bytecode is an ELF file");
}

fn library_passed_post_compile_time(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments =
        vec!["test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let linker_symbols = Libraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&linker_symbols)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_second_call(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments =
        vec!["test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let linker_symbols = Libraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked_empty = memory_buffer
        .link_module_eravm(&BTreeMap::new())
        .expect("Link failure");
    let memory_buffer_linked = memory_buffer_linked_empty
        .link_module_eravm(&linker_symbols)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_redundant_args(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments = vec![
        "fake.sol:Fake=0x0000000000000000000000000000000000000000".to_owned(),
        "scam.sol:Scam=0x0000000000000000000000000000000000000000".to_owned(),
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ];
    let linker_symbols = Libraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&linker_symbols)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_non_elf(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments =
        vec!["test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = Libraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(Libraries::default(), &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&libraries)
        .expect("Link failure");
    let memory_buffer_linked_non_elf = memory_buffer_linked
        .link_module_eravm(&libraries)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked_non_elf.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_produce_equal_bytecode_in_both_cases(
    version: semver::Version,
    codegen: SolcStandardJsonInputSettingsCodegen,
) {
    let library_arguments =
        vec!["test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = Libraries::try_from(library_arguments.as_slice()).expect("Always valid");
    let linker_symbols = libraries.as_linker_symbols().expect("Always valid");

    let bytecode_compile_time = get_bytecode(libraries, &version, codegen);
    let memory_buffer_compile_time = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode_compile_time.as_slice(),
        "bytecode_compile_time",
        false,
    );

    let bytecode_post_compile_time = get_bytecode(Libraries::default(), &version, codegen);
    let memory_buffer_post_compile_time =
        inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            bytecode_post_compile_time.as_slice(),
            "bytecode_post_compile_time",
            false,
        );
    let memory_buffer_linked_post_compile_time = memory_buffer_post_compile_time
        .link_module_eravm(&linker_symbols)
        .expect("Link failure");

    dbg!(
        hex::encode(memory_buffer_compile_time.as_slice()),
        hex::encode(memory_buffer_linked_post_compile_time.as_slice())
    );
    assert!(
        memory_buffer_compile_time.as_slice() == memory_buffer_linked_post_compile_time.as_slice(),
        "The bytecodes are not equal"
    );
}
