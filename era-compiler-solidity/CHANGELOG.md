# The `zksolc` changelog

## [Unreleased]

### Changed

- Migrated to the LLVM-based assembler and linker
- Updated to Rust v1.80.1

## [1.5.2] - 2024-07-31

### Added

- The support for multiple `urls` to local files in standard JSON input

### Removed

- The source code loading mechanism leading to file system errors
- The Solidity source code part of the metadata hash preimage
- Some duplicate fields in the metadata hash preimage

### Fixed

- Errors with imports of files not included into the initial input

## [1.5.1] - 2024-06-27

### Added

- Parallelization in AST and IR parsing
- More LLVM optimizations
- Location resolution for EraVM-specific messages in standard JSON output
- The `suppress-errors` parameter to disable some compilation errors

### Changed

- All messages are now written to JSON output in standard JSON mode
- AST and IRs are not emitted anymore if not explicitly requested
- Empty files and contracts are pruned from standard JSON output
- Message formats are made more compatible with original messages of solc
- `<address payable>.send/transfer(<X>)` now triggers a compilation error
- Updated to Rust v1.79.0

### Removed

- Obsolete warnings for `extcodesize` and `ecrecover`
- EVM-specific warnings which solc has been emitting unconditionally

### Fixed

- Dependency graph inefficiency that caused excessive compilation time
- Removed JSON stream readers which are much slower than strings and vectors
- Missing output with non-canonical input paths in combined JSON output
- Missing warnings with solc v0.4.x and v0.5.x due to differences in AST
- Cryptic error on `type(T).runtimeCode` usage with EVM assembly codegen
- The unknown `bytecodeHash` error in standard JSON mode with solc v0.5.x

## [1.5.0] - 2024-06-10

### Added

- The support for compiling multiple files in Yul, LLVM IR, and EraVM assembly modes
- The support for Yul, LLVM IR, and EraVM assembly languages in standard JSON mode
- The support for `urls` to local files in standard JSON input
- The solc v0.8.26 support
- More LLVM optimizations
- The `--llvm-options` parameter to pass arbitrary options to LLVM
- The `--threads` parameter to control the number of threads
- Caching of the underlying compiler's metadata, including `--version`

### Changed

- Updated to EraVM v1.5.0
- Renamed the `system-mode` flag to `enable-eravm-extensions`
- Renamed the `zkasm` flag to `eravm-assembly`
- Added all missing CLI flags to standard JSON input
- Updated to Rust v1.78.0

### Deprecated

- `force-evmla`, `detect-missing-libraries`, `system-mode` CLI flags in standard JSON mode
- `system-mode` alias of `enable-eravm-extension` flag
- `zkasm` alias of `eravm` flag

### Fixed

- The bytes-to-cells LLVM misoptimization
- LLVM IR generator errors are now written to JSON in standard JSON mode
- Removed `:` from output filenames, as it is not allowed on Windows
- Excessive RAM usage and compilation time with some projects
- Redundancy in error printing

## [1.4.1] - 2024-04-24

### Added

- More LLVM optimizations, including jump tables
- The jump table density threshold optimization parameter
- Simulations to forward return data pointers
- Simulations to manipulate multiple active pointers
- The solc v0.8.25 support
- The support for `useLiteralContent` flag in metadata

### Changed

- Updated to LLVM 17

### Fixed

- The `xor(zext(cmp), -1)` optimization bug
- Libraries passed with `--libraries` and now added to input files
- Printing `--help` if not arguments are provided
- Missing `--overwrite` flag now triggers an error
- Bytecode is now printed to `--output-dir` as a hexadecimal string
- The broken pipe error when piping the output to another process

## [1.4.0] - 2024-02-19

### Added

- The solc v0.8.24 support with temporarily unsupported transient storage and blobs
- The `MCOPY` instruction support
- The `--evm-version` parameter to the CLI
- An option to disable the system request memoization
- More compiler optimizations

### Fixed

- An issue with `MCOPY` for overlapping memory regions

## [1.3.23] - 2024-01-30

### Added

- More LLVM optimizations

## [1.3.22] - 2024-01-12

### Fixed

- Incorrect handling of input paths on Windows
- The issue with different bytecode hash across different platforms

## [1.3.21] - 2024-01-05

### Added

- An option to fallback to optimizing for size if the bytecode is too large

## [1.3.19] - 2023-12-16

### Added

- The support for the EraVM-friendly edition of solc

### Changed

- Disabled the solc optimizer to prevent unnecessary interference

### Fixed

- The incorrect behavior of complex sequences of overflow flags

## [1.3.18] - 2023-12-04

### Added

