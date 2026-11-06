object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // The loop may change `dest` before the call, so the call goes to `dest`.
            let dest := 0xFFFC
            for { let i := 0 } lt(i, calldataload(0)) { i := add(i, 1) } {
                dest := add(dest, 1)
            }
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
