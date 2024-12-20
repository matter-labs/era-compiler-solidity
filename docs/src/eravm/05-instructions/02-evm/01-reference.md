# Native EVM Instructions

Such instructions are grouped into the following categories according to [the original reference](https://www.evm.codes/):

- [Arithmetic](./02-arithmetic.md)
- [Bitwise](./03-bitwise.md)
- [Block](./04-block.md)
- [Call](./05-call.md)
- [Create](./06-create.md)
- [Environment](./07-environment.md)
- [Logging](./08-logging.md)
- [Logical](./09-logical.md)
- [Memory](./10-memory.md)
- [Return](./11-return.md)
- [SHA3](./12-sha3.md)
- [Stack](./13-stack.md)



### EraVM Assembly

Assembly emitted for LLVM standard library functions depends on available optimizations which differ between versions. If there is no
assembly example under the instruction, compile a reproducing contract with the latest version of `zksolc`.

EraVM specification contains a list of [all EraVM instructions (see the table of contents)](https://matter-labs.github.io/eravm-spec/spec.html).
