# Call

All EVM call instructions are handled similarly.

The call type is encoded on the assembly level, so we will describe the common handling workflow, mentioning distinctions if there are any.

For more information, see the [ZKsync Era documentation](/zksync-protocol/differences/evm-instructions).



## CALL

Original [EVM](https://www.evm.codes/#f1?fork=shanghai) instruction.

The code checks if the call is non-static and the Ether value is non-zero. If so, the call is redirected to the [MsgValueSimulator](/zksync-protocol/compiler/specification/system-contracts#ether-value-simulator).

- [EraVM instruction: `call` (near call)](https://matter-labs.github.io/eravm-spec/spec.html#NearCallDefinition)
- [EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)



## DELEGATECALL

Original [EVM](https://www.evm.codes/#f4?fork=shanghai) instruction.

[EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)



## STATICCALL

Original [EVM](https://www.evm.codes/#fa?fork=shanghai) instruction.

[EraVM instruction: `far_call`](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls)
