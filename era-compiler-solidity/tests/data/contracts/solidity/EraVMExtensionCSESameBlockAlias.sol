// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// The solc optimizer replaces `y` with `x`, which holds the same value in the same block.
contract EraVMExtensionCSESameBlockAlias {
    function f(uint256 n) external returns (uint256 r, uint256 s) {
        assembly {
            let x := 0xFFFC
            sstore(0, n)
            let y := 0xFFFC
            r := staticcall(gas(), y, 0, 0, 0, 0)
            for { let i := 0 } lt(i, n) { i := add(i, 1) } { x := add(x, i) }
            s := x
        }
    }
}
