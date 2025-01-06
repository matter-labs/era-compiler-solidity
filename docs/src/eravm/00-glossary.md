# EraVM Target Compilation Specification

This is a technical deep dive into the specifics of compiling for the EraVM target.

The deep dive outlines concepts, modules, and terms used in the EraVM target compilation process, such as:

- [Code Separation](./01-code-separation.md)
- [EVM Assembly Translator](./02-evm-assembly-translator.md)
- [System Contracts](./03-system-contracts.md)
- [Exception Handling](./04-exception-handling.md)

The following sections provide a detailed specification of compilation of each individual instruction:

- [Instructions](./05-instructions/01-reference.md)
- [Extensions](./06-extensions.md)

Finally, this document describes the binary layout of EraVM bytecode:

- [Binary Layout](./07-binary-layout.md)



## Glossary

| Term                      | Definition                                                                                                                            |
|:--------------------------|:--------------------------------------------------------------------------------------------------------------------------------------|
| zksolc                    | Solidity compiler developed by Matter Labs.                                                                                           |
| solc                      | High-level Solidity compiler developed by the Ethereum community. Called by zksolc to get IRs and other auxiliary data.               |
| LLVM                      | The world's most popular and powerful compiler framework, used for optimizations and assembly generation.                             |
| Assembler                 | Tool that translates assembly to bytecode.                                                                                            |
| Linker                    | Tool that links dependencies, such as libraries, before final bytecode can be emitted.                                                |
| Virtual Machine           | ZKsync Era virtual machine with a custom instruction set.                                                                             |
| EraVM Specification       | A combination of human readable documentation and formal description of EraVM, including its structure, semantics, and encoding.      |
| IR                        | Intermediate representation used by the compiler internally to represent source code.                                                 |
| Yul                       | One of two Solidity IRs. A superset of assembly available in Solidity. Used by default for contracts written in Solidity ≥0.8.        |
| EVM Assembly              | One of two Solidity IRs. A predecessor of Yul that is closer to EVM bytecode. Used by default for contracts written in Solidity <0.8. |
| LLVM IR                   | IR native to the LLVM framework.                                                                                                      |
| EraVM Assembly            | Text representation of EraVM bytecode. Emitted by the LLVM framework. Translated into EraVM bytecode by the EraVM assembler.          |
| EraVM Bytecode            | Contract bytecode executed by EraVM.                                                                                                  |
| Stack                     | Segment of non-persistent contract memory. Consists of two parts: global data and function stack frame.                               |
| Heap                      | Segment of non-persistent contract memory. Allocation is handled by the solc’s allocator only.                                        |
| Auxiliary heap            | Segment of non-persistent contract memory. Introduced to avoid conflicts with the solc’s allocator.                                   |
| Calldata                  | Segment of non-persistent contract memory. Heap or auxiliary heap of the parent/caller contract.                                      |
| Return data               | Segment of non-persistent contract memory. Heap or auxiliary heap of the child/callee contract.                                       |
| Storage                   | Persistent contract memory with no important differences from that of EVM.                                                            |
| Transient storage         | Transient contract memory with no important differences from that of EVM.                                                             |
| System contracts          | Set of ZKsync kernel contracts written in Solidity by Matter Labs.                                                                    |
| Contract context          | Storage of the VM keeping data such as current address, caller’s address, block timestamp, etc.                                       |