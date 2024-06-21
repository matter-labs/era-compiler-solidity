//!
//! The Solidity compiler unit tests for messages.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::message_type::MessageType;
use crate::solc::pipeline::Pipeline as SolcPipeline;

#[test]
fn tx_origin() {
    assert!(super::check_solidity_warning(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn tx_origin_suppressed() {
    assert!(!super::check_solidity_warning(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![MessageType::TxOrigin],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_ASSEMBLY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TxOriginExample {
    function isOriginSender() public view returns (bool) {
        address txOrigin;
        address sender = msg.sender;

        assembly {
            txOrigin := origin() // Get the transaction origin using the 'origin' instruction
        }

        return txOrigin == sender;
    }
}
    "#;

#[test]
fn tx_origin_assembly() {
    assert!(super::check_solidity_warning(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn tx_origin_assembly_suppressed() {
    assert!(!super::check_solidity_warning(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin' in your code, which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![MessageType::TxOrigin],
    )
    .expect("Test failure"));
}

pub const SEND_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SendExample {
    address payable public recipient;

    constructor(address payable _recipient) {
        recipient = _recipient;
    }

    function forwardEther() external payable {
        bool success = recipient.send(msg.value);
        require(success, "Failed to send Ether");
    }
}
    "#;

#[test]
fn send() {
    assert!(super::check_solidity_warning(
        SEND_TEST_SOURCE,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn send_suppressed() {
    assert!(!super::check_solidity_warning(
        SEND_TEST_SOURCE,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![MessageType::SendTransfer],
    )
    .expect("Test failure"));
}

pub const TRANSFER_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TransferExample {
    address payable public recipient;

    constructor(address payable _recipient) {
        recipient = _recipient;
    }

    function forwardEther() external payable {
        recipient.transfer(msg.value);
    }
}
    "#;

#[test]
fn transfer() {
    assert!(super::check_solidity_warning(
        TRANSFER_TEST_SOURCE,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn transfer_suppressed() {
    assert!(!super::check_solidity_warning(
        TRANSFER_TEST_SOURCE,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        BTreeMap::new(),
        SolcPipeline::Yul,
        false,
        vec![MessageType::SendTransfer],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TxOriginExample {
    function isOriginSender() public view returns (bool) {
        return tx.origin == msg.sender;
    }
}
    "#;

#[test]
fn internal_function_pointer_argument() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

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

    assert!(super::check_solidity_warning(
        source_code,
        "Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn internal_function_pointer_stack() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

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

    assert!(super::check_solidity_warning(
        source_code,
        "Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}

#[test]
fn internal_function_pointer_storage() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

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

    assert!(super::check_solidity_warning(
        source_code,
        "Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
        true,
        vec![],
    )
    .expect("Test failure"));
}
