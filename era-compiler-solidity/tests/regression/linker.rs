//!
//! The Solidity compiler unit tests for the linker.
//!

use std::collections::BTreeMap;

use era_compiler_solidity::libraries::Libraries;
use era_compiler_solidity::solc::pipeline::Pipeline as SolcPipeline;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
fn library_not_passed_08_evmla() {
    library_not_passed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_not_passed_08_yul() {
    library_not_passed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}
#[test]
fn library_passed_08_evmla() {
    library_passed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn library_passed_08_yul() {
    library_passed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
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

fn library_not_passed(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let build = common::build_solidity(
        sources,
        BTreeMap::new(),
        None,
        &version,
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
    let bytecode = hex::decode(bytecode_hexadecimal).expect("Invalid bytecode");
    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(memory_buffer.is_elf(), "The bytecode is not an ELF file");
}

fn library_passed(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let libraries = Libraries::into_standard_json(vec![
        "test.sol:GreaterHelper=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ])
    .expect("Always valid");

    let build = common::build_solidity(
        sources,
        libraries,
        None,
        &version,
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
    let bytecode = hex::decode(bytecode_hexadecimal).expect("Invalid bytecode");
    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(!memory_buffer.is_elf(), "The bytecode is an ELF file");
}
