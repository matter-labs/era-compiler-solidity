//!
//! The Solidity compiler unit test constants.
//!

macro_rules! test_script_path {
    ($name:ident, $base:expr) => {
        /// Shell script test constants.
        #[cfg(target_os = "windows")]
        pub const $name: &str = concat!($base, ".bat");
        #[cfg(not(target_os = "windows"))]
        pub const $name: &str = concat!($base, ".sh");
    };
}

/// Download directory for `solc` executables.
pub const SOLC_DOWNLOAD_DIRECTORY: &str = "solc-bin";

/// Path to the `solc` executable configuration file.
pub const SOLC_BIN_CONFIG_PATH: &str = "tests/solc-bin.json";

/// A test input file.
pub const TEST_CONTRACTS_PATH: &str = "tests/data/contracts/";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_NAME: &str = "Test.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_PATH: &str = "tests/data/contracts/solidity/Test.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_GREETER_PATH: &str = "tests/data/contracts/solidity/Greeter.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH: &str =
    "tests/data/contracts/solidity/caller/Main.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH: &str =
    "tests/data/contracts/solidity/caller/Callable.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH: &str =
    "tests/data/contracts/solidity/SimpleContract.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH: &str =
    "tests/data/contracts/solidity/LinkedMixedDeps.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH: &str =
    "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH: &str =
    "tests/data/contracts/solidity/Optimized.sol";

/// A test input file.
pub const SOLIDITY_BIN_OUTPUT_NAME_ERAVM: &str = "Test.zbin";

/// A test input file.
pub const SOLIDITY_BIN_OUTPUT_NAME_EVM: &str = "Test.bin";

/// A test input file.
pub const SOLIDITY_ASM_OUTPUT_NAME_ERAVM: &str = "Test.zasm";

/// A test input file.
pub const SOLIDITY_ASM_OUTPUT_NAME_EVM: &str = "Test.asm";

/// A test input file.
pub const TEST_YUL_CONTRACT_PATH: &str = "tests/data/contracts/yul/Default.yul";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_PATH: &str = "tests/data/contracts/llvm_ir/Test.ll";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_EVM_PATH: &str = "tests/data/contracts/llvm_ir/Test.evm.ll";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_INVALID_PATH: &str = "tests/data/contracts/llvm_ir/Invalid.ll";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_PATH: &str = "tests/data/contracts/eravm_assembly/Test.zasm";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_CONTRACT_INVALID_PATH: &str =
    "tests/data/contracts/eravm_assembly/Invalid.zasm";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH: &str =
    "tests/data/standard_json_input/non_existent.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH: &str =
    "tests/data/standard_json_input/invalid_utf8.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH: &str =
    "tests/data/standard_json_input/solidity_solc.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_empty_sources.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_missing_sources.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_invalid.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_RECURSION_PATH: &str =
    "tests/data/standard_json_input/solidity_zksolc_recursion.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/solidity_zksolc_invalid.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_SOLC_PATH: &str = "tests/data/standard_json_input/yul_solc.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/yul_solc_urls_invalid.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_ZKSOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/yul_zksolc_urls_invalid.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_ZKSOLC_BOTH_URLS_AND_CONTENT_PATH: &str =
    "tests/data/standard_json_input/yul_zksolc_both_urls_and_content.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_ZKSOLC_NEITHER_URLS_NOR_CONTENT_PATH: &str =
    "tests/data/standard_json_input/yul_zksolc_neither_urls_nor_content.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_INVALID_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls_invalid.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_MISSING_FILE_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls_missing_file.json";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_STANDARD_JSON_PATH: &str =
    "tests/data/standard_json_input/eravm_assembly_urls.json";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_STANDARD_JSON_INVALID_PATH: &str =
    "tests/data/standard_json_input/eravm_assembly_urls_invalid.json";

/// A test input file.
pub const TEST_ERAVM_ASSEMBLY_STANDARD_JSON_MISSING_FILE_PATH: &str =
    "tests/data/standard_json_input/eravm_assembly_urls_missing_file.json";

