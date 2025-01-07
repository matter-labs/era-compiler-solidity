# System Contracts

Many EVM instructions require special handling by the System Contracts. The full detailed list of instructions that require special
handling is provided at [the EVM instructions reference](https://docs.zksync.io/zksync-protocol/differences/evm-instructions).

There are several types of System Contracts from the perspective of how they are handled by **zksolc**:

1. [Environmental data storage](#environmental-data-storage).
2. [KECCAK256 hash function](#keccak256-hash-function).
3. [Contract deployer](#contract-deployer).
4. [Ether value simulator](#ether-value-simulator).
5. [Simulator of immutables](#simulator-of-immutables).
6. [Event handler](#event-handler).



### Environmental Data Storage

Such storage contracts are accessed with static calls in order to retrieve values for the block, transaction, and other
environmental entities: `CHAINID`, `DIFFICULTY`, `BLOCKHASH`, etc.

One good example of such contract is
[SystemContext](https://github.com/matter-labs/era-contracts/blob/main/system-contracts/contracts/SystemContext.sol) that provides
the majority of the environmental data.

Since EVM is not using external calls for these instructions, we must use [the auxiliary heap](#auxiliary-heap) for
their calldata.

Steps to handle such instructions:

1. Store the calldata for the System Contract call on the auxiliary heap.
2. Call the System Contract with a static call.
3. Check the return status code of the call.
4. [Revert or throw](./04-exception-handling.md) if the status code is zero.
5. Read the ABI data and extract the result. All such System Contracts return a single 256-bit value.
6. Return the value as the result of the EVM instruction.



### KECCAK256 Hash Function

Handling of this function is similar to [Environmental Data Storage](#environmental-data-storage) with one key difference: because the EVM also uses heap memory to store the calldata for `KECCAK256`, the IR generator allocates the required memory chunk, so **zksolc** does not need to use [the auxiliary heap](#auxiliary-heap).



### Contract Deployer

See [handling CREATE](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#create-create2)
and [dependency code substitution instructions](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#datasize-dataoffset-datacopy) on ZKsync Era documentation.



### Ether Value Simulator

EraVM does not support passing Ether natively, so this feature is provided by a special System Contract called [MsgValueSimulator](https://github.com/matter-labs/era-contracts/blob/main/system-contracts/contracts/MsgValueSimulator.sol).

An external call is redirected through this simulator if:

1. The [call](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#call-staticcall-delegatecall) is ordinary, that is neither static nor delegate.
2. Its Ether value is non-zero.

Calls to the simulator require additional data passed via ABI using registers:

1. Ether value.
2. The address of the contract to call.
3. The [system call bit](https://matter-labs.github.io/eravm-spec/spec.html#to_system), set only when redirecting a call to the [ContractDeployer](#contract-deployer), that is, when `CREATE` or `CREATE2` is called with non-zero Ether.

To pass Ether in EraVM, the compiler uses:

1. The special 128-bit register [`context_u128`](https://matter-labs.github.io/eravm-spec/spec.html#gs_context_u128)
which is a part of the EraVM [transient state](https://matter-labs.github.io/eravm-spec/spec.html#StateDefinitions).
2. An [immutable value of `context_u128`](https://matter-labs.github.io/eravm-spec/spec.html#ecf_context_u128_value)
captured in the stack frame at the moment of the call.

Details on setting and capturing this value are covered in the [Context Register of the EraVM specification](https://matter-labs.github.io/eravm-spec/spec.html#StateDefinitions).



### Simulator of Immutables

Refer to [the handling immutables documentation](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable) in ZKsync Era.



### Event Handler

Event payloads are sent to a special System Contract called
[EventWriter](https://github.com/matter-labs/era-contracts/blob/main/system-contracts/contracts/EventWriter.yul). As with EVM, the payload consists of topics and data:

1. Topics with a length prefix are passed via ABI using registers.
2. Data is passed through the default heap, just like in EVM.



## Auxiliary Heap

[zksolc](https://matter-labs.github.io/era-compiler-solidity/latest/) works on [the IR level](https://docs.zksync.io/zksync-protocol/compiler/toolchain#ir-compilers). Because of this, they cannot directly manage the heap memory allocator; that responsibility remains with [the high-level source code compilers](https://docs.zksync.io/zksync-protocol/compiler/toolchain#high-level-source-code-compilers) that emit IRs.

However, there are scenarios in which EraVM must allocate memory on the heap while EVM does not, leading to the introduction of the auxiliary heap. The auxiliary heap is used for:

1. [Returning immutables](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#setimmutable-loadimmutable) from the constructor.
2. Allocating calldata and return data for calls to System Contracts.

While the ordinary heap contains calldata and return data for calls to **user contracts**, the auxiliary heap holds calldata and return data for calls to **System Contracts**. This preserves EVM compatibility by preventing System Contract calls from affecting calldata or return data, thereby avoiding conflicts with the heap layout that contract developers expect.

For more details on heaps, refer to the EraVM specification, which describes [types of heaps](https://matter-labs.github.io/eravm-spec/spec.html#data_page_params), their connections to [stack frames and memory growth](https://matter-labs.github.io/eravm-spec/spec.html#ctx_heap_page_id), and their role in [contract-to-contract communication](https://matter-labs.github.io/eravm-spec/spec.html#MemoryForwarding).