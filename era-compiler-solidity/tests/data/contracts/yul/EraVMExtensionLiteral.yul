object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // A literal destination selects the `meta` extension.
            let result := staticcall(0, 0xFFFC, 0, 0, 0, 0)
            mstore(0, result)
            return(0, 32)
        }
    }
}
