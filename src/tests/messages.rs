//!
//! The Solidity compiler unit tests for AST warnings and errors.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;

#[test]
fn ecrecover() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ECRecoverExample {
    function recoverAddress(
        bytes32 messageHash,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public pure returns (address) {
        return ecrecover(messageHash, v, r, s);
    }
}
    "#;

    assert!(
        super::check_solidity_warning(
            source_code,
            "Warning: It looks like you are using 'ecrecover' to validate a signature of a user account.",
            BTreeMap::new(),
            SolcPipeline::Yul,
        ).expect("Test failure")
    );
}

#[test]
fn send() {
    let source_code = r#"
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

    assert!(
        super::check_solidity_warning(
            source_code,
            "Warning: It looks like you are using '<address payable>.send/transfer(<X>)' without providing",
            BTreeMap::new(),
            SolcPipeline::Yul,
        ).expect("Test failure")
    );
}

#[test]
fn transfer() {
    let source_code = r#"
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

    assert!(
        super::check_solidity_warning(
            source_code,
            "Warning: It looks like you are using '<address payable>.send/transfer(<X>)' without providing",
            BTreeMap::new(),
            SolcPipeline::Yul,
        ).expect("Test failure")
    );
}

#[test]
fn extcodesize() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ExternalCodeSize {
    function getExternalCodeSize(address target) public view returns (uint256) {
        uint256 codeSize;
        assembly {
            codeSize := extcodesize(target)
        }
        return codeSize;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: Your code or one of its dependencies uses the 'extcodesize' instruction,",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn tx_origin() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TxOriginExample {
    function isOriginSender() public view returns (bool) {
        return tx.origin == msg.sender;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are checking for 'tx.origin' in your code, which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn tx_origin_assembly() {
    let source_code = r#"
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

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are checking for 'tx.origin' in your code, which might lead to",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn block_timestamp() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockTimestampExample {
    function getCurrentTimestamp() public view returns (uint256) {
        return block.timestamp;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'block.timestamp' in your code, which might lead to unexpected behaviour.",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn block_timestamp_assembly() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockTimestampExample {
    function getCurrentTimestamp() public view returns (uint256) {
        uint256 blockTimestamp;

        assembly {
            blockTimestamp := timestamp() // Get the current block timestamp using the 'timestamp' instruction
        }

        return blockTimestamp;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'block.timestamp' in your code, which might lead to unexpected behaviour.",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn block_number() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockNumberExample {
    function getCurrentBlockNumber() public view returns (uint256) {
        return block.number;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'block.number' in your code which we are planning to change in the near",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn block_number_assembly() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockNumberExample {
    function getCurrentBlockNumber() public view returns (uint256) {
        uint256 blockNumber;

        assembly {
            blockNumber := number() // Get the current block number using the 'number' instruction
        }

        return blockNumber;
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'block.number' in your code which we are planning to change in the near",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn blockhash() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockHashExample {
    function getBlockHash(uint blockNumber) public view returns (bytes32) {
        return blockhash(blockNumber);
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'blockHash' in your code which we are planning to change in the near",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

#[test]
fn blockhash_assembly() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BlockHashExampleAssembly {
    function getBlockHash(uint blockNumber) public view returns (bytes32 blockHash) {
        assembly {
            blockHash := blockhash(blockNumber)
        }
    }
}
    "#;

    assert!(super::check_solidity_warning(
        source_code,
        "Warning: You are using 'blockHash' in your code which we are planning to change in the near",
        BTreeMap::new(),
        SolcPipeline::Yul,
    )
    .expect("Test failure"));
}

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
        "Error: Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
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
        "Error: Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
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
        "Error: Internal function pointers are not supported in EVM legacy assembly pipeline.",
        BTreeMap::new(),
        SolcPipeline::EVMLA,
    )
    .expect("Test failure"));
}
