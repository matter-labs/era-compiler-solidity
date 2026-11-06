object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // A variable that holds a selector at the call site selects the extension.
            let dest := 0xFFFC
            let result := staticcall(0, dest, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
