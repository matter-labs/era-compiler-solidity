object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // The two `dest` variables live in sibling blocks and are unrelated.
            // The second one is not a constant, so the call is ordinary.
            {
                let dest := 0xFFFC
                mstore(0, dest)
            }
            {
                let dest := calldataload(0)
                let result := staticcall(0, dest, 0, 0, 0, 0)
                mstore(0, result)
                return(0, 32)
            }
        }
    }
}
