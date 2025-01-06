# Native EVM Instructions

EVM instructions are grouped into categories based on [the official reference](https://www.evm.codes/):

- [Arithmetic](./02-arithmetic.md)
- [Bitwise](./03-bitwise.md)
- [Block](./04-block.md)
- [Calls](./05-calls.md)
- [Create](./06-create.md)
- [Environment](./07-environment.md)
- [Logging](./08-logging.md)
- [Logical](./09-logical.md)
- [Memory](./10-memory.md)
- [Return](./11-return.md)
- [SHA3](./12-sha3.md)
- [Stack](./13-stack.md)



### EraVM Assembly

The assembly generated for LLVM standard library functions depends on available optimizations, which vary by version. If you do not see an assembly example for a particular instruction, try compiling a reproducing contract using the latest `zksolc`.

For a comprehensive list of instructions, see the [EraVM specification](https://matter-labs.github.io/eravm-spec/spec.html), which provides them in its table of contents.
