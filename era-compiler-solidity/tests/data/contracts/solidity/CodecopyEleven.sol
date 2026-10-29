// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/// A hand-written `codecopy` to memory offset 11, which is also the offset
/// solc's library deploy stub uses. The source is a runtime value, so this is
/// an ordinary copy and must not be replaced with the library marker byte.
contract Test {
    function main(uint256 src) external pure returns (uint256 r) {
        assembly {
            codecopy(11, src, 32)
            r := mload(11)
        }
    }
}
