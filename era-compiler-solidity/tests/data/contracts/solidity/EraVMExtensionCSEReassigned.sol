// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// The loop may change `a` before the call, so the call goes to `a`.
contract EraVMExtensionCSEReassigned {
    function f(uint256 n) external view returns (uint256 r) {
        assembly {
            let a := 0xFFFC
            for { let i := 0 } lt(i, n) { i := add(i, 1) } { a := add(a, 1) }
            r := staticcall(gas(), a, 0, 0, 0, 0)
        }
    }
}