/// A test input file.
pub const TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_AND_WARNINGS: &str =
    "tests/data/standard_json_input/suppressed_errors_and_warnings.json";

/// A test input file.
pub const TEST_JSON_CONTRACT_PATH_SUPPRESSED_ERRORS_INVALID: &str =
    "tests/data/standard_json_input/suppressed_errors_invalid.json";

/// A test input file.
pub const TEST_JSON_CONTRACT_PATH_SUPPRESSED_WARNINGS_INVALID: &str =
    "tests/data/standard_json_input/suppressed_warnings_invalid.json";

/// A test input file.
pub const TEST_DISASSEMBLER_BYTECODE_PATH: &str = "tests/data/bytecodes/disassembler.zbin";

/// A test input file.
pub const TEST_LINKER_BYTECODE_PATH: &str = "tests/data/bytecodes/linker.zbin";

/// The temporary directory for data created by tests.
pub const TEST_TEMP_DIRECTORY: &str = "tests/data/temp";

/// A test input file.
/// The linker hexadecimal string bytecode sample path.
/// This file must be copied from `TEST_LINKER_BYTECODE_PATH` before linking and removed afterwards.
pub const TEST_LINKER_BYTECODE_COPY_PATH: &str = "tests/data/temp/linker_copy.zbin";

/// A test input file.
pub const TEST_LINKER_STANDARD_JSON_INPUT_WITH_LIBRARIES_PATH: &str =
    "tests/data/standard_json_input/linker/with_libraries.json";

/// A test input file.
pub const TEST_LINKER_STANDARD_JSON_INPUT_WITHOUT_LIBRARIES_PATH: &str =
    "tests/data/standard_json_input/linker/without_libraries.json";

/// The broken input file path.
pub const TEST_BROKEN_INPUT_PATH: &str = "tests/data/broken.bad";

test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_OUTPUT_ERROR_PATH,
    "tests/scripts/solc_version_output_error"
);
test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_TOO_OLD_PATH,
    "tests/scripts/solc_version_too_old"
);
test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_TOO_NEW_PATH,
    "tests/scripts/solc_version_too_new"
);
test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_LINES_PATH,
    "tests/scripts/solc_version_not_enough_lines"
);
test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_NOT_ENOUGH_WORDS_IN_2ND_LINE_PATH,
    "tests/scripts/solc_version_not_enough_words_in_2nd_line"
);
test_script_path!(
    TEST_SCRIPT_SOLC_VERSION_PARSING_ERROR_PATH,
    "tests/scripts/solc_version_parsing_error"
);

/// A test constant.
pub const LIBRARY_DEFAULT: &str = "tests/data/contracts/solidity/MiniMath.sol:MiniMath=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

/// A test constant.
pub const LIBRARY_CONTRACT_NAME_MISSING: &str =
    "tests/data/contracts/solidity/MiniMath.sol=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

/// A test constant.
pub const LIBRARY_ADDRESS_MISSING: &str = "tests/data/contracts/solidity/MiniMath.sol:MiniMath";

/// A test constant.
pub const LIBRARY_ADDRESS_INVALID: &str =
    "tests/data/contracts/solidity/MiniMath.sol:MiniMath=INVALID";

/// A test constant.
pub const LIBRARY_LINKER: &str =
    "Greeter.sol:GreeterHelper=0x1234567890abcdef1234567890abcdef12345678";

/// A test constant.
pub const LIBRARY_LINKER_CONTRACT_NAME_MISSING: &str =
    "Greeter.sol=0x1234567890abcdef1234567890abcdef12345678";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_MISSING: &str = "Greeter.sol:GreeterHelper";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_INVALID: &str = "Greeter.sol:GreeterHelper=XINVALID";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_INCORRECT_SIZE: &str = "Greeter.sol:GreeterHelper=0x12345678";
