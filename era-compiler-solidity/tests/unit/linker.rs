//!
//! The Solidity compiler unit tests for the linker.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use test_case::test_case;

use crate::common;

fn get_bytecode(
    libraries: era_solc::StandardJsonInputLibraries,
    version: &semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) -> Vec<u8> {
    let sources = common::read_sources(&[common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH]);

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
        .get(common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH)
        .expect("Missing file")
        .get("SimpleContract")
        .expect("Missing contract")
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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_not_passed_compile_time(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_not_passed_post_compile_time(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_passed_compile_time(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
        .expect("Always valid");

    let bytecode = get_bytecode(libraries, &version, codegen);

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    assert!(!memory_buffer.is_elf_eravm(), "The bytecode is an ELF file");
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_passed_post_compile_time(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let linker_symbols =
        era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
            .expect("Always valid")
            .as_linker_symbols()
            .expect("Always valid");

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_passed_post_compile_time_second_call(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let linker_symbols =
        era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
            .expect("Always valid")
            .as_linker_symbols()
            .expect("Always valid");

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_passed_post_compile_time_redundant_args(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments = vec![
        "tests/data/contracts/solidity/fake.sol:Fake=0x0000000000000000000000000000000000000000".to_owned(),
        "tests/data/contracts/solidity/scam.sol:Scam=0x0000000000000000000000000000000000000000".to_owned(),
        "tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned(),
    ];
    let linker_symbols =
        era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
            .expect("Always valid")
            .as_linker_symbols()
            .expect("Always valid");

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

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

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
#[should_panic(expected = "Input binary is not an EraVM ELF file")]
fn library_passed_post_compile_time_non_elf(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
        .expect("Always valid")
        .as_linker_symbols()
        .expect("Always valid");

    let bytecode = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode.as_slice(),
        "bytecode",
        false,
    );
    let memory_buffer_linked = memory_buffer
        .link_module_eravm(&libraries)
        .expect("Link failure");
    let _memory_buffer_linked_non_elf = memory_buffer_linked
        .link_module_eravm(&libraries)
        .expect("Link failure");
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn library_produce_equal_bytecode_in_both_cases(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let library_arguments =
        vec!["tests/data/contracts/solidity/SimpleContract.sol:SimpleLibrary=0x1234567890abcdef1234567890abcdef12345678".to_owned()];
    let libraries = era_solc::StandardJsonInputLibraries::try_from(library_arguments.as_slice())
        .expect("Always valid");
    let linker_symbols = libraries.as_linker_symbols().expect("Always valid");

    let bytecode_compile_time = get_bytecode(libraries, &version, codegen);
    let memory_buffer_compile_time = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
        bytecode_compile_time.as_slice(),
        "bytecode_compile_time",
        false,
    );

    let bytecode_post_compile_time = get_bytecode(
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    );
    let memory_buffer_post_compile_time =
        inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            bytecode_post_compile_time.as_slice(),
            "bytecode_post_compile_time",
            false,
        );
    let memory_buffer_linked_post_compile_time = memory_buffer_post_compile_time
        .link_module_eravm(&linker_symbols)
        .expect("Link failure");

    assert!(
        memory_buffer_compile_time.as_slice() == memory_buffer_linked_post_compile_time.as_slice(),
        "The bytecodes are not equal"
    );
}
