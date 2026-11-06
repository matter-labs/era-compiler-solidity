// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// The solc optimizer replaces the literal with `a` and moves `mload(0x40)` into a variable
// declared between `a` and the call.
contract EraVMExtensionCSEStraightLine {
    function f(uint256 n) external view returns (uint256 r, uint256 s) {
        assembly {
            let a := 0xFFFC
            r := staticcall(gas(), a, mload(0x40), 4, mload(0x40), 32)
            for { let i := 0 } lt(i, n) { i := add(i, 1) } { a := add(a, i) }
            s := a
        }
    }
}
