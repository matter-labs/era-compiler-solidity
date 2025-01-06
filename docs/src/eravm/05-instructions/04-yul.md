# Yul

These instructions do not map directly to EVM or EraVM but instead perform auxiliary operations necessary for generating the target bytecode.



## datasize

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, on EraVM this instruction returns only the size of the header part of the calldata sent to the [ContractDeployer](https://docs.zksync.io/zksync-protocol/compiler/specification/system-contracts#contract-deployer).

For more information, see [CREATE](https://docs.zksync.io/zksync-protocol/compiler/specification/instructions/evm/create).



## dataoffset

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, this instruction does not relate to offsets. Instead, it returns the bytecode hash of the contract referenced by the Yul object identifier.

For more information, see [CREATE](https://docs.zksync.io/zksync-protocol/compiler/specification/instructions/evm/create).



## datacopy

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, on EraVM this instruction copies the bytecode hash passed as [dataoffset](#dataoffset) to the destination. Because our compiler translates instructions without analyzing the surrounding context, there is no other way to obtain the bytecode hash within [datacopy](#datacopy).

For more information, see [CREATE](https://docs.zksync.io/zksync-protocol/compiler/specification/instructions/evm/create).



## setimmutable

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#setimmutable-loadimmutable) auxiliary instruction.

Writes immutables to the auxiliary heap.

For more information, see the [Differences with Ethereum](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## loadimmutable

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#setimmutable-loadimmutable) auxiliary instruction.

Reads immutables from the [ImmutableSimulator](https://docs.zksync.io/zksync-protocol/compiler/specification/system-contracts#simulator-of-immutables) in runtime code, or from temporary values on auxiliary heap in deploy code.

For more information, see the
[Differences with Ethereum](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## linkersymbol

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#linkersymbol) auxiliary instruction.

Sets the placeholder of a deployable library. The address must be passed to `zksolc` with the `--libraries` option,
either in [compiler](../../02-command-line-interface.md#--libraries) or [linker](../../05-linker.md) mode.



## memoryguard

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#memoryguard) auxiliary instruction.

It is a Yul optimizer hint ignored by **zksolc**.



## verbatim

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#verbatim) auxiliary instruction.

Unlike on EVM, on EraVM this instruction has nothing to do with insertions of EVM bytecode. Instead, it is used to implement [ZKsync EraVM Yul Extensions](https://matter-labs.github.io/era-compiler-solidity/latest/06-eravm-extensions.html). In order to compile a Yul contract with extensions, both [Yul mode](../../02-command-line-interface.md#--yul) and [EraVM extensions](../../02-command-line-interface.md#--enable-eravm-extensions) must be enabled.
