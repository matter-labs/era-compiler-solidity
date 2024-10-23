//!
//! The Solidity compiler unit tests for the linker.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_solidity::libraries::Libraries;
use era_compiler_solidity::solc::pipeline::Pipeline as SolcPipeline;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
fn library_not_passed_compile_time_08_evmla() {
    library_not_passed_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_not_passed_compile_time_08_yul() {
    library_not_passed_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}
#[test]
fn library_not_passed_post_compile_time_08_evmla() {
    library_not_passed_post_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_not_passed_post_compile_time_08_yul() {
    library_not_passed_post_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}
#[test]
fn library_passed_compile_time_08_evmla() {
    library_passed_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_passed_compile_time_08_yul() {
    library_passed_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}
#[test]
fn library_passed_post_compile_time_08_evmla() {
    library_passed_post_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_passed_post_compile_time_08_yul() {
    library_passed_post_compile_time(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}
#[test]
fn library_passed_post_compile_time_second_call_08_evmla() {
    library_passed_post_compile_time_second_call(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
    );
}
#[test]
fn library_passed_post_compile_time_second_call_08_yul() {
    library_passed_post_compile_time_second_call(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
    );
}
#[test]
fn library_passed_post_compile_time_redundant_args_08_evmla() {
    library_passed_post_compile_time_redundant_args(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
    );
}
#[test]
fn library_passed_post_compile_time_redundant_args_08_yul() {
    library_passed_post_compile_time_redundant_args(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
    );
}
#[test]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf_08_evmla() {
    library_passed_post_compile_time_non_elf(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
    );
}
#[test]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf_08_yul() {
    library_passed_post_compile_time_non_elf(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
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
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    version: &semver::Version,
    pipeline: SolcPipeline,
) -> Vec<u8> {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let build = common::build_solidity(
        sources,
        libraries,
        BTreeSet::new(),
        version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let bytecode_hexadecimal = build
        .contracts
        .as_ref()
        .expect("Missing field `contracts`")
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

fn library_not_passed_compile_time(version: semver::Version, pipeline: SolcPipeline) {
    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

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

fn library_not_passed_post_compile_time(version: semver::Version, pipeline: SolcPipeline) {
    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

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

fn library_passed_compile_time(version: semver::Version, pipeline: SolcPipeline) {
    let libraries = Libraries::into_standard_json(vec![
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let bytecode = get_bytecode(libraries, &version, pipeline);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(!memory_buffer.is_elf_eravm(), "The bytecode is an ELF file");
}

fn library_passed_post_compile_time(version: semver::Version, pipeline: SolcPipeline) {
    let libraries = Libraries::into_linker(vec![
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&libraries)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_second_call(version: semver::Version, pipeline: SolcPipeline) {
    let libraries = Libraries::into_linker(vec![
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked_empty = memory_buffer
        .link_module_eravm(&BTreeMap::new())
        .expect("Link failure");
    let memory_buffer_linked = memory_buffer_linked_empty
        .link_module_eravm(&libraries)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_redundant_args(
    version: semver::Version,
    pipeline: SolcPipeline,
) {
    let libraries = Libraries::into_linker(vec![
        "fake.sol:Fake=0x0000000000000000000000000000000000000000".to_owned(),
        "scam.sol:Scam=0x0000000000000000000000000000000000000000".to_owned(),
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&libraries)
        .expect("Link failure");
    assert!(
        !memory_buffer_linked.is_elf_eravm(),
        "The bytecode is an ELF file"
    );
}

fn library_passed_post_compile_time_non_elf(version: semver::Version, pipeline: SolcPipeline) {
    let libraries = Libraries::into_linker(vec![
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let bytecode = get_bytecode(BTreeMap::new(), &version, pipeline);

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
