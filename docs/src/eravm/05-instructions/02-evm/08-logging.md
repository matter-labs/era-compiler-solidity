# Logging



## Events

The EraVM event instructions operate at a lower level. Each `LOG`-like instruction is expanded into a loop, with each iteration writing two 256-bit words in the following order:

1. The initializer cell, which describes the number of indexed words (e.g. `I`) and the size of non-indexed data in bytes (e.g. `D`).
2. `I` indexed 32-byte words.
3. `D` bytes of data.

If only one word remains to be written, the second input is zero.

For a detailed reference, see [EraVM instruction: `log.event`](https://matter-labs.github.io/eravm-spec/spec.html#EventDefinition)



## LOG0 - LOG4

[LOG0](https://www.evm.codes/#a0?fork=shanghai) - [LOG4](https://www.evm.codes/#a4?fork=shanghai)



### System Contract

This information is requested a System Contract called [EventWriter](https://github.com/code-423n4/2024-03-zksync/blob/main/code/system-contracts/contracts/EventWriter.yul).

On how the contract is called, see [the relevant section](https://docs.zksync.io/zksync-protocol/contracts/system-contracts).
