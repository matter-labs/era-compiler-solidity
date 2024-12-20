# EVM Assembly

These instructions do not have a direct representation in EVM or EraVM instruction sets. Instead, they perform auxiliary operations
required for generating the target bytecode.



## PUSH [$]

The same as [datasize](./04-yul.md#datasize).



## PUSH #[$]

The same as [dataoffset](./04-yul.md#dataoffset).



## ASSIGNIMMUTABLE

The same as [setimmutable](./04-yul.md#setimmutable).

For more information, see [differences with Ethereum](/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## PUSHIMMUTABLE

The same as [loadimmutable](./04-yul.md#loadimmutable).

For more information, see [differences with Ethereum](/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable).



## PUSHLIB

The same as [linkersymbol](./04-yul.md#linkersymbol).



## PUSHDEPLOYADDRESS

Returns the address the contract is deployed to.



## PUSHSIZE

Can be only found in deploy code. On EVM, returns the total size of the runtime code and constructor arguments.

On EraVM, it is always 0, since EraVM does not operate on runtime code in deploy code.



## PUSH data

Pushes a data chunk onto the stack. Data chunks are resolved during the processing of input assembly JSON.



## PUSH [tag]

Pushes an EVM Legacy Assembly destination block identifier onto the stack.



## Tag

Starts a new EVM Legacy Assembly block. Tags are processed during the translation of EVM Legacy Assembly into EthIR.
