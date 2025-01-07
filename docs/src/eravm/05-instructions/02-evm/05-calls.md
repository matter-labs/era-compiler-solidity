# Calls

All EVM call instructions follow a similar handling approach. The call type is encoded at the assembly level, so we will focus on the common workflow and note any differences where they arise.

For more information, see the [ZKsync Era documentation](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#call-staticcall-delegatecall).



## CALL

Original [EVM](https://www.evm.codes/#f1?fork=shanghai) instruction.

The code checks whether the call is non-static and whether the Ether value is non-zero. If both conditions are met, the call is redirected to the [MsgValueSimulator](../../03-system-contracts.md#ether-value-simulator).

- [EraVM instruction: `call` (near call)](https://matter-labs.github.io/eravm-spec/spec.html#NearCallDefinition)
- [EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)



## DELEGATECALL

Original [EVM](https://www.evm.codes/#f4?fork=shanghai) instruction.

[EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)



## STATICCALL

Original [EVM](https://www.evm.codes/#fa?fork=shanghai) instruction.

[EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)
