# Yul

These instructions do not have a direct representation in EVM or EraVM. Instead, they perform auxiliary operations
required for generating the target bytecode.



## datasize

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, on EraVM this instruction returns the size of the header part of the calldata sent to the
[ContractDeployer](/zksync-protocol/compiler/specification/system-contracts#contract-deployer).
For more information, see [CREATE](/zksync-protocol/compiler/specification/instructions/evm/create).



## dataoffset

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, on EraVM this instruction has nothing to do with the offset. Instead, it returns the bytecode hash
of the contract referenced by the Yul object identifier. Since our compiler translates instructions without analyzing
the surrounding context, it is not possible to get the bytecode hash from anywhere else in [datacopy](#datacopy). For
more information, see [CREATE](/zksync-protocol/compiler/specification/instructions/evm/create).



## datacopy

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#datasize-dataoffset-datacopy) auxiliary instruction.

Unlike on EVM, on EraVM this instruction copies the bytecode hash passed as [dataoffset](#dataoffset) to the
destination. For more information, see [CREATE](/zksync-protocol/compiler/specification/instructions/evm/create).



## setimmutable

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#setimmutable-loadimmutable) auxiliary instruction.

Writes immutables to the auxiliary heap.

For more information, see the [Differences with Ethereum](/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## loadimmutable

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#setimmutable-loadimmutable) auxiliary instruction.

Reads immutables from the [ImmutableSimulator](/zksync-protocol/compiler/specification/system-contracts#simulator-of-immutables).

For more information, see the
[Differences with Ethereum](/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## linkersymbol

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#linkersymbol) auxiliary instruction.

Returns the address of a deployable library. The address must be passed to `zksolc` with the `--libraries` option,
either in [compiler](../../02-command-line-interface.md#--libraries) or [linker](../../05-linker.md) mode.



## memoryguard

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#memoryguard) auxiliary instruction.

Is a Yul optimizer hint which is not used by our compiler. Instead, its only argument is simply unwrapped and returned.



## verbatim

Original [Yul](https://docs.soliditylang.org/en/latest/yul.html#verbatim) auxiliary instruction.

Unlike on EVM, on EraVM this instruction has nothing to do with inserting of EVM bytecode. Instead, it is used to implement
[ZKsync EraVM Yul Extensions](https://matter-labs.github.io/era-compiler-solidity/latest/06-eravm-extensions.html).
In order to compile a Yul contract with extensions, both [Yul mode](../../02-command-line-interface.md#--yul) and [EraVM extensions](../../02-command-line-interface.md#--enable-eravm-extensions) must be enabled.
