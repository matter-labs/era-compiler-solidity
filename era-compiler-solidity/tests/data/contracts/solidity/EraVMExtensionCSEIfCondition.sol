// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

// The call is in the condition of the `if` right after the declaration of `a`.
contract EraVMExtensionCSEIfCondition {
    function f(uint256 n) external view returns (uint256 s) {
        assembly {
            let a := 0xFFFC
            if iszero(staticcall(gas(), a, 0, 0, 0, 0)) { revert(0, 0) }
            for { let i := 0 } lt(i, n) { i := add(i, 1) } { a := add(a, i) }
            s := a
        }
    }
}
