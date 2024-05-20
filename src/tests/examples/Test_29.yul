object "Test_29" {
    code {
        {
            return(0, 0)
        }
    }

    object "Test_29_deployed" {
        code {
            {
                mstore(0, 42)
                return(0, 32)
            }
        }
    }
}
