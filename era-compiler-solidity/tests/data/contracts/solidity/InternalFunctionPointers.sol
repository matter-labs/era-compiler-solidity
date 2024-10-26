// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.4.22;

contract Test {
    uint x;

    function() internal pure returns (uint) fp1;
    function() internal pure returns (uint) fp2;

    function c0() internal pure returns (uint) { return 0xc0; }
    function c1() internal pure returns (uint) { return 0xc1; }

    constructor(int i) public {
        if (i == 0) { fp1 = c0; }
        if (i == 1) { fp2 = c1; }
        x = fp1() + fp2();
    }

    function m() public view returns (uint) {
        return x + fp1() + fp2();
    }
}