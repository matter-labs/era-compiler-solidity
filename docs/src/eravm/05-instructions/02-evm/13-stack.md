# Stack



## POP

Original [EVM](https://www.evm.codes/#50?fork=shanghai) instruction.

In Yul, only used to mark unused values, and is not translated to LLVM IR.

```solidity
pop(staticcall(gas(), address(), 0, 64, 0, 32))
```

For EVMLA, see [EVM Legacy Assembly Translator](/zksync-protocol/compiler/specification/evmla-translator).



## JUMPDEST

Original [EVM](https://www.evm.codes/#5b?fork=shanghai) instruction.

Is not available in Yul.

Ignored in EVMLA. See [EVM Legacy Assembly Translator](/zksync-protocol/compiler/specification/evmla-translator) for more information.



## PUSH - PUSH32

Original [EVM](https://www.evm.codes/#5f?fork=shanghai) instructions.

Is not available in Yul.

For EVMLA, see [EVM Legacy Assembly Translator](/zksync-protocol/compiler/specification/evmla-translator).



## DUP1 - DUP16

Original [EVM](https://www.evm.codes/#80?fork=shanghai) instructions.

Is not available in Yul.

For EVMLA, see [EVM Legacy Assembly Translator](/zksync-protocol/compiler/specification/evmla-translator).



## SWAP1 - SWAP16

Original [EVM](https://www.evm.codes/#90?fork=shanghai) instructions.

Is not available in Yul.

For EVMLA, see [EVM Legacy Assembly Translator](/zksync-protocol/compiler/specification/evmla-translator).