- More LLVM optimizations, especially for precompiles

### Fixed

- The incorrect detection of constant addresses in call simulations

## [1.3.17] - 2023-11-11

### Added

- The Solidity import remappings support
- The solc v0.8.22 support
- The solc v0.8.23 support
- Simulations to work with constant arrays in code section
- LLVM attributes to Yul function names via `$llvm_<attrs>_llvm$` syntax
- More LLVM optimizations

## [1.3.16] - 2023-10-28

### Added

- The missing EVM legacy assembly fields in standard JSON output
- The LLVM attribute syntax in Yul function identifiers

### Fixed

- The incorrect behavior of complex sequences of modular operations

## [1.3.15] - 2023-10-05

### Added

- More LLVM optimizations

### Changed

- The `INVALID` instruction now burns all gas
- Moved the standard library functions to LLVM

### Removed

- The warnings for `block.*` environment variables

### Fixed

- The crash with exceeded JSON deserialization recursion limit
- The missing `evm.legacyAssembly` field in standard JSON output

## [1.3.14] - 2023-09-06

### Added

- A mode for detection of missing deployable library addresses
- An option to suppress compiler warnings
- The solc v0.8.21 support

## [1.3.13] - 2023-06-29

### Added

- The Yul validation via call to `solc --strict-assembly <path>`
- A warning for `blockhash` usage

### Fixed

- A bug with `CODECOPY` where the bytecode hash was set to 0
- An inefficiency in the Yul lexical analyzer
- A non-deterministic EVMLA output by disabling the `solc` constant optimizer
- Unclear error message for invalid Yul object names
- The CLI argument validation to rule out incompatible options

## [1.3.11] - 2023-05-29

### Added

- The solc v0.8.20 support
- The zkEVM assembly compilation mode (`--zkasm`)

### Changed

- `metadata.bytecodeHash` field in standard JSON is now optional

### Removed

- The potentially dangerous compatible block workaround in EVMLA

### Fixed

- Parsing escape sequences in string and hexadecimal literals
- Some runtime errors with EVMLA from `solc` v0.4
- The bug where the scrutinee of Yul switch was not executed

## [1.3.10] - 2023-04-23

### Fixed

- The evaluation order of Yul function arguments

## [1.3.9] - 2023-04-18

### Added

- A warning that Yul is not validated in system mode
- The `CODESIZE` support in runtime code
- An option not to include the metadata hash at the end of bytecode

### Changed

- Internal function pointers now trigger a compile-time error with the EVMLA pipeline
- Calldata instructions now return 0 in deploy code

### Removed

- Disabled Yul validation via `solc` due to its crashes when attempting to compile to EVM

### Fixed

- The bug with addresses of unresolved libraries replaced with 0
- `CODECOPY` in EVMLA runtime code now zeroes memory out
- The Solidity AST is now passed through without changes
- The LLVM crash with memory offsets `>= 2^64`
- The LLVM crash with ternary operator on fat memory pointers

## [1.3.8] - 2023-04-04

### Added

- Better errors for unsupported `type(X).runtimeCode` with the Yul pipeline
- An option to disable the `solc` optimizer

### Changed

- Increased the stack size for `rayon` workers to 16 MB
- Improved the CLI interface description (see `--help`)

### Fixed

- Another stack overflow issue with the EVMLA pipeline
- `CODECOPY` in runtime code now does not copy calldata with the EVMLA pipeline
- `CODESIZE` in runtime code now returns 0 with the EVMLA pipeline
- Hexadecimal arguments in EVMLA are now parsed as case-insensitive

## [1.3.7] - 2023-03-15

### Added

- LLVM options for debugging and verification
- Fields `metadata`, `devdoc`, `userdoc`, `storage-layout`, `ast` to the combined JSON output

### Removed

- Options `--abi` and `--hashes` due to inefficiency in calling the `solc` subprocess

### Fixed

- The missing `abi` field in the combined JSON output
- The `hashes` field in the combined JSON output is now only printed if requested
- The stack-too-deep error produced by `solc` in some cases of the combined JSON usage
- Invalid behavior of exception handling with the near call ABI
- IRs are not removed from the standard JSON output anymore

## [1.3.6] - 2023-03-09

### Added

- The contract metadata hash to the end of bytecode
- The solc v0.8.19 support
- Source code validation in the Yul mode via a call to `solc`
- Output selection flags `metadata`, `devdoc`, `userdoc`

### Changed

- The optimizer settings to support multiple modes
- The optimizer now optimizes for performance instead of size by default

## [1.3.5] - 2023-02-18

### Added

- The solc v0.8.18 support

