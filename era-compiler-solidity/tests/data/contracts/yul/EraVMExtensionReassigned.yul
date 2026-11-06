object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // `dest` is reassigned from the `meta` selector to an ordinary
            // address, so the call goes to 0x8002.
            let dest := 0xFFFC
            dest := 0x8002
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
