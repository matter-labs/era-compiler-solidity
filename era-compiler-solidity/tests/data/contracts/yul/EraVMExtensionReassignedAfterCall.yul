object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // `dest` holds the `meta` selector at the call and is changed only after it.
            let dest := 0xFFFC
            let result := staticcall(0, dest, 0, 0, 0, 0)
            for { let i := 0 } lt(i, calldataload(0)) { i := add(i, 1) } {
                dest := add(dest, i)
            }
            mstore(0, result)
            mstore(32, dest)
            return(0, 64)
        }
    }
}