## [1.3.4] - 2023-02-17

### Fixed

- The broken optimization flag in the standard JSON mode

## [1.3.3] - 2023-02-16

### Fixed

- The `send` and `transfer` now produce a warning again due to false-positives
- Malfunctioned `CODECOPY` in some cases with the EVMLA pipeline
- The near call exception handling for the requests to system contracts

## [1.3.2] - 2023-02-14

### Added

- The LLVM build commit ID to the `--version` output
- More LLVM optimizations
- Warnings for the instructions `ORIGIN`, `NUMBER`, and `TIMESTAMP`

### Changed

- The `send` and `transfer` methods now produce a compile-time error
- The minimal supported `solc` version to 0.4.12

### Removed

- The `long_version` field from the combined JSON output

### Fixed

- Calls now only copy `min(output_size, return_data_size)` of the return data
- Missing zkEVM warnings in non-standard-JSON outputs

## [1.3.1] - 2023-02-06

### Changed

- Some ABI data layout parameters

## [1.3.0] - 2023-02-02

### Added

- The LLVM IR compilation mode (`--llvm-ir`)

### Changed

- System contract calls now use remaining ergs instead of 0
- The LLVM optimization manager to the new one
- The contract ABI to match that of zkEVM v1.3
- Moved the event decoding to the system contracts
- Simplified the CLI arguments used for debugging

### Removed

- The `extcodesize` check at the beginning of runtime code

### Fixed

- `msg.value >= 2^128` now set the call status code to zero
- `BALANCE` now returns 0 if `address >= 2^160`
- `KECCAK256` now returns an empty error in case of revert
- `SIGNEXTEND` now returns the original value if `bytes >= 31`
- `CODESIZE` is forbidden in Yul runtime code
- `RETURNDATACOPY` now reverts on attempt to copy from beyond the return data
- `RETURN` and `REVERT` offsets and lengths are now clamped to `2^32 - 1`
- Only block hashes of the last 256 blocks are now accessible
- `ptr.pack` is not optimized out by LLVM anymore

## [1.2.3] - 2023-01-18

### Fixed

- The non-zero initial return data size value
- The stack overflow in EVMLA with a try-catch inside an infinite loop

## [1.2.2] - 2022-12-16

### Added

- More LLVM optimizations

### Changed

- Updated LLVM to v15.0.4

### Fixed

- The crash with some uncovered LLVM IR nodes
- The missing check for `msg.value` > `2^128 - 1`
- Some missing fields in the output JSONs

## [1.2.1] - 2022-12-01

### Added

- The option to dump IRs to files
- More contract size optimizations
- The system contracts compilation mode
- The Windows platform support

### Changed

- The `CODECOPY` instruction now produces a compile-time error in the runtime code
- The `CALLCODE` instruction now produces a compile-time error

### Fixed

- The `BYTE` instruction overflow

## [1.2.0] - 2022-10-10

### Added

- Many improvements for the memory security and EVM-compatibility
- Optimizations for the heap allocation
- Support for optimizations for the calldata and returndata forwarding
- More LLVM optimizations
- Support for solc v0.8.17

### Changed

- System contract calls now require a system call flag
- The handling of `msg.value` became more robust
- Failed system contract calls now do bubble-up the reverts

## [1.1.6] - 2022-09-02

### Added

- Better compile-time errors for the Yul mode
- The compiler versions to all output JSONs

### Changed

- Unsupported instructions `PC`, `EXTCODECOPY`, `SELFDESTRUCT` now produce compile-time errors

### Fixed

- Bloating the array of immutables with zero values

## [1.1.5] - 2022-08-16

### Added

- Support for the `BASEFEE` instruction
- Support for solc v0.8.16

## [1.1.4] - 2022-08-08

### Added

- Better compatibility of opcodes `GASLIMIT`, `GASPRICE`, `CHAINID`, `DIFFICULTY`, `COINBASE` etc.

### Fixed

- The check for reserved function names in variable names
- An EVMLA stack inconsistency issue with the `GASPRICE` opcode

## [1.1.3] - 2022-07-16

### Added

- The extcodesize check before the method selector
- The check for the latest supportable version of `solc`
- A lot of LLVM optimizations

### Changed

- The default memory allocator for MUSL to `mimalloc`

### Fixed

- Overwriting the return data size during non-EVM far calls
- The incorrect behavior of immutables in some cases

## [1.1.2] - 2022-07-01

### Changed

- The exponentiation algorithm from linear to binary

## [1.1.1] - 2022-06-24

### Fixed

- The evaluation order of event indexed fields

## [1.1.0] - 2022-06-21

### Added

- Initial release
