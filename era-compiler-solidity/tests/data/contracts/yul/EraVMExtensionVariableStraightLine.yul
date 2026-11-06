object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // Other statements between the declaration and the call do not change `dest`.
            let dest := 0xFFFC
            let pointer := mload(0x40)
            mstore(pointer, 1)
            let result := staticcall(0, dest, pointer, 32, pointer, 32)
            mstore(0, result)
            return(0, 32)
        }
    }
}
