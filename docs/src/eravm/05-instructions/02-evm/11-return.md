# Return



## STOP

Original [EVM](https://www.evm.codes/#00?fork=shanghai) instruction.

This instruction is a [RETURN](#return) with an empty data payload.

### LLVM IR

The same as for [RETURN](#return).



## RETURN

Original [EVM](https://www.evm.codes/#f3?fork=shanghai) instruction.

This instruction works differently in deploy code. For more information, see [the ZKsync Era documentation](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#return-stop).

### LLVM IR

```llvm
define void @__return(i256 %0, i256 %1, i256 %2) "noinline-oz" #5 personality i32()* @__personality {
entry:
  %abi = call i256@__aux_pack_abi(i256 %0, i256 %1, i256 %2)
  tail call void @llvm.syncvm.return(i256 %abi)
  unreachable
}
```



## REVERT

Original [EVM](https://www.evm.codes/#fd?fork=shanghai) instruction.

### LLVM IR

```llvm
define void @__revert(i256 %0, i256 %1, i256 %2) "noinline-oz" #5 personality i32()* @__personality {
entry:
  %abi = call i256@__aux_pack_abi(i256 %0, i256 %1, i256 %2)
  tail call void @llvm.syncvm.revert(i256 %abi)
  unreachable
}
```

### EraVM

See also EraVM instruction `revert`: [when returning from near calls](https://matter-labs.github.io/eravm-spec/spec.html#NearRevertDefinition)
and [when returning from far calls](https://matter-labs.github.io/eravm-spec/spec.html#FarRevertDefinition).



## INVALID

Original [EVM](https://www.evm.codes/#fe?fork=shanghai) instruction.

This instruction is a [REVERT](#revert) with an empty data payload, but it also burns all available gas.

### LLVM IR

The same as for [REVERT](#revert).

### EraVM

See also EraVM instruction `revert`: [when returning from near calls](https://matter-labs.github.io/eravm-spec/spec.html#NearRevertDefinition)
and [when returning from far calls](https://matter-labs.github.io/eravm-spec/spec.html#FarRevertDefinition).
