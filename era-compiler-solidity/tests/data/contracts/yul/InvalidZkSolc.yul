object "Test" {
    code {
        {
            return(0, 0)
        }
    }

    object "Test_deployed" {
        code {
            {
                selfdestruct(0)
                return(0, 32)
            }
        }
    }
}
