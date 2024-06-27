//!
//! The Solidity compiler unit tests for messages.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::message_type::MessageType;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::Compiler as SolcCompiler;

#[test]
fn send_04_evmla() {
    send(
        semver::Version::new(0, 4, 26),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_04,
    );
}
#[test]
fn send_05_evmla() {
    send(
        semver::Version::new(0, 5, 17),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_06_evmla() {
    send(
        semver::Version::new(0, 6, 12),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_07_evmla() {
    send(
        semver::Version::new(0, 7, 6),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_08_evmla() {
    send(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE,
    );
}
#[test]
fn send_08_yul() {
    send(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
        SEND_TEST_SOURCE,
    );
}

#[test]
fn send_suppressed_04_evmla() {
    send_suppressed(
        semver::Version::new(0, 4, 26),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_04,
    );
}
#[test]
fn send_suppressed_05_evmla() {
    send_suppressed(
        semver::Version::new(0, 5, 17),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_suppressed_06_evmla() {
    send_suppressed(
        semver::Version::new(0, 6, 12),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_suppressed_07_evmla() {
    send_suppressed(
        semver::Version::new(0, 7, 6),
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE_05_07,
    );
}
#[test]
fn send_suppressed_08_evmla() {
    send_suppressed(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
        SEND_TEST_SOURCE,
    );
}
#[test]
fn send_suppressed_08_yul() {
    send_suppressed(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
        SEND_TEST_SOURCE,
    );
}

#[test]
fn transfer_04_evmla() {
    transfer(
        semver::Version::new(0, 4, 26),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_04,
    );
}
#[test]
fn transfer_05_evmla() {
    transfer(
        semver::Version::new(0, 5, 17),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_06_evmla() {
    transfer(
        semver::Version::new(0, 6, 12),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_07_evmla() {
    transfer(
        semver::Version::new(0, 7, 6),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_08_evmla() {
    transfer(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE,
    );
}
#[test]
fn transfer_08_yul() {
    transfer(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
        TRANSFER_TEST_SOURCE,
    );
}

#[test]
fn transfer_suppressed_04_evmla() {
    transfer_suppressed(
        semver::Version::new(0, 4, 26),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_04,
    );
}
#[test]
fn transfer_suppressed_05_evmla() {
    transfer_suppressed(
        semver::Version::new(0, 5, 17),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_suppressed_06_evmla() {
    transfer_suppressed(
        semver::Version::new(0, 6, 12),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_suppressed_07_evmla() {
    transfer_suppressed(
        semver::Version::new(0, 7, 6),
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE_05_07,
    );
}
#[test]
fn transfer_suppressed_08_evmla() {
    transfer_suppressed(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::EVMLA,
        TRANSFER_TEST_SOURCE,
    );
}
#[test]
fn transfer_suppressed_08_yul() {
    transfer_suppressed(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcPipeline::Yul,
        TRANSFER_TEST_SOURCE,
    );
}

#[test]
fn runtime_code_05_evmla() {
    runtime_code(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn runtime_code_06_evmla() {
    runtime_code(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn runtime_code_07_evmla() {
    runtime_code(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn runtime_code_08_evmla() {
    runtime_code(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn runtime_code_08_yul() {
    runtime_code(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
fn internal_function_pointer_argument_04() {
    internal_function_pointer_argument(semver::Version::new(0, 4, 26));
}
#[test]
fn internal_function_pointer_argument_05() {
    internal_function_pointer_argument(semver::Version::new(0, 5, 17));
}
#[test]
fn internal_function_pointer_argument_06() {
    internal_function_pointer_argument(semver::Version::new(0, 6, 12));
}
#[test]
fn internal_function_pointer_argument_07() {
    internal_function_pointer_argument(semver::Version::new(0, 7, 6));
}
#[test]
fn internal_function_pointer_argument_08() {
    internal_function_pointer_argument(SolcCompiler::LAST_SUPPORTED_VERSION);
}

#[test]
fn internal_function_pointer_stack_04() {
    internal_function_pointer_stack(semver::Version::new(0, 4, 26));
}
#[test]
fn internal_function_pointer_stack_05() {
    internal_function_pointer_stack(semver::Version::new(0, 5, 17));
}
#[test]
fn internal_function_pointer_stack_06() {
    internal_function_pointer_stack(semver::Version::new(0, 6, 12));
}
#[test]
fn internal_function_pointer_stack_07() {
    internal_function_pointer_stack(semver::Version::new(0, 7, 6));
}
#[test]
fn internal_function_pointer_stack_08() {
    internal_function_pointer_stack(SolcCompiler::LAST_SUPPORTED_VERSION);
}

#[test]
fn internal_function_pointer_storage_04() {
    internal_function_pointer_storage(semver::Version::new(0, 4, 26));
}
#[test]
fn internal_function_pointer_storage_05() {
    internal_function_pointer_storage(semver::Version::new(0, 5, 17));
}
#[test]
fn internal_function_pointer_storage_06() {
    internal_function_pointer_storage(semver::Version::new(0, 6, 12));
}
#[test]
fn internal_function_pointer_storage_07() {
    internal_function_pointer_storage(semver::Version::new(0, 7, 6));
}
#[test]
fn internal_function_pointer_storage_08() {
    internal_function_pointer_storage(SolcCompiler::LAST_SUPPORTED_VERSION);
}

#[test]
fn tx_origin_04_evmla() {
    tx_origin(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_05_evmla() {
    tx_origin(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_06_evmla() {
    tx_origin(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_07_evmla() {
    tx_origin(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_08_evmla() {
    tx_origin(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_08_yul() {
    tx_origin(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
fn tx_origin_suppressed_04_evmla() {
    tx_origin_suppressed(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_suppressed_05_evmla() {
    tx_origin_suppressed(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_suppressed_06_evmla() {
    tx_origin_suppressed(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_suppressed_07_evmla() {
    tx_origin_suppressed(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_suppressed_08_evmla() {
    tx_origin_suppressed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_suppressed_08_yul() {
    tx_origin_suppressed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
fn tx_origin_assembly_04_evmla() {
    tx_origin_assembly(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_05_evmla() {
    tx_origin_assembly(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_06_evmla() {
    tx_origin_assembly(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_07_evmla() {
    tx_origin_assembly(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_08_evmla() {
    tx_origin_assembly(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_08_yul() {
    tx_origin_assembly(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
fn tx_origin_assembly_suppressed_04_evmla() {
    tx_origin_assembly_suppressed(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_suppressed_05_evmla() {
    tx_origin_assembly_suppressed(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_suppressed_06_evmla() {
    tx_origin_assembly_suppressed(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_suppressed_07_evmla() {
    tx_origin_assembly_suppressed(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_suppressed_08_evmla() {
    tx_origin_assembly_suppressed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn tx_origin_assembly_suppressed_08_yul() {
    tx_origin_assembly_suppressed(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

pub const SEND_TEST_SOURCE_04: &str = r#"
contract SendExample {
    function s() public returns (bool) {
        address r = address(0);
        return r.send(0);
    }
}
"#;

pub const SEND_TEST_SOURCE_05_07: &str = r#"
contract SendExample {
    function s() public returns (bool) {
        address payable r = address(0);
        return r.send(0);
    }
}
"#;

pub const SEND_TEST_SOURCE: &str = r#"
contract SendExample {
    function s() public payable returns (bool) {
        address r = address(0);
        return payable(r).send(msg.value);
    }
}
"#;

fn send(version: semver::Version, pipeline: SolcPipeline, source_code: &str) {
    assert!(super::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![],
    )
    .expect("Test failure"));
}

fn send_suppressed(version: semver::Version, pipeline: SolcPipeline, source_code: &str) {
    assert!(!super::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![MessageType::SendTransfer],
    )
    .expect("Test failure"));
}

pub const TRANSFER_TEST_SOURCE_04: &str = r#"
contract TransferExample {
    function s() public {
        address r = address(0);
        r.transfer(0);
    }
}
"#;

pub const TRANSFER_TEST_SOURCE_05_07: &str = r#"
contract TransferExample {
    function s() public {
        address payable r = address(0);
        r.transfer(0);
    }
}
"#;

pub const TRANSFER_TEST_SOURCE: &str = r#"
contract TransferExample {
    function s() public payable {
        address r = address(0);
        payable(r).transfer(msg.value);
    }
}
"#;

fn transfer(version: semver::Version, pipeline: SolcPipeline, source_code: &str) {
    assert!(super::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![],
    )
    .expect("Test failure"));
}

fn transfer_suppressed(version: semver::Version, pipeline: SolcPipeline, source_code: &str) {
    assert!(!super::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![MessageType::SendTransfer],
    )
    .expect("Test failure"));
}

pub const RUNTIME_CODE_SOURCE_CODE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.5.3;

contract A {}

contract Test {
    function main() public pure returns(bytes memory) {
        return type(A).runtimeCode;
    }
}
"#;

fn runtime_code(version: semver::Version, pipeline: SolcPipeline) {
    assert!(super::check_solidity_message(
        RUNTIME_CODE_SOURCE_CODE,
        "Deploy and runtime code are merged together on ZKsync",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![],
    )
    .expect("Test failure"));
}

fn internal_function_pointer_argument(version: semver::Version) {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

contract InternalFunctionPointerExample {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }

    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return a - b;
    }

    function executeOperation(
        function (uint256, uint256) internal pure returns (uint256) operation,
        uint256 a,
        uint256 b
    ) private pure returns (uint256) {
        return operation(a, b);
    }

    function testAdd(uint256 a, uint256 b) public pure returns (uint256) {
        return executeOperation(add, a, b);
    }

    function testSub(uint256 a, uint256 b) public pure returns (uint256) {
        return executeOperation(sub, a, b);
    }
}
    "#;

    assert!(super::check_solidity_message(
        source_code,
        "Internal function pointers are not supported in the EVM assembly pipeline.",
        BTreeMap::new(),
        &version,
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}

fn internal_function_pointer_stack(version: semver::Version) {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

contract StackFunctionPointerExample {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }

    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return a - b;
    }

    function testAdd(uint256 a, uint256 b) public pure returns (uint256) {
        function (uint256, uint256) internal pure returns (uint256) operation = add;
        return operation(a, b);
    }

    function testSub(uint256 a, uint256 b) public pure returns (uint256) {
        function (uint256, uint256) internal pure returns (uint256) operation = sub;
        return operation(a, b);
    }
}
    "#;

    assert!(super::check_solidity_message(
        source_code,
        "Internal function pointers are not supported in the EVM assembly pipeline.",
        BTreeMap::new(),
        &version,
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}

fn internal_function_pointer_storage(version: semver::Version) {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

contract StorageFunctionPointerExample {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }

    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return a - b;
    }

    function (uint256, uint256) internal pure returns (uint256) operation;
    bool private isOperationSet = false;

    function setOperation(bool isAdd) public {
        if (isAdd) {
            operation = add;
        } else {
            operation = sub;
        }
        isOperationSet = true;
    }

    function executeOperation(uint256 a, uint256 b) public view returns (uint256) {
        require(isOperationSet, "Operation not set");
        return operation(a, b);
    }
}
    "#;

    assert!(super::check_solidity_message(
        source_code,
        "Internal function pointers are not supported in the EVM assembly pipeline.",
        BTreeMap::new(),
        &version,
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_TEST_SOURCE: &str = r#"
contract TxOriginExample {
    function main() private {
        address txOrigin = tx.origin;
    }
}
"#;

fn tx_origin(version: semver::Version, pipeline: SolcPipeline) {
    assert!(super::check_solidity_message(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![],
    )
    .expect("Test failure"));
}

fn tx_origin_suppressed(version: semver::Version, pipeline: SolcPipeline) {
    assert!(!super::check_solidity_message(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![MessageType::TxOrigin],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_ASSEMBLY_TEST_SOURCE: &str = r#"
contract TxOriginExample {
    function main() private {
        assembly {
            let txOrigin := origin()
        }
    }
}
"#;

fn tx_origin_assembly(version: semver::Version, pipeline: SolcPipeline) {
    assert!(super::check_solidity_message(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![],
    )
    .expect("Test failure"));
}

fn tx_origin_assembly_suppressed(version: semver::Version, pipeline: SolcPipeline) {
    assert!(!super::check_solidity_message(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin' in your code, which might lead to",
        BTreeMap::new(),
        &version,
        pipeline,
        false,
        vec![MessageType::TxOrigin],
    )
    .expect("Test failure"));
}
