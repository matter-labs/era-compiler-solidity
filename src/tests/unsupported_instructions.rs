//!
//! The Solidity compiler unit tests for unsupported instructions.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::Compiler as SolcCompiler;

#[test]
fn codecopy_runtime_04_evmla() {
    codecopy_runtime(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn codecopy_runtime_05_evmla() {
    codecopy_runtime(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn codecopy_runtime_06_evmla() {
    codecopy_runtime(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn codecopy_runtime_07_evmla() {
    codecopy_runtime(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn codecopy_runtime_08_evmla() {
    codecopy_runtime(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CODECOPY` instruction is not supported")]
fn codecopy_runtime_08_yul() {
    codecopy_runtime(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_04_evmla() {
    callcode(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_05_evmla() {
    callcode(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_06_evmla() {
    callcode(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_07_evmla() {
    callcode(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_08_evmla() {
    callcode(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `CALLCODE` instruction is not supported")]
fn callcode_08_yul() {
    callcode(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_04_evmla() {
    extcodecopy(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_05_evmla() {
    extcodecopy(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_06_evmla() {
    extcodecopy(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_07_evmla() {
    extcodecopy(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_08_evmla() {
    extcodecopy(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "The `EXTCODECOPY` instruction is not supported")]
fn extcodecopy_08_yul() {
    extcodecopy(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_04_evmla() {
    selfdestruct(
        semver::Version::new(0, 4, 26),
        SolcPipeline::EVMLA,
        SELFDESTRUCT_TEST_SOURCE_04,
    );
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_05_evmla() {
    selfdestruct(
        semver::Version::new(0, 5, 17),
        SolcPipeline::EVMLA,
        SELFDESTRUCT_TEST_SOURCE_05,
    );
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_06_evmla() {
    selfdestruct(
        semver::Version::new(0, 6, 12),
        SolcPipeline::EVMLA,
        SELFDESTRUCT_TEST_SOURCE_06,
    );
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_07_evmla() {
    selfdestruct(
        semver::Version::new(0, 7, 6),
        SolcPipeline::EVMLA,
        SELFDESTRUCT_TEST_SOURCE,
    );
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_08_evmla() {
    selfdestruct(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
        SELFDESTRUCT_TEST_SOURCE,
    );
}
#[test]
#[should_panic(expected = "The `SELFDESTRUCT` instruction is not supported")]
fn selfdestruct_08_yul() {
    selfdestruct(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
        SELFDESTRUCT_TEST_SOURCE,
    );
}

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
    super::build_yul(sources).expect("Test failure");
}

fn codecopy_runtime(version: semver::Version, pipeline: SolcPipeline) {
    let source_code = r#"
// SPDX-License-Identifier: MIT
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

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        None,
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const CALLCODE_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
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

fn callcode(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), CALLCODE_TEST_SOURCE.to_owned());

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        None,
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const EXTCODECOPY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
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

fn extcodecopy(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), EXTCODECOPY_TEST_SOURCE.to_owned());

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        None,
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

pub const SELFDESTRUCT_TEST_SOURCE_04: &str = r#"
// SPDX-License-Identifier: MIT
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
    // SPDX-License-Identifier: MIT
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
        // SPDX-License-Identifier: MIT
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
// SPDX-License-Identifier: MIT
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

fn selfdestruct(version: semver::Version, pipeline: SolcPipeline, source: &str) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), source.to_owned());

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        None,
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}
