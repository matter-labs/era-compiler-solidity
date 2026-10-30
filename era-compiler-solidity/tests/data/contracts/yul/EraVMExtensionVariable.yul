object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // Extensions are selected by a literal call destination only.
            // A selector held in a variable is an ordinary call.
            let dest := 0xFFFC
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
