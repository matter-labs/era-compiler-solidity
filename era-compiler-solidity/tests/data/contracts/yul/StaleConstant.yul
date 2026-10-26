object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // `dest` first holds the `meta` simulation selector, so the constant
            // propagation caches 0xFFFC against it. The reassignment must drop
            // that entry: the call below goes to 0x8002 and has to be compiled
            // as an ordinary static call, not as the meta extension.
            let dest := 0xFFFC
            dest := 0x8002
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
