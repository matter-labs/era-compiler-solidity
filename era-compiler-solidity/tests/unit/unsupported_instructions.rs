//!
//! Unit tests for unsupported instructions.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use test_case::test_case;

#[test]
#[should_panic(expected = "The `PC` instruction is not supported")]
fn pc_yul() {
    let source_code = r#"
object "ProgramCounter" {
    code {
        datacopy(0, dataoffset("ProgramCounter_deployed"), datasize("ProgramCounter_deployed"))
        return(0, datasize("ProgramCounter_deployed"))
    }
    object "ProgramCounter_deployed" {
        code {
            function getPC() -> programCounter {
                programCounter := pc()
            }

            let pcValue := getPC()
            sstore(0, pcValue)
        }
    }
}
    "#;

    let mut sources = BTreeMap::new();
    sources.insert("test.yul".to_owned(), source_code.to_owned());
    crate::common::build_yul(sources).expect("Test failure");
}

#[test]
#[should_panic(expected = "The `CODECOPY` instruction is not supported")]
fn codecopy_runtime() {
    let source_code = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract FixedCodeCopy {
    function copyCode() public pure returns (bytes memory) {
        uint256 fixedCodeSize = 64;
        bytes memory code = new bytes(fixedCodeSize);

        assembly {
            codecopy(add(code, 0x20), 0, fixedCodeSize)
        }

        return code;
    }
}
    "#;

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), source_code.to_owned());

    crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &era_solc::Compiler::LAST_SUPPORTED_VERSION,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const CALLCODE_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract CallcodeTest {
    function testCallcode(address target, bytes4 signature, uint256 inputValue) public returns (bool) {
        bool success;

        assembly {
            let input := mload(0x40)
            mstore(input, signature)
            mstore(add(input, 0x04), inputValue)

            let callResult := callcode(gas(), target, 0, input, 0x24, 0, 0)

            success := and(callResult, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)
        }

        return success;
    }
}
    "#;

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
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        panic!("The `CALLCODE` instruction is not supported");
    }

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), CALLCODE_TEST_SOURCE.to_owned());

    crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const EXTCODECOPY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract ExternalCodeCopy {
    function copyExternalCode(address target, uint256 codeSize) public view returns (bytes memory) {
        bytes memory code = new bytes(codeSize);

        assembly {
            extcodecopy(target, add(code, 0x20), 0, codeSize)
        }

        return code;
    }
}
    "#;

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
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        panic!("The `EXTCODECOPY` instruction is not supported");
    }

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), EXTCODECOPY_TEST_SOURCE.to_owned());

    crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const SELFDESTRUCT_TEST_SOURCE_04: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.12;

contract MinimalDestructible {
    address public owner;

    constructor() public {
        owner = msg.sender;
    }

    function destroy() public {
        require(msg.sender == owner, "Only the owner can call this function.");
        selfdestruct(owner);
    }
}
    "#;

pub const SELFDESTRUCT_TEST_SOURCE_05: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.5.0;

contract MinimalDestructible {
    address payable public owner;

    constructor() public {
        owner = msg.sender;
    }

    function destroy() public {
        require(msg.sender == owner, "Only the owner can call this function.");
        selfdestruct(owner);
    }
}
    "#;

pub const SELFDESTRUCT_TEST_SOURCE_06: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.6.0;

contract MinimalDestructible {
    address payable public owner;

    constructor() public {
        owner = payable(msg.sender);
    }

    function destroy() public {
        require(msg.sender == owner, "Only the owner can call this function.");
        selfdestruct(owner);
    }
}
    "#;

pub const SELFDESTRUCT_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.7.0;

contract MinimalDestructible {
    address payable public owner;

    constructor() {
        owner = payable(msg.sender);
    }

    function destroy() public {
        require(msg.sender == owner, "Only the owner can call this function.");
        selfdestruct(owner);
    }
}
    "#;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SELFDESTRUCT_TEST_SOURCE_04
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SELFDESTRUCT_TEST_SOURCE_05
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SELFDESTRUCT_TEST_SOURCE_06
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SELFDESTRUCT_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA,
    SELFDESTRUCT_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul,
    SELFDESTRUCT_TEST_SOURCE
)]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
    source: &str,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        panic!("The `SELFDESTRUCT` instruction is not supported");
    }

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), source.to_owned());

    crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}
