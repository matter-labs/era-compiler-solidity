object "Test" {
    code {
        return(0, 0)
    }
    object "Test_deployed" {
        code {
            // The condition of the next `if` is evaluated right after the declaration.
            let dest := 0xFFFC
            if iszero(staticcall(0, dest, 0, 0, 0, 0)) {
                revert(0, 0)
            }
            return(0, 0)
        }
    }
}
