// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// The `meta` selector is written as a literal at the call, but the solc optimizer replaces it
// with the variable `a` declared right before the call, because `a` is changed later.
contract EraVMExtensionCSE {
    function f(uint256 n) external view returns (uint256 r, uint256 s) {
        assembly {
            let a := 0xFFFC
            r := staticcall(gas(), 0xFFFC, 0, 0, 0, 0)
            for { let i := 0 } lt(i, n) { i := add(i, 1) } { a := add(a, i) }
            s := a
        }
    }
}
