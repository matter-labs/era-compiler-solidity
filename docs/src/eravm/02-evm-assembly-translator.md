# EVM Assembly Translator

There are two Solidity codegens used in our toolchain:

|    Codegen    |    Supported     |      Default     |
|---------------|------------------|------------------|
| Yul           | >=0.8.0          | >=0.8            |
| EVM Assembly  | >=0.4.12         | <0.8             |



## ZKsync Fork of *solc*

EVM assembly is very challenging to translate to LLVM IR, since it obfuscates the control flow of the contract and
uses a lot of dynamic jumps. 

There are several issues with the original *solc* compiler when it comes to static analysis of EVM assembly:

1. Internal function pointers are written to memory or storage, and then dynamically loaded and called.
2. With local recursion, there is another stack frame allocated on every iteration.
3. Some try-catch patterns leave values on the stack, hindering stack analysis.

All the issues have been resolved in [our fork of *solc*](https://github.com/matter-labs/era-solidity),
where we have changed the codegen to remove the dynamic jumps and add the necessary metadata.



## Source Code

In this and following sections you can see a minimal example of a Solidity contract and its EVM assembly translated
to LLVM IR which is then compiled to EraVM bytecode.

```solidity
contract Example {
  function main() public pure returns (uint256 result) {
    result = 42;
  }
}
```



## EVM Legacy Assembly

Produced by the upstream Solidity compiler v0.7.6.

```txt
| Line | Instruction  | Value/Tag |
| ---- | ------------ | --------- |
| 000  | PUSH         | 80        |
| 001  | PUSH         | 40        |
| 002  | MSTORE       |           |
| 003  | CALLVALUE    |           |
| 004  | DUP1         |           |
| 005  | ISZERO       |           |
| 006  | PUSH         | [tag] 1   |
| 007  | JUMPI        |           |
| 008  | PUSH         | 0         |
| 009  | DUP1         |           |
| 010  | REVERT       |           |
| 011  | Tag 1        |           |
| 012  | JUMPDEST     |           |
| 013  | POP          |           |
| 014  | PUSH         | 4         |
| 015  | CALLDATASIZE |           |
| 016  | LT           |           |
| 017  | PUSH         | [tag] 2   |
| 018  | JUMPI        |           |
| 019  | PUSH         | 0         |
| 020  | CALLDATALOAD |           |
| 021  | PUSH         | E0        |
| 022  | SHR          |           |
| 023  | DUP1         |           |
| 024  | PUSH         | 5A8AC02D  |
| 025  | EQ           |           |
| 026  | PUSH         | [tag] 3   |
| 027  | JUMPI        |           |
| 028  | Tag 2        |           |
| 029  | JUMPDEST     |           |
| 030  | PUSH         | 0         |
| 031  | DUP1         |           |
| 032  | REVERT       |           |
| 033  | Tag 3        |           |
| 034  | JUMPDEST     |           |
| 035  | PUSH         | [tag] 4   |
| 036  | PUSH         | [tag] 5   |
| 037  | JUMP         | [in]      |
| 038  | Tag 4        |           |
| 039  | JUMPDEST     |           |
| 040  | PUSH         | 40        |
| 041  | DUP1         |           |
| 042  | MLOAD        |           |
| 043  | SWAP2        |           |
| 044  | DUP3         |           |
| 045  | MSTORE       |           |
| 046  | MLOAD        |           |
| 047  | SWAP1        |           |
| 048  | DUP2         |           |
| 049  | SWAP1        |           |
| 050  | SUB          |           |
| 051  | PUSH         | 20        |
| 052  | ADD          |           |
| 053  | SWAP1        |           |
| 054  | RETURN       |           |
| 055  | Tag 5        |           |
| 056  | JUMPDEST     |           |
| 057  | PUSH         | 2A        |
| 058  | SWAP1        |           |
| 059  | JUMP         | [out]     |
```



## EthIR

EthIR (Ethereal IR) is an IR developed specifically for our translator. The IR serves several purposes:

1. Tracking the stack state to extract jump destinations.
2. Duplicating blocks that are reachable from predecessors with different stack states.
3. Restoring the complete control-flow graph of the contract using the abovementioned data.
4. Resolving dependencies and static data chunks.

Meaning of EthIR data:

1. `V_<name>` - value returned by an instruction `<name>`.
2. `T_<tag>` - tag of a block `<tag>`.
3. `40` - hexadecimal constant.
4. `tests/solidity/simple/default.sol:Test` - contract definition.

Stack format: `[ V_CALLVALUE ]` (current values) - `[ V_CALLVALUE ]` (popped values) + `[ V_ISZERO ]` (pushed values)

```text
// The default entry function of the contract.
function main {
// The maximum stack size in the function.
    stack_usage: 6
block_dt_0/0:                           // Deploy Code Tag 0, Instance 0.
// PUSHed 0x80 onto the stack.
    PUSH           80                                                               [  ] + [ 80 ]
// PUSHed 0x40 onto the stack.
    PUSH           40                                                               [ 80 ] + [ 40 ]
// POPped 0x40 at 0x80 from the stack to store 0x80 at 0x40.
    MSTORE                                                                          [  ] - [ 80 | 40 ]
// PUSHed CALLVALUE onto the stack.
    CALLVALUE                                                                       [  ] + [ V_CALLVALUE ]
    DUP1                                                                            [ V_CALLVALUE ] + [ V_CALLVALUE ]
    ISZERO                                                                          [ V_CALLVALUE ] - [ V_CALLVALUE ] + [ V_ISZERO ]
    PUSH [tag]     1                                                                [ V_CALLVALUE | V_ISZERO ] + [ T_1 ]
// JUMPI schedules rt_0/0 for analysis with the current stack state.
    JUMPI                                                                           [ V_CALLVALUE ] - [ V_ISZERO | T_1 ]
    PUSH           0                                                                [ V_CALLVALUE ] + [ 0 ]
    DUP1                                                                            [ V_CALLVALUE | 0 ] + [ 0 ]
    REVERT                                                                          [ V_CALLVALUE ] - [ 0 | 0 ]
block_dt_1/0: (predecessors: dt_0/0)    // Deploy Code Tag 1, Instance 0; the only predecessor of this block is dt_0/0.
// JUMPDESTs are ignored as we are only interested in the stack state and tag destinations.
    JUMPDEST                                                                        [ V_CALLVALUE ]
    POP                                                                             [  ] - [ V_CALLVALUE ]
    PUSH #[$]      tests/solidity/simple/default.sol:Test                           [  ] + [ tests/solidity/simple/default.sol:Test ]
    DUP1                                                                            [ tests/solidity/simple/default.sol:Test ] + [ tests/solidity/simple/default.sol:Test ]
    PUSH [$]       tests/solidity/simple/default.sol:Test                           [ tests/solidity/simple/default.sol:Test | tests/solidity/simple/default.sol:Test ] + [ tests/solidity/simple/default.sol:Test ]
    PUSH           0                                                                [ tests/solidity/simple/default.sol:Test | tests/solidity/simple/default.sol:Test | tests/solidity/simple/default.sol:Test ] + [ 0 ]
    CODECOPY                                                                        [ tests/solidity/simple/default.sol:Test ] - [ tests/solidity/simple/default.sol:Test | tests/solidity/simple/default.sol:Test | 0 ]
    PUSH           0                                                                [ tests/solidity/simple/default.sol:Test ] + [ 0 ]
    RETURN                                                                          [  ] - [ tests/solidity/simple/default.sol:Test | 0 ]
// The runtime code is analyzed in the same control-flow graph as the deploy code, as it is possible to call its functions from the constructor.
block_rt_0/0:                           // Deploy Code Tag 0, Instance 0.
    PUSH           80                                                               [  ] + [ 80 ]
    PUSH           40                                                               [ 80 ] + [ 40 ]
    MSTORE                                                                          [  ] - [ 80 | 40 ]
    CALLVALUE                                                                       [  ] + [ V_CALLVALUE ]
    DUP1                                                                            [ V_CALLVALUE ] + [ V_CALLVALUE ]
    ISZERO                                                                          [ V_CALLVALUE ] - [ V_CALLVALUE ] + [ V_ISZERO ]
    PUSH [tag]     1                                                                [ V_CALLVALUE | V_ISZERO ] + [ T_1 ]
    JUMPI                                                                           [ V_CALLVALUE ] - [ V_ISZERO | T_1 ]
    PUSH           0                                                                [ V_CALLVALUE ] + [ 0 ]
    DUP1                                                                            [ V_CALLVALUE | 0 ] + [ 0 ]
    REVERT                                                                          [ V_CALLVALUE ] - [ 0 | 0 ]
block_rt_1/0: (predecessors: rt_0/0)    // Runtime Code Tag 1, Instance 0; the only predecessor of this block is rt_0/0.
    JUMPDEST                                                                        [ V_CALLVALUE ]
    POP                                                                             [  ] - [ V_CALLVALUE ]
    PUSH           4                                                                [  ] + [ 4 ]
    CALLDATASIZE                                                                    [ 4 ] + [ V_CALLDATASIZE ]
    LT                                                                              [  ] - [ 4 | V_CALLDATASIZE ] + [ V_LT ]
    PUSH [tag]     2                                                                [ V_LT ] + [ T_2 ]
    JUMPI                                                                           [  ] - [ V_LT | T_2 ]
    PUSH           0                                                                [  ] + [ 0 ]
    CALLDATALOAD                                                                    [  ] - [ 0 ] + [ V_CALLDATALOAD ]
    PUSH           E0                                                               [ V_CALLDATALOAD ] + [ E0 ]
    SHR                                                                             [  ] - [ V_CALLDATALOAD | E0 ] + [ V_SHR ]
    DUP1                                                                            [ V_SHR ] + [ V_SHR ]
    PUSH           5A8AC02D                                                         [ V_SHR | V_SHR ] + [ 5A8AC02D ]
    EQ                                                                              [ V_SHR ] - [ V_SHR | 5A8AC02D ] + [ V_EQ ]
    PUSH [tag]     3                                                                [ V_SHR | V_EQ ] + [ T_3 ]
    JUMPI                                                                           [ V_SHR ] - [ V_EQ | T_3 ]
    Tag 2                                                                           [ V_SHR ]
// This instance is called with a different stack state using the JUMPI above.
block_rt_2/0: (predecessors: rt_1/0)    // Runtime Code Tag 2, Instance 0.
    JUMPDEST                                                                        [  ]
    PUSH           0                                                                [  ] + [ 0 ]
    DUP1                                                                            [ 0 ] + [ 0 ]
    REVERT                                                                          [  ] - [ 0 | 0 ]
// This instance is also called from rt_1/0, but using a fallthrough 'Tag 2'.
// Given different stack states, we create a new instance of the block operating on different data
// and potentially different tag destinations, although usually such blocks are merged back by LLVM.
block_rt_2/1: (predecessors: rt_1/0)    // Runtime Code Tag 2, Instance 1.
    JUMPDEST                                                                        [ V_SHR ]
    PUSH           0                                                                [ V_SHR ] + [ 0 ]
    DUP1                                                                            [ V_SHR | 0 ] + [ 0 ]
    REVERT                                                                          [ V_SHR ] - [ 0 | 0 ]
block_rt_3/0: (predecessors: rt_1/0)    // Runtime Code Tag 3, Instance 0.
    JUMPDEST                                                                        [ V_SHR ]
    PUSH [tag]     4                                                                [ V_SHR ] + [ T_4 ]
    PUSH [tag]     5                                                                [ V_SHR | T_4 ] + [ T_5 ]
    JUMP           [in]                                                             [ V_SHR | T_4 ] - [ T_5 ]
block_rt_4/0: (predecessors: rt_5/0)    // Runtime Code Tag 4, Instance 0.
    JUMPDEST                                                                        [ V_SHR | 2A ]
    PUSH           40                                                               [ V_SHR | 2A ] + [ 40 ]
    DUP1                                                                            [ V_SHR | 2A | 40 ] + [ 40 ]
    MLOAD                                                                           [ V_SHR | 2A | 40 ] - [ 40 ] + [ V_MLOAD ]
    SWAP2                                                                           [ V_SHR | V_MLOAD | 40 | 2A ]
    DUP3                                                                            [ V_SHR | V_MLOAD | 40 | 2A ] + [ V_MLOAD ]
    MSTORE                                                                          [ V_SHR | V_MLOAD | 40 ] - [ 2A | V_MLOAD ]
    MLOAD                                                                           [ V_SHR | V_MLOAD ] - [ 40 ] + [ V_MLOAD ]
    SWAP1                                                                           [ V_SHR | V_MLOAD | V_MLOAD ]
    DUP2                                                                            [ V_SHR | V_MLOAD | V_MLOAD ] + [ V_MLOAD ]
    SWAP1                                                                           [ V_SHR | V_MLOAD | V_MLOAD | V_MLOAD ]
    SUB                                                                             [ V_SHR | V_MLOAD ] - [ V_MLOAD | V_MLOAD ] + [ V_SUB ]
    PUSH           20                                                               [ V_SHR | V_MLOAD | V_SUB ] + [ 20 ]
    ADD                                                                             [ V_SHR | V_MLOAD ] - [ V_SUB | 20 ] + [ V_ADD ]
    SWAP1                                                                           [ V_SHR | V_ADD | V_MLOAD ]
    RETURN                                                                          [ V_SHR ] - [ V_ADD | V_MLOAD ]
block_rt_5/0: (predecessors: rt_3/0)    // Runtime Code Tag 5, Instance 0.
    JUMPDEST                                                                        [ V_SHR | T_4 ]
    PUSH           2A                                                               [ V_SHR | T_4 ] + [ 2A ]
    SWAP1                                                                           [ V_SHR | 2A | T_4 ]
// JUMP [out] is usually a return statement
    JUMP           [out]                                                            [ V_SHR | 2A ] - [ T_4 ]
```



### Unoptimized LLVM IR

In LLVM IR, the necessary stack space is allocated at the beginning of the function.

Every stack operation interacts with a statically known stack pointer with an offset derived from EthIR.

```llvm
; Function Attrs: nofree null_pointer_is_valid
define i256 @__entry(ptr addrspace(3) %0, i256 %1, i256 %2, i256 %3, i256 %4, i256 %5, i256 %6, i256 %7, i256 %8, i256 %9, i256 %10, i256 %11) #7 personality ptr @__personality {
entry:
  store ptr addrspace(3) %0, ptr @ptr_calldata, align 32
  %abi_pointer_value = ptrtoint ptr addrspace(3) %0 to i256
  %abi_pointer_value_shifted = lshr i256 %abi_pointer_value, 96
  %abi_length_value = and i256 %abi_pointer_value_shifted, 4294967295
  store i256 %abi_length_value, ptr @calldatasize, align 32
  %calldatasize = load i256, ptr @calldatasize, align 32
  %ptr_calldata = load ptr addrspace(3), ptr @ptr_calldata, align 32
  %calldata_end_pointer = getelementptr i8, ptr addrspace(3) %ptr_calldata, i256 %calldatasize
  store ptr addrspace(3) %calldata_end_pointer, ptr @ptr_return_data, align 32
  store ptr addrspace(3) %calldata_end_pointer, ptr @ptr_decommit, align 32
  %calldatasize1 = load i256, ptr @calldatasize, align 32
  %ptr_calldata2 = load ptr addrspace(3), ptr @ptr_calldata, align 32
  %calldata_end_pointer3 = getelementptr i8, ptr addrspace(3) %ptr_calldata2, i256 %calldatasize1
  store ptr addrspace(3) %calldata_end_pointer3, ptr @ptr_active, align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 1), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 2), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 3), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 4), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 5), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 6), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 7), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 8), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 9), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 10), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 11), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 12), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 13), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 14), align 32
  store ptr addrspace(3) %calldata_end_pointer3, ptr getelementptr inbounds ([16 x ptr addrspace(3)], ptr @ptr_active, i256 0, i256 15), align 32
  store i256 %1, ptr @call_flags, align 32
  store i256 %2, ptr @extra_abi_data, align 32
  store i256 %3, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 1), align 32
  store i256 %4, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 2), align 32
  store i256 %5, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 3), align 32
  store i256 %6, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 4), align 32
  store i256 %7, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 5), align 32
  store i256 %8, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 6), align 32
  store i256 %9, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 7), align 32
  store i256 %10, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 8), align 32
  store i256 %11, ptr getelementptr inbounds ([10 x i256], ptr @extra_abi_data, i256 0, i32 9), align 32
  %is_deploy_code_call_flag_truncated = and i256 %1, 1
  %is_deploy_code_call_flag = icmp eq i256 %is_deploy_code_call_flag_truncated, 1
  br i1 %is_deploy_code_call_flag, label %deploy_code_call_block, label %runtime_code_call_block

return:                                           ; preds = %runtime_code_call_block, %deploy_code_call_block
  ret i256 0

deploy_code_call_block:                           ; preds = %entry
  call void @__deploy()
  br label %return

runtime_code_call_block:                          ; preds = %entry
  call void @__runtime()
  br label %return
}

; Function Attrs: nofree null_pointer_is_valid
define private void @__deploy() #7 personality ptr @__personality {
entry:
  call void @main(i1 true)
  br label %return

return:                                           ; preds = %entry
  ret void
}

; Function Attrs: nofree null_pointer_is_valid
define private void @__runtime() #7 personality ptr @__personality {
entry:
  call void @main(i1 false)
  br label %return

return:                                           ; preds = %entry
  ret void
}

; Function Attrs: nofree null_pointer_is_valid
define private void @main(i1 %0) #7 personality ptr @__personality {
entry:
  %stack_var_000 = alloca i256, align 32
  store i256 0, ptr %stack_var_000, align 32
  %stack_var_001 = alloca i256, align 32
  store i256 0, ptr %stack_var_001, align 32
  %stack_var_002 = alloca i256, align 32
  store i256 0, ptr %stack_var_002, align 32
  %stack_var_003 = alloca i256, align 32
  store i256 0, ptr %stack_var_003, align 32
  %stack_var_004 = alloca i256, align 32
  store i256 0, ptr %stack_var_004, align 32
  %stack_var_005 = alloca i256, align 32
  store i256 0, ptr %stack_var_005, align 32
  %stack_var_006 = alloca i256, align 32
  store i256 0, ptr %stack_var_006, align 32
  br i1 %0, label %"block_dt_0/0", label %"block_rt_0/0"

return:                                           ; No predecessors!
  ret void

"block_dt_0/0":                                   ; preds = %entry
  store i256 128, ptr %stack_var_000, align 32
  store i256 64, ptr %stack_var_001, align 32
  %argument_0 = load i256, ptr %stack_var_001, align 32
  %argument_1 = load i256, ptr %stack_var_000, align 32
  %memory_store_pointer = inttoptr i256 %argument_0 to ptr addrspace(1)
  store i256 %argument_1, ptr addrspace(1) %memory_store_pointer, align 1
  %get_u128_value = call i256 @llvm.eravm.getu128()
  store i256 %get_u128_value, ptr %stack_var_000, align 32
  %dup1 = load i256, ptr %stack_var_000, align 32
  store i256 %dup1, ptr %stack_var_001, align 32
  %argument_01 = load i256, ptr %stack_var_001, align 32
  %comparison_result = icmp eq i256 %argument_01, 0
  %comparison_result_extended = zext i1 %comparison_result to i256
  store i256 %comparison_result_extended, ptr %stack_var_001, align 32
  store i256 1, ptr %stack_var_002, align 32
  %conditional_dt_1_condition = load i256, ptr %stack_var_001, align 32
  %conditional_dt_1_condition_compared = icmp ne i256 %conditional_dt_1_condition, 0
  br i1 %conditional_dt_1_condition_compared, label %"block_dt_1/0", label %conditional_dt_1_join_block

"block_dt_1/0":                                   ; preds = %"block_dt_0/0"
  store i256 2, ptr %stack_var_000, align 32
  br label %"block_dt_2/0"

"block_dt_2/0":                                   ; preds = %"block_dt_1/0"
  store i256 0, ptr %stack_var_000, align 32
  %dup14 = load i256, ptr %stack_var_000, align 32
  store i256 %dup14, ptr %stack_var_001, align 32
  store i256 0, ptr %stack_var_002, align 32
  store i256 0, ptr %stack_var_003, align 32
  %argument_05 = load i256, ptr %stack_var_003, align 32
  %argument_16 = load i256, ptr %stack_var_002, align 32
  %argument_2 = load i256, ptr %stack_var_001, align 32
  %calldata_copy_destination_pointer = inttoptr i256 %argument_05 to ptr addrspace(1)
  %calldata_pointer = load ptr addrspace(3), ptr @ptr_calldata, align 32
  %calldata_source_pointer = getelementptr i8, ptr addrspace(3) %calldata_pointer, i256 %argument_16
  call void @llvm.memcpy.p1.p3.i256(ptr addrspace(1) align 1 %calldata_copy_destination_pointer, ptr addrspace(3) align 1 %calldata_source_pointer, i256 %argument_2, i1 false)
  store i256 0, ptr %stack_var_001, align 32
  %argument_07 = load i256, ptr %stack_var_001, align 32
  %argument_18 = load i256, ptr %stack_var_000, align 32
  store i256 32, ptr addrspace(2) inttoptr (i256 256 to ptr addrspace(2)), align 1
  store i256 0, ptr addrspace(2) inttoptr (i256 288 to ptr addrspace(2)), align 1
  call void @__return(i256 256, i256 64, i256 2)
  unreachable

"block_rt_0/0":                                   ; preds = %entry
  store i256 128, ptr %stack_var_000, align 32
  store i256 64, ptr %stack_var_001, align 32
  %argument_09 = load i256, ptr %stack_var_001, align 32
  %argument_110 = load i256, ptr %stack_var_000, align 32
  %memory_store_pointer11 = inttoptr i256 %argument_09 to ptr addrspace(1)
  store i256 %argument_110, ptr addrspace(1) %memory_store_pointer11, align 1
  %get_u128_value12 = call i256 @llvm.eravm.getu128()
  store i256 %get_u128_value12, ptr %stack_var_000, align 32
  %dup113 = load i256, ptr %stack_var_000, align 32
  store i256 %dup113, ptr %stack_var_001, align 32
  %argument_014 = load i256, ptr %stack_var_001, align 32
  %comparison_result15 = icmp eq i256 %argument_014, 0
  %comparison_result_extended16 = zext i1 %comparison_result15 to i256
  store i256 %comparison_result_extended16, ptr %stack_var_001, align 32
  store i256 1, ptr %stack_var_002, align 32
  %conditional_rt_1_condition = load i256, ptr %stack_var_001, align 32
  %conditional_rt_1_condition_compared = icmp ne i256 %conditional_rt_1_condition, 0
  br i1 %conditional_rt_1_condition_compared, label %"block_rt_1/0", label %conditional_rt_1_join_block

"block_rt_1/0":                                   ; preds = %"block_rt_0/0"
  store i256 4, ptr %stack_var_000, align 32
  %calldatasize = load i256, ptr @calldatasize, align 32
  store i256 %calldatasize, ptr %stack_var_001, align 32
  %argument_019 = load i256, ptr %stack_var_001, align 32
  %argument_120 = load i256, ptr %stack_var_000, align 32
  %comparison_result21 = icmp ult i256 %argument_019, %argument_120
  %comparison_result_extended22 = zext i1 %comparison_result21 to i256
  store i256 %comparison_result_extended22, ptr %stack_var_000, align 32
  store i256 2, ptr %stack_var_001, align 32
  %conditional_rt_2_condition = load i256, ptr %stack_var_000, align 32
  %conditional_rt_2_condition_compared = icmp ne i256 %conditional_rt_2_condition, 0
  br i1 %conditional_rt_2_condition_compared, label %"block_rt_2/0", label %conditional_rt_2_join_block

"block_rt_2/0":                                   ; preds = %"block_rt_1/0"
  store i256 0, ptr %stack_var_000, align 32
  store i256 0, ptr %stack_var_001, align 32
  %argument_032 = load i256, ptr %stack_var_001, align 32
  %argument_133 = load i256, ptr %stack_var_000, align 32
  call void @__revert(i256 %argument_032, i256 %argument_133, i256 0)
  unreachable

"block_rt_2/1":                                   ; preds = %conditional_rt_3_join_block
  store i256 0, ptr %stack_var_001, align 32
  store i256 0, ptr %stack_var_002, align 32
  %argument_034 = load i256, ptr %stack_var_002, align 32
  %argument_135 = load i256, ptr %stack_var_001, align 32
  call void @__revert(i256 %argument_034, i256 %argument_135, i256 0)
  unreachable

"block_rt_3/0":                                   ; preds = %conditional_rt_2_join_block
  store i256 4, ptr %stack_var_001, align 32
  store i256 5, ptr %stack_var_002, align 32
  br label %"block_rt_5/0"

"block_rt_4/0":                                   ; preds = %"block_rt_6/0"
  store i256 64, ptr %stack_var_002, align 32
  %argument_036 = load i256, ptr %stack_var_002, align 32
  %memory_load_pointer = inttoptr i256 %argument_036 to ptr addrspace(1)
  %memory_load_result = load i256, ptr addrspace(1) %memory_load_pointer, align 1
  store i256 %memory_load_result, ptr %stack_var_002, align 32
  %dup137 = load i256, ptr %stack_var_002, align 32
  store i256 %dup137, ptr %stack_var_003, align 32
  %dup3 = load i256, ptr %stack_var_001, align 32
  store i256 %dup3, ptr %stack_var_004, align 32
  %dup2 = load i256, ptr %stack_var_003, align 32
  store i256 %dup2, ptr %stack_var_005, align 32
  %argument_038 = load i256, ptr %stack_var_005, align 32
  %argument_139 = load i256, ptr %stack_var_004, align 32
  %memory_store_pointer40 = inttoptr i256 %argument_038 to ptr addrspace(1)
  store i256 %argument_139, ptr addrspace(1) %memory_store_pointer40, align 1
  store i256 32, ptr %stack_var_004, align 32
  %argument_041 = load i256, ptr %stack_var_004, align 32
  %argument_142 = load i256, ptr %stack_var_003, align 32
  %addition_result = add i256 %argument_041, %argument_142
  store i256 %addition_result, ptr %stack_var_003, align 32
  %swap2_top_value = load i256, ptr %stack_var_003, align 32
  %swap2_swap_value = load i256, ptr %stack_var_001, align 32
  store i256 %swap2_swap_value, ptr %stack_var_003, align 32
  store i256 %swap2_top_value, ptr %stack_var_001, align 32
  store i256 64, ptr %stack_var_002, align 32
  %argument_043 = load i256, ptr %stack_var_002, align 32
  %memory_load_pointer44 = inttoptr i256 %argument_043 to ptr addrspace(1)
  %memory_load_result45 = load i256, ptr addrspace(1) %memory_load_pointer44, align 1
  store i256 %memory_load_result45, ptr %stack_var_002, align 32
  %dup146 = load i256, ptr %stack_var_002, align 32
  store i256 %dup146, ptr %stack_var_003, align 32
  %swap2_top_value47 = load i256, ptr %stack_var_003, align 32
  %swap2_swap_value48 = load i256, ptr %stack_var_001, align 32
  store i256 %swap2_swap_value48, ptr %stack_var_003, align 32
  store i256 %swap2_top_value47, ptr %stack_var_001, align 32
  %argument_049 = load i256, ptr %stack_var_003, align 32
  %argument_150 = load i256, ptr %stack_var_002, align 32
  %subtraction_result = sub i256 %argument_049, %argument_150
  store i256 %subtraction_result, ptr %stack_var_002, align 32
  %swap1_top_value = load i256, ptr %stack_var_002, align 32
  %swap1_swap_value = load i256, ptr %stack_var_001, align 32
  store i256 %swap1_swap_value, ptr %stack_var_002, align 32
  store i256 %swap1_top_value, ptr %stack_var_001, align 32
  %argument_051 = load i256, ptr %stack_var_002, align 32
  %argument_152 = load i256, ptr %stack_var_001, align 32
  call void @__return(i256 %argument_051, i256 %argument_152, i256 0)
  unreachable

"block_rt_5/0":                                   ; preds = %"block_rt_3/0"
  store i256 0, ptr %stack_var_002, align 32
  store i256 42, ptr %stack_var_003, align 32
  %swap1_top_value53 = load i256, ptr %stack_var_003, align 32
  %swap1_swap_value54 = load i256, ptr %stack_var_002, align 32
  store i256 %swap1_swap_value54, ptr %stack_var_003, align 32
  store i256 %swap1_top_value53, ptr %stack_var_002, align 32
  %dup155 = load i256, ptr %stack_var_002, align 32
  store i256 %dup155, ptr %stack_var_003, align 32
  br label %"block_rt_6/0"

"block_rt_6/0":                                   ; preds = %"block_rt_5/0"
  %swap1_top_value56 = load i256, ptr %stack_var_002, align 32
  %swap1_swap_value57 = load i256, ptr %stack_var_001, align 32
  store i256 %swap1_swap_value57, ptr %stack_var_002, align 32
  store i256 %swap1_top_value56, ptr %stack_var_001, align 32
  br label %"block_rt_4/0"

conditional_dt_1_join_block:                      ; preds = %"block_dt_0/0"
  store i256 0, ptr %stack_var_001, align 32
  store i256 0, ptr %stack_var_002, align 32
  %argument_02 = load i256, ptr %stack_var_002, align 32
  %argument_13 = load i256, ptr %stack_var_001, align 32
  call void @__revert(i256 %argument_02, i256 %argument_13, i256 0)
  unreachable

conditional_rt_1_join_block:                      ; preds = %"block_rt_0/0"
  store i256 0, ptr %stack_var_001, align 32
  store i256 0, ptr %stack_var_002, align 32
  %argument_017 = load i256, ptr %stack_var_002, align 32
  %argument_118 = load i256, ptr %stack_var_001, align 32
  call void @__revert(i256 %argument_017, i256 %argument_118, i256 0)
  unreachable

conditional_rt_2_join_block:                      ; preds = %"block_rt_1/0"
  store i256 0, ptr %stack_var_000, align 32
  %argument_023 = load i256, ptr %stack_var_000, align 32
  %calldata_pointer24 = load ptr addrspace(3), ptr @ptr_calldata, align 32
  %calldata_pointer_with_offset = getelementptr i8, ptr addrspace(3) %calldata_pointer24, i256 %argument_023
  %calldata_value = load i256, ptr addrspace(3) %calldata_pointer_with_offset, align 32
  store i256 %calldata_value, ptr %stack_var_000, align 32
  store i256 224, ptr %stack_var_001, align 32
  %argument_025 = load i256, ptr %stack_var_001, align 32
  %argument_126 = load i256, ptr %stack_var_000, align 32
  %shr_call = call i256 @__shr(i256 %argument_025, i256 %argument_126)
  store i256 %shr_call, ptr %stack_var_000, align 32
  %dup127 = load i256, ptr %stack_var_000, align 32
  store i256 %dup127, ptr %stack_var_001, align 32
  store i256 3758009808, ptr %stack_var_002, align 32
  %argument_028 = load i256, ptr %stack_var_002, align 32
  %argument_129 = load i256, ptr %stack_var_001, align 32
  %comparison_result30 = icmp eq i256 %argument_028, %argument_129
  %comparison_result_extended31 = zext i1 %comparison_result30 to i256
  store i256 %comparison_result_extended31, ptr %stack_var_001, align 32
  store i256 3, ptr %stack_var_002, align 32
  %conditional_rt_3_condition = load i256, ptr %stack_var_001, align 32
  %conditional_rt_3_condition_compared = icmp ne i256 %conditional_rt_3_condition, 0
  br i1 %conditional_rt_3_condition_compared, label %"block_rt_3/0", label %conditional_rt_3_join_block

conditional_rt_3_join_block:                      ; preds = %conditional_rt_2_join_block
  store i256 2, ptr %stack_var_001, align 32
  br label %"block_rt_2/1"
}

attributes #0 = { cold noreturn nounwind }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #3 = { nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #4 = { nounwind }
attributes #5 = { nounwind willreturn memory(none) }
attributes #6 = { nomerge nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #7 = { nofree null_pointer_is_valid }
```



### Optimized LLVM IR

The redundancy is optimized by LLVM, resulting in the optimized LLVM IR below.

```llvm
; Function Attrs: nofree noreturn null_pointer_is_valid
define i256 @__entry(ptr addrspace(3) %0, i256 %1, i256 %2, i256 %3, i256 %4, i256 %5, i256 %6, i256 %7, i256 %8, i256 %9, i256 %10, i256 %11) local_unnamed_addr #1 personality ptr @__personality {
entry:
  %is_deploy_code_call_flag_truncated = and i256 %1, 1
  %is_deploy_code_call_flag.not = icmp eq i256 %is_deploy_code_call_flag_truncated, 0
  store i256 128, ptr addrspace(1) inttoptr (i256 64 to ptr addrspace(1)), align 64
  %get_u128_value.i.i4 = tail call i256 @llvm.eravm.getu128()
  br i1 %is_deploy_code_call_flag.not, label %runtime_code_call_block, label %deploy_code_call_block

deploy_code_call_block:                           ; preds = %entry
  %comparison_result.i.i = icmp eq i256 %get_u128_value.i.i4, 0
  br i1 %comparison_result.i.i, label %"block_dt_1/0.i.i", label %"block_rt_2/0.i.i"

"block_dt_1/0.i.i":                               ; preds = %deploy_code_call_block
  store i256 32, ptr addrspace(2) inttoptr (i256 256 to ptr addrspace(2)), align 256
  store i256 0, ptr addrspace(2) inttoptr (i256 288 to ptr addrspace(2)), align 32
  tail call void @llvm.eravm.return(i256 53919893334301279589334030174039261352344891250716429051063678533632)
  unreachable

"block_rt_2/0.i.i":                               ; preds = %runtime_code_call_block, %conditional_rt_2_join_block.i.i, %deploy_code_call_block
  tail call void @llvm.eravm.revert(i256 0)
  unreachable

runtime_code_call_block:                          ; preds = %entry
  %abi_pointer_value = ptrtoint ptr addrspace(3) %0 to i256
  %comparison_result.i.i5 = icmp ne i256 %get_u128_value.i.i4, 0
  %12 = and i256 %abi_pointer_value, 340282366604025813406317257057592410112
  %comparison_result21.i.i = icmp eq i256 %12, 0
  %or.cond.i = select i1 %comparison_result.i.i5, i1 true, i1 %comparison_result21.i.i
  br i1 %or.cond.i, label %"block_rt_2/0.i.i", label %conditional_rt_2_join_block.i.i

"block_rt_3/0.i.i":                               ; preds = %conditional_rt_2_join_block.i.i
  store i256 42, ptr addrspace(1) inttoptr (i256 128 to ptr addrspace(1)), align 128
  tail call void @llvm.eravm.return(i256 2535301202817642044428229017600)
  unreachable

conditional_rt_2_join_block.i.i:                  ; preds = %runtime_code_call_block
  %calldata_value.i.i = load i256, ptr addrspace(3) %0, align 32
  %shift_res.i.mask.i.i = and i256 %calldata_value.i.i, -26959946667150639794667015087019630673637144422540572481103610249216
  %comparison_result30.i.i = icmp eq i256 %shift_res.i.mask.i.i, -14476345239007179661737236217584162293203948892620596377535322027150077329408
  br i1 %comparison_result30.i.i, label %"block_rt_3/0.i.i", label %"block_rt_2/0.i.i"
}

; Function Attrs: noreturn nounwind
declare void @llvm.eravm.revert(i256) #2

; Function Attrs: noreturn nounwind
declare void @llvm.eravm.return(i256) #2

attributes #0 = { mustprogress nofree nosync nounwind willreturn memory(none) }
attributes #1 = { nofree noreturn null_pointer_is_valid }
attributes #2 = { noreturn nounwind }
```



### EraVM Assembly

The optimized LLVM IR is translated into EraVM assembly below, allowing the bytecode size
to be comparable with that produced via the Yul pipeline.

```asm
        .text
        .file   "test.sol:Example"
        .globl  __entry
__entry:
.func_begin0:
        add     128, r0, r3
        stm.h   64, r3
        ldvl    r3
        and!    1, r2, r0
        jump.ne @.BB0_1
        sub!    r3, r0, r0
        jump.ne @.BB0_2
        and!    code[@CPI0_1], r1, r0
        jump.eq @.BB0_2
        ldp     r1, r1
        and     code[@CPI0_2], r1, r1
        sub.s!  code[@CPI0_3], r1, r0
        jump.ne @.BB0_2
        add     42, r0, r1
        stm.h   128, r1
        add     code[@CPI0_4], r0, r1
        retl    @DEFAULT_FAR_RETURN
.BB0_1:
        sub!    r3, r0, r0
        jump.ne @.BB0_2
        add     32, r0, r1
        stm.ah  256, r1
        stm.ah  288, r0
        add     code[@CPI0_0], r0, r1
        retl    @DEFAULT_FAR_RETURN
.BB0_2:
        add     r0, r0, r1
        revl    @DEFAULT_FAR_REVERT
.func_end0:

        .rodata
CPI0_0:
        .cell   53919893334301279589334030174039261352344891250716429051063678533632
CPI0_1:
        .cell   340282366604025813406317257057592410112
CPI0_2:
        .cell   -26959946667150639794667015087019630673637144422540572481103610249216
CPI0_3:
        .cell   -14476345239007179661737236217584162293203948892620596377535322027150077329408
CPI0_4:
        .cell   2535301202817642044428229017600
        .text
DEFAULT_UNWIND:
        pncl    @DEFAULT_UNWIND
DEFAULT_FAR_RETURN:
        retl    @DEFAULT_FAR_RETURN
DEFAULT_FAR_REVERT:
        revl    @DEFAULT_FAR_REVERT
```

For comparison, with Yul pipeline of *solc* v0.8.28 the following EraVM assembly is produced:

```asm
        .text
        .file   "test.sol:Example"
        .globl  __entry
__entry:
.func_begin0:
        add     128, r0, r3
        stm.h   64, r3
        and!    1, r2, r0
        jump.ne @.BB0_1
        and!    code[@CPI0_1], r1, r0
        jump.eq @.BB0_7
        ldp     r1, r1
        and     code[@CPI0_2], r1, r1
        sub.s!  code[@CPI0_3], r1, r0
        jump.ne @.BB0_7
        ldvl    r1
        sub!    r1, r0, r0
        jump.ne @.BB0_7
        add     42, r0, r1
        stm.h   128, r1
        add     code[@CPI0_4], r0, r1
        retl    @DEFAULT_FAR_RETURN
.BB0_1:
        ldvl    r1
        sub!    r1, r0, r0
        jump.ne @.BB0_7
        add     32, r0, r1
        stm.ah  256, r1
        stm.ah  288, r0
        add     code[@CPI0_0], r0, r1
        retl    @DEFAULT_FAR_RETURN
.BB0_7:
        add     r0, r0, r1
        revl    @DEFAULT_FAR_REVERT
.func_end0:

        .rodata
CPI0_0:
        .cell   53919893334301279589334030174039261352344891250716429051063678533632
CPI0_1:
        .cell   340282366604025813406317257057592410112
CPI0_2:
        .cell   -26959946667150639794667015087019630673637144422540572481103610249216
CPI0_3:
        .cell   -14476345239007179661737236217584162293203948892620596377535322027150077329408
CPI0_4:
        .cell   2535301202817642044428229017600
        .text
DEFAULT_UNWIND:
        pncl    @DEFAULT_UNWIND
DEFAULT_FAR_RETURN:
        retl    @DEFAULT_FAR_RETURN
DEFAULT_FAR_REVERT:
        revl    @DEFAULT_FAR_REVERT
```
