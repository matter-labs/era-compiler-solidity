# EraVM Extensions

EraVM extensions are a set of additional instructions that can be expressed in Solidity and Yul, but can only be compiled to EraVM bytecode.

Since *zksolc* could only operate on Yul received from *solc*, it was not possible to add EraVM-specific functionality to Solidity and Yul. Instead, *zksolc* introduced a hack with external call instructions that would be replaced with EraVM-specific instructions during emitting LLVM IR. In such external call instructions, the address argument denotes the instruction type, whereas the rest of the arguments are used as instruction arguments.

In other parts of the ZKsync documentation, this hack can be referred to as *Call Simulations*.



## Active Pointers

Active pointers are a set of calldata and return data pointers stored in global LLVM IR variables. They are not accessible directly from Yul, but they can be used to forward call and return data between contracts. The number of active pointers is fixed at 10, and they are numbered from 0 to 9. Some instructions can only use the 0th pointer due to the lack of spare arguments to specify the pointer number.

Instructions that use active pointers have a reference to this section.



## Documentation Structure

The sections below have the following structure:
1. EraVM instruction name and substituted address.
2. Instruction description.
3. Pseudo-code with arguments.
4. Solidity usage example.

For instance:

## Example (0xXXXX)

Executes an EraVM instruction.

Pseudo-code:
```
return_value = instruction(arg1, arg2, arg3)
```

Solidity usage:
```solidity
assembly {
    let return_value := call(arg1, 0xXXXX, arg2, arg3, 0xFFFF, 0, 0)
}
```

### Notes

1. The 5th parameter that is supposed to be `input_length` is always set to 0xFFFF or non-zero argument. It prevents the *solc*'s optimizer from removing the call.
2. Instructions that do not modify state are using `staticcall` instead of `call`.
3. Instructions such as `raw_call` preserve the call type, so they can be used with `call`, `staticcall`, and `delegatecall`.



# Instruction Reference

## To L1 (0xFFFF)

Send a message to L1.

Pseudo-code:
```
to_l1(is_first, value_1, value_2)
```

Solidity usage:
```solidity
assembly {
    let _ := call(is_first, 0xFFFF, value_1, value_2, 0xFFFF, 0, 0)
}
```



## Code Source (0xFFFE)

Returns the address where the contract is actually deployed, even if it is called with a delegate call. Mostly used in EraVM System Contracts.

Pseudo-code:
```
code_source = code_source()
```

Solidity usage:
```solidity
assembly {
    let code_source := staticcall(0, 0xFFFE, 0, 0xFFFF, 0, 0)
}
```



## Precompile (0xFFFD)

Calls an EraVM precompile.

Pseudo-code:
```
return_value = precompile(input_data, ergs)
```

Solidity usage:
```solidity
assembly {
    let return_value := staticcall(input_data, 0xFFFD, ergs, 0xFFFF, 0, 0)
}
```



## Decommit (0xFFDD)

Calls the EraVM decommit.

Pseudo-code:
```
return_value = decommit(versioned_hash, ergs)
```

Solidity usage:
```solidity
assembly {
    let return_value := staticcall(versioned_hash, 0xFFDD, ergs, 0xFFFF, 0, 0)
}
```



## Meta (0xFFFC)

Returns a part of the internal EraVM state.

Pseudo-code:
```
meta = meta()
```

Solidity usage:
```solidity
assembly {
    let meta := staticcall(0, 0xFFFC, 0, 0xFFFF, 0, 0)
}
```



## Mimic Call (0xFFFB)

Executes an EraVM mimic call.

Pseudo-code:
```
status = mimic_call(callee_address, abi_data, mimic_address)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFFB, 0, abi_data, mimic_address, 0, 0)
}
```



## System Mimic Call (0xFFFA)

Executes an EraVM mimic call with additional arguments for System Contracts.

Pseudo-code:
```
status = system_mimic_call(callee_address, abi_data, mimic_address, r3_value, r4_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFFA, 0, abi_data, mimic_address, r3_value, r4_value)
}
```



## Mimic Call by Reference (0xFFF9)

Executes an EraVM mimic call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```
status = mimic_call_by_ref(callee_address, mimic_address)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF9, 0, 0, mimic_address, 0, 0)
}
```



## System Mimic Call by Reference (0xFFF8)

Executes an EraVM mimic call with additional arguments for System Contracts, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```
status = system_mimic_call_by_ref(callee_address, mimic_address, r3_value, r4_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF8, 0, 0, mimic_address, r3_value, r4_value)
}
```



## Raw Call (0xFFF7)

Executes an EraVM raw call.

Pseudo-code:
```
status = raw_call(callee_address, abi_data, output_offset, output_length)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF7, 0, 0, abi_data, output_offset, output_length)
    let status := staticcall(callee_address, 0xFFF7, 0, abi_data, output_offset, output_length)
    let status := delegatecall(callee_address, 0xFFF7, 0, abi_data, output_offset, output_length)
}
```



## Raw Call by Reference (0xFFF6)

Executes an EraVM raw call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```
status = raw_call_by_ref(callee_address, output_offset, output_length)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF7, 0, 0, 0, output_offset, output_length)
}
```



## System Call (0xFFF5)

Executes an EraVM system call.

Pseudo-code:
```
status = system_call(callee_address, r3_value, r4_value, abi_data, r5_value, r6_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF5, r3_value, r4_value, abi_data, r5_value, r6_value)
}
```



## System Call (0xFFF4)

Executes an EraVM system call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```
status = system_call(callee_address, r3_value, r4_value, r5_value, r6_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF4, r3_value, r4_value, 0xFFFF, r5_value, r6_value)
}
```