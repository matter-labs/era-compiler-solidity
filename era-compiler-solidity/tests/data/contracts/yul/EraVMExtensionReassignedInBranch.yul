object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // The branch may change `dest` before the call, so the call goes to `dest`.
            let dest := 0xFFFC
            if calldataload(0) {
                dest := 0x8002
            }
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
