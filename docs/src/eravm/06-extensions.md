# Extensions

EraVM extensions are a set of additional instructions that can be expressed in Solidity and Yul, that can only be compiled to EraVM bytecode.

There are two ways of using EraVM extensions with *zksolc*:

1. [Call simulations](#call-simulations) in Solidity.
2. [`verbatim`](#verbatim) function in Yul mode.

### Call simulations

Since *zksolc* could only operate on Yul received from *solc*, it was not possible to add EraVM-specific functionality to Solidity and Yul. Instead, *zksolc* introduced a hack with external call instructions that would be replaced with EraVM-specific instructions during emitting LLVM IR. In such external call instructions, the address argument denotes the instruction type, whereas the rest of the arguments are used as instruction arguments.

Call simulations are the only way to use EraVM extensions in Solidity.

### `verbatim`

In Yul mode, there is a special instruction called `verbatim` that allows emitting EraVM-specific instructions directly from Yul. This instruction is more robust than call simulations, as it allows passing more arguments to the instruction, and it is not affected by the *solc*'s optimizer. Unfortunately, `verbatim` is only available in Yul mode and cannot be used in Solidity.

It is recommended to only use `verbatim` in Yul mode, as it is more robust and less error-prone than call simulations in Solidity.



## Call Types

In addition to EVM-like `call`, `staticcall` and `delegatecall`, EraVM introduces a few more call types:

1. Mimic call
2. System call
3. Raw call

Each of the call types above has [its by-ref modification](#mimic-call-by-reference-0xfff9), which [allows passing pointers](#active-pointers) to ABI data instead of data itself.

### Mimic Call

Mimic call is a call type that allows the caller to execute a call to a contract, but with the ability to specify the address of the contract that will be used as the caller. This is useful for EraVM System Contracts that need to call other contracts on behalf of the user. Essentially, it is a more complete version of `DELEGATECALL`.

For a deeper dive into the Mimic Call, visit [the EraVM formal specification](https://matter-labs.github.io/eravm-spec/spec.html).

### System Call

System call allows passing more arguments to the callee contract using EraVM registers. This is useful for System Contracts that often require auxiliary data that cannot be passed via calldata.

There are also [system mimic calls](#system-mimic-call-0xfffa), which are a combination of both, that is auxiliary arguments can be passed via EraVM registers.

### Raw Call

Raw calls are similar to EVM's `CALL`, `STATICCALL`, and `DELEGATECALL`, but they do not encode the ABI data. Instead, the ABI data is passed as an argument to the instruction. This is useful for EraVM System Contracts that need to call other contracts with a specific ABI data that cannot be encoded in the calldata.



## Active Pointers

Active pointers are a set of calldata and return data pointers stored in global LLVM IR variables. They are not accessible directly from Yul, but they can be used to forward call and return data between contracts.

The number of active pointers is fixed at 10, and they are numbered from 0 to 9. Some instructions can only use the 0th pointer due to the lack of spare arguments to specify the pointer number. In order to use pointers other than the 0th, use [the swap instruction](#active-pointer-swap-0xffd9).

Instructions that use active pointers have a reference to this section.



## Constant Arrays

Constant arrays are a set of global arrays that can be used to store constant values. They are not accessible directly from Yul, but they can be used to store constant values that are used in multiple places in the contract.



# Instruction Reference

The sections below have the following structure:

1. EraVM instruction name and substituted address.
2. Instruction description.
3. Pseudo-code illustrating the behavior under the hood.
4. Solidity call simulation usage example.
5. Yul `verbatim` usage example.

For instance:

## Example (0xXXXX)

Executes an EraVM instruction.

Pseudo-code:
```solidity
return_value = instruction(arg1, arg2, arg3)
```

Solidity usage:
```solidity
assembly {
    let return_value := call(arg1, 0xXXXX, arg2, arg3, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let return_value := verbatim_3i_1o("instruction", arg1, arg2, arg3)
}
```

Full list of instructions:

- [To L1 (0xFFFF)](#to-l1-0xffff)
- [Precompile (0xFFFD)](#precompile-0xfffd)
- [Decommit (0xFFDD)](#decommit-0xffdd)
- [Set Context Value (0xFFF3)](#set-context-value-0xfff3)
- [Set Pubdata Price (0xFFF2)](#set-pubdata-price-0xfff2)
- [Increment TX Counter (0xFFF1)](#increment-tx-counter-0xfff1)

- [Code Source (0xFFFE)](#code-source-0xfffe)
- [Meta (0xFFFC)](#meta-0xfffc)
- [Get Calldata Pointer (0xFFF0)](#get-calldata-pointer-0xfff0)
- [Get Call Flags (0xFFEF)](#get-call-flags-0xffef)
- [Get Return Data Pointer (0xFFEE)](#get-return-data-pointer-0xffee)
- [Get Extra ABI Data (0xFFE5)](#get-extra-abi-data-0xffe5)

- [Multiplication with Overflow (0xFFE6)](#multiplication-with-overflow-0xffe6)

- [Event Initialize (0xFFED)](#event-initialize-0xffed)
- [Event Write (0xFFEC)](#event-write-0xffec)

- [Mimic Call (0xFFFB)](#mimic-call-0xfffb)
- [Mimic Call by Reference (0xFFF9)](#mimic-call-by-reference-0xfff9)
- [System Mimic Call (0xFFFA)](#system-mimic-call-0xfffa)
- [System Mimic Call by Reference (0xFFF8)](#system-mimic-call-by-reference-0xfff8)
- [Raw Call (0xFFF7)](#raw-call-0xfff7)
- [Raw Call by Reference (0xFFF6)](#raw-call-by-reference-0xfff6)
- [System Call (0xFFF5)](#system-call-0xfff5)
- [System Call by Reference (0xFFF4)](#system-call-by-reference-0xfff4)

- [Active Pointer: Load Calldata (0xFFEB)](#active-pointer-load-calldata-0xffeb)
- [Active Pointer: Load Return Data (0xFFEA)](#active-pointer-load-return-data-0xffea)
- [Active Pointer: Load Decommit (0xFFDC)](#active-pointer-load-decommit-0xffdc)
- [Active Pointer: Increment (0xFFE9)](#active-pointer-increment-0xffe9)
- [Active Pointer: Shrink (0xFFE8)](#active-pointer-shrink-0xffe8)
- [Active Pointer: Pack (0xFFE7)](#active-pointer-pack-0xffe7)
- [Active Pointer: Load (0xFFE4)](#active-pointer-load-0xffe4)
- [Active Pointer: Copy (0xFFE3)](#active-pointer-copy-0xffe3)
- [Active Pointer: Size (0xFFE2)](#active-pointer-size-0xffe2)
- [Active Pointer: Swap (0xFFD9)](#active-pointer-swap-0xffd9)
- [Active Pointer: Return (0xFFDB)](#active-pointer-return-0xffdb)
- [Active Pointer: Revert (0xFFDA)](#active-pointer-revert-0xffda)

- [Constant Array: Declare (0xFFE1)](#constant-array-declare-0xffe1)
- [Constant Array: Set (0xFFE0)](#constant-array-set-0xffe0)
- [Constant Array: Finalize (0xFFDF)](#constant-array-finalize-0xffdf)
- [Constant Array: Get (0xFFDE)](#constant-array-get-0xffde)

- [Return Deployed (verbatim-only)](#return-deployed-verbatim-only)

- [Throw (verbatim-only)](#throw-verbatim-only)

Notes:

1. The `input_length` parameter is always set to 0xFFFF or non-zero value. It prevents the *solc*'s optimizer from optimizing the call out.
2. Instructions that do not modify state are using `staticcall` instead of `call`.
3. Instructions such as raw calls preserve the call type, so they act as modifiers of `call`, `staticcall`, and `delegatecall`.



## To L1 (0xFFFF)

Send a message to L1.

Pseudo-code:
```solidity
to_l1(is_first, value_1, value_2)
```

Solidity usage:
```solidity
assembly {
    let _ := call(is_first, 0xFFFF, value_1, value_2, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_3i_0o("to_l1", is_first, value_1, value_2)
}
```



## Precompile (0xFFFD)

Calls an EraVM precompile.

Pseudo-code:
```solidity
return_value = precompile(input_data, ergs)
```

Solidity usage:
```solidity
assembly {
    let return_value := staticcall(input_data, 0xFFFD, ergs, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let return_value := verbatim_2i_01("precompile", input_data, ergs)
}
```



## Decommit (0xFFDD)

Calls the EraVM decommit.

Pseudo-code:
```solidity
return_value = decommit(versioned_hash, ergs)
```

Solidity usage:
```solidity
assembly {
    let return_value := staticcall(versioned_hash, 0xFFDD, ergs, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let return_value := verbatim_2i_01("decommit", versioned_hash, ergs)
}
```



## Set Context Value (0xFFF3)

Sets the 128-bit context value. Usually the value is used to pass Ether to the callee contract.

Pseudo-code:
```solidity
set_context_value(value)
```

Solidity usage:
```solidity
assembly {
    let _ := call(0, 0xFFF3, value, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_1i_0o("set_context_u128", value)
}
```



## Set Pubdata Price (0xFFF2)

Sets the public data price.

Pseudo-code:
```solidity
set_pubdata_price(value)
```

Solidity usage:
```solidity
assembly {
    let _ := call(value, 0xFFF2, 0, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_1i_0o("set_pubdata_price", value)
}
```



## Increment TX Counter (0xFFF1)

Increments the EraVM transaction counter.

Pseudo-code:
```solidity
increment_tx_counter()
```

Solidity usage:
```solidity
assembly {
    let _ := call(0, 0xFFF1, 0, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("increment_tx_counter")
}
```



## Code Source (0xFFFE)

Returns the address where the contract is actually deployed, even if it is called with a delegate call. Mostly used in EraVM System Contracts.

Pseudo-code:
```solidity
code_source = code_source()
```

Solidity usage:
```solidity
assembly {
    let code_source := staticcall(0, 0xFFFE, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let code_source := verbatim_0i_1o("code_source")
}
```



## Meta (0xFFFC)

Returns a part of the internal EraVM state.

Pseudo-code:
```solidity
meta = meta()
```

Solidity usage:
```solidity
assembly {
    let meta := staticcall(0, 0xFFFC, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let meta := verbatim_0i_1o("meta")
}
```



## Get Calldata Pointer (0xFFF0)

Returns the ABI-encoded calldata pointer as integer.

Pseudo-code:
```solidity
pointer = get_calldata_pointer()
```

Solidity usage:
```solidity
assembly {
    let pointer := staticcall(0, 0xFFF0, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let pointer := verbatim_0i_1o("get_global::ptr_calldata")
}
```



## Get Call Flags (0xFFEF)

Returns the call flags encoded as 256-bit integer.

Pseudo-code:
```solidity
flags = get_call_flags()
```

Solidity usage:
```solidity
assembly {
    let flags := staticcall(0, 0xFFEF, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let flags := verbatim_0i_1o("get_global::call_flags")
}
```



## Get Return Data Pointer (0xFFEE)

Returns the ABI-encoded return data pointer as integer.

Pseudo-code:
```solidity
pointer = get_return_data_pointer()
```

Solidity usage:
```solidity
assembly {
    let pointer := staticcall(0, 0xFFEE, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let pointer := verbatim_0i_1o("get_global::ptr_return_data")
}
```



## Get Extra ABI Data (0xFFE5)

Returns the N-th extra ABI data value passed via registers `r3`-`r12`.

Pseudo-code:
```solidity
value = get_extra_abi_data(index)
```

Solidity usage:
```solidity
assembly {
    let value := staticcall(index, 0xFFE5, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let value := verbatim_0i_1o("get_global::extra_abi_data_0")
    let value := verbatim_0i_1o("get_global::extra_abi_data_1")
    ...
    let value := verbatim_0i_1o("get_global::extra_abi_data_9")
}
```



## Multiplication with Overflow (0xFFE6)

Performs a multiplication with overflow, returning the higher register.

Pseudo-code:
```solidity
higher_register = mul_high(a, b)
```

Solidity usage:
```solidity
assembly {
    let higher_register := staticcall(a, 0xFFE6, b, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let higher_register := verbatim_2i_1o("mul_high", a, b)
}
```



## Event Initialize (0xFFED)

Initializes a new EVM-like event.

Pseudo-code:
```solidity
event_initialize(value_1, value_2)
```

Solidity usage:
```solidity
assembly {
    let _ := call(value_1, 0xFFED, value_2, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_2i_0o("event_initialize", value_1, value_2)
}
```



## Event Write (0xFFEC)

Writes more data to the previously initialized EVM-like event.

Pseudo-code:
```solidity
event_write(value_1, value_2)
```

Solidity usage:
```solidity
assembly {
    let _ := call(value_1, 0xFFEC, value_2, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_2i_0o("event_write", value_1, value_2)
}
```



## Mimic Call (0xFFFB)

Executes an EraVM mimic call.

Pseudo-code:
```solidity
status = mimic_call(callee_address, mimic_address, abi_data)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFFB, 0, abi_data, mimic_address, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_3i_1o("mimic_call", callee_address, mimic_address, abi_data)
}
```



## Mimic Call by Reference (0xFFF9)

Executes an EraVM mimic call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```solidity
status = mimic_call_byref(callee_address, mimic_address)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF9, 0, 0, mimic_address, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_2i_1o("mimic_call_byref", callee_address, mimic_address)
}
```



## System Mimic Call (0xFFFA)

Executes an EraVM mimic call with additional arguments for System Contracts.

Pseudo-code:
```solidity
status = system_mimic_call(callee_address, mimic_address, abi_data, r3_value, r4_value, [r5_value, r6_value])
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFFA, 0, abi_data, mimic_address, r3_value, r4_value)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_5i_1o("system_mimic_call", callee_address, mimic_address, abi_data, r3_value, r4_value, r5_value, r6_value)
}
```

> Yul's `verbatim` allows passing two more extra arguments as it is no limited by the semantics of the `call` instruction.



## System Mimic Call by Reference (0xFFF8)

Executes an EraVM mimic call with additional arguments for System Contracts, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```solidity
status = system_mimic_call_byref(callee_address, mimic_address, r3_value, r4_value, [r5_value, r6_value])
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF8, 0, 0, mimic_address, r3_value, r4_value)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_4i_1o("system_mimic_call_byref", callee_address, mimic_address, r3_value, r4_value, r5_value, r6_value)
}
```

> Yul's `verbatim` allows passing two more extra arguments as it is no limited by the semantics of the `call` instruction.



## Raw Call (0xFFF7)

Executes an EraVM raw call.

Pseudo-code:
```solidity
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

Yul usage:
```solidity
assembly {
    let status := verbatim_4i_1o("raw_call", callee_address, abi_data, output_offset, output_length)
    let status := verbatim_4i_1o("raw_static_call", callee_address, abi_data, output_offset, output_length)
    let status := verbatim_4i_1o("raw_delegate_call", callee_address, abi_data, output_offset, output_length)
}
```



## Raw Call by Reference (0xFFF6)

Executes an EraVM raw call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```solidity
status = raw_call_byref(callee_address, output_offset, output_length)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF7, 0, 0, 0, output_offset, output_length)
    let status := staticcall(callee_address, 0xFFF7, 0, 0, output_offset, output_length)
    let status := delegatecall(callee_address, 0xFFF7, 0, 0, output_offset, output_length)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_3i_1o("raw_call_byref", callee_address, output_offset, output_length)
    let status := verbatim_3i_1o("raw_static_call_byref", callee_address, output_offset, output_length)
    let status := verbatim_3i_1o("raw_delegate_call_byref", callee_address, output_offset, output_length)
}
```



## System Call (0xFFF5)

Executes an EraVM system call.

Pseudo-code:
```solidity
status = system_call(callee_address, r3_value, r4_value, abi_data, r5_value, r6_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF5, r3_value, r4_value, abi_data, r5_value, r6_value)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_6i_1o("system_call", callee_address, abi_data, r3_value, r4_value, r5_value, r6_value)
    let status := verbatim_6i_1o("system_static_call", callee_address, abi_data, r3_value, r4_value, r5_value, r6_value)
    let status := verbatim_6i_1o("system_delegate_call", callee_address, abi_data, r3_value, r4_value, r5_value, r6_value)
}
```

> Static and delegate system calls are only available in Yul as `verbatim`.



## System Call by Reference (0xFFF4)

Executes an EraVM system call, passing [the 0th active pointer](#active-pointers) instead of ABI data.

Pseudo-code:
```solidity
status = system_call_byref(callee_address, r3_value, r4_value, r5_value, r6_value)
```

Solidity usage:
```solidity
assembly {
    let status := call(callee_address, 0xFFF4, r3_value, r4_value, 0xFFFF, r5_value, r6_value)
}
```

Yul usage:
```solidity
assembly {
    let status := verbatim_5i_1o("system_call_byref", callee_address, r3_value, r4_value, r5_value, r6_value)
    let status := verbatim_5i_1o("system_static_call_byref", callee_address, r3_value, r4_value, r5_value, r6_value)
    let status := verbatim_5i_1o("system_delegate_call_byref", callee_address, r3_value, r4_value, r5_value, r6_value)
}
```

> Static and delegate system calls are only available in Yul as `verbatim`.



## Active Pointer: Load Calldata (0xFFEB)

Loads the calldata pointer to [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_load_calldata()
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(0, 0xFFEB, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("calldata_ptr_to_active")
}
```



## Active Pointer: Load Return Data (0xFFEA)

Loads the return data pointer to [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_load_return_data()
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(0, 0xFFEA, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("return_data_ptr_to_active")
}
```



## Active Pointer: Load Decommit (0xFFDC)

Loads the decommit pointer to [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_load_decommit()
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(0, 0xFFDC, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("decommit_ptr_to_active")
}
```



## Active Pointer: Increment (0xFFE9)

Increments the offset of [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_add(value)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(value, 0xFFE9, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_1i_0o("active_ptr_add_assign", value)
}
```



## Active Pointer: Shrink (0xFFE8)

Decrements the slice length of [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_shrink(value)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(value, 0xFFE8, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_1i_0o("active_ptr_shrink_assign", value)
}
```



## Active Pointer: Pack (0xFFE7)

Writes the upper 128 bits to [the 0th active pointer](#active-pointers).

Pseudo-code:
```solidity
active_ptr_pack(value)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(value, 0xFFE7, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_1i_0o("active_ptr_pack_assign", value)
}
```



## Active Pointer: Load (0xFFE4)

Loads a value from [the 0th active pointer](#active-pointers) at the specified offset, similarly to EVM's `CALLDATALOAD`.

Pseudo-code:
```solidity
value = active_ptr_load(offset)
```

Solidity usage:
```solidity
assembly {
    let value := staticcall(offset, 0xFFE4, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let value := verbatim_1i_1o("active_ptr_data_load", offset)
}
```



## Active Pointer: Copy (0xFFE3)

Copies a slice from the [the 0th active pointer](#active-pointers) to the heap, similarly to EVM's `CALLDATACOPY` and `RETURNDATACOPY`.

Pseudo-code:
```solidity
active_ptr_copy(destination, source, size)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(destination, 0xFFE3, source, 0xFFFF, size, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_3i_0o("active_ptr_data_copy", destination, source, size)
}
```



## Active Pointer: Size (0xFFE2)

Returns the length of the slice referenced by [the 0th active pointer](#active-pointers), similarly to EVM's `CALLDATASIZE` and `RETURNDATASIZE`.

Pseudo-code:
```solidity
size = active_ptr_size()
```

Solidity usage:
```solidity
assembly {
    let size := staticcall(0, 0xFFE2, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let size := verbatim_0i_1o("active_ptr_data_size")
}
```



## Active Pointer: Swap (0xFFD9)

Swaps the Nth and Mth [active pointers](#active-pointers). Swapping allows the active pointer instructions to use pointers other than the 0th.

Pseudo-code:
```solidity
active_ptr_swap(N, M)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(N, 0xFFD9, M, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_2i_0o("active_ptr_swap", N, M)
}
```



## Active Pointer: Return (0xFFDB)

Returns from the contract, using [the 0th active pointer](#active-pointers) as the return data.

Pseudo-code:
```solidity
active_ptr_return()
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(0, 0xFFDB, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("active_ptr_return_forward")
}
```



## Active Pointer: Revert (0xFFDA)

Reverts from the contract, using [the 0th active pointer](#active-pointers) as the return data.

Pseudo-code:
```solidity
active_ptr_revert()
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(0, 0xFFDA, 0, 0xFFFF, 0, 0)
}
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("active_ptr_revert_forward")
}
```



## Constant Array: Declare (0xFFE1)

Declares a new [global array of constants](#constant-arrays). After the array is declared, it must be right away filled with values using [the set instruction](#constant-array-set-0xffe0) and declared final using [the finalization instruction](#constant-array-finalize-0xffdf).

Index must be an 8-bit constant value in the range `[0; 255]`.

Size must be a 16-bit constant value in the range `[0; 65535]`.

Pseudo-code:
```solidity
const_array_declare(index, size)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(index, 0xFFE1, size, 0xFFFF, 0, 0)
}
```

> This instruction is not available in Yul as `verbatim`.



## Constant Array: Set (0xFFE0)

Sets a value in a [global array of constants](#constant-arrays).

Index must be an 8-bit constant value in the range `[0; 255]`.

Size must be a 16-bit constant value in the range `[0; 65535]`.

Value must be a 256-bit constant value.

Pseudo-code:
```solidity
const_array_set(index, size, value)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(index, 0xFFE0, size, 0xFFFF, value, 0)
}
```

> This instruction is not available in Yul as `verbatim`.



## Constant Array: Finalize (0xFFDF)

Finalizes a [global array of constants](#constant-arrays).

Index must be an 8-bit constant value in the range `[0; 255]`.

Pseudo-code:
```solidity
const_array_finalize(index)
```

Solidity usage:
```solidity
assembly {
    let _ := staticcall(index, 0xFFDF, 0, 0xFFFF, 0, 0)
}
```

> This instruction is not available in Yul as `verbatim`.



## Constant Array: Get (0xFFDE)

Gets a value from a [global array of constants](#constant-arrays).

Index must be an 8-bit constant value in the range `[0; 255]`.

Offset must be a 16-bit constant value in the range `[0; 65535]`.

Pseudo-code:
```solidity
value = const_array_get(index, offset)
```

Solidity usage:
```solidity
assembly {
    let value := staticcall(index, 0xFFDE, offset, 0xFFFF, 0, 0)
}
```

> This instruction is not available in Yul as `verbatim`.



## Return Deployed (verbatim-only)

Returns heap data from the constructor.

Since EraVM constructors always return immutables via auxiliary heap, it is not possible to use them for EVM-like scenarios, such as EVM emulators.

Pseudo-code:
```solidity
return_deployed(offset, length)
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_2i_0o("return_deployed", offset, length)
}
```



## Throw (verbatim-only)

Throws a [function-level exception](./03-exception-handling.md#function-level).

For a deeper dive into EraVM exceptions, see [this page](./03-exception-handling.md).

Pseudo-code:
```solidity
throw()
```

Yul usage:
```solidity
assembly {
    let _ := verbatim_0i_0o("throw")
}
```
