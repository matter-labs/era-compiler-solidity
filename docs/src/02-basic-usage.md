# Using *zksolc*

The command line interface (CLI) of *zksolc* is designed with resemblance to the CLI of *solc*. There are three main modes in the *zksolc* interface:

1. [Basic CLI](#basic-cli).
2. [Combined JSON](./03-combined-json.md).
3. [Standard JSON](./04-standard-json.md).

The basic CLI and combined JSON modes are more light-weight are suitable for calling from the shell. The standard JSON mode is more similar to client-server interaction, thus more suitable for calling from other applications, such as Foundry.

This page focuses on the basic CLI mode. For more information on the other modes, see the corresponding [combined JSON](./03-combined-json.md) and [standard JSON](./04-standard-json.md) pages.



## Basic CLI

Basic CLI mode is the simplest way to compile a file with the source code. To compile a basic Solidity contract, make sure that [the *solc* compiler](#--solc) is present in your environment and then try the example from the [*--bin*](#--bin) section.

The rest of this section describes the available CLI options and their usage.



### `--solc`

Specifies the path to the *solc* compiler. This option is useful when the *solc* compiler is not available in the system path.

```bash
zksolc Simple.sol --bin --solc '/path/to/solc'
```

> [!IMPORTANT]
> Examples in the subsequent sections assume that *solc* [is installed and available](./01-installation.md#installing-solc) in the system path.
> If you prefer specifying the full path to *solc*, use the `--solc` option with the examples below.



### `--bin`

Enables the output of compiled bytecode. The following command compiles a Solidity file and prints the bytecode:

```bash
zksolc Simple.sol --bin
```

Example output:

```
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```

It is possible to dry-run the compilation without writing any output. To do this, simply omit `--bin` and other output options:

```bash
zksolc Simple.sol
```



### `--asm`

Enables the output of contract assembly. The assembly format depends on the [*--target*](#--target) architecture the contract is compiled for.

EraVM assembly format and layout are described in the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

```bash
zksolc Simple.sol --asm
```

Example output:

```asm
======= Simple.sol:Simple =======
EraVM assembly:
        .text
        .file   "Simple.sol:Simple"
        .globl  __entry
__entry:
.func_begin0:
        add     128, r0, r3
        stm.h   64, r3
...
```

The `--asm` option can be combined with other output options, such as `--bin`:

```bash
zksolc Simple.sol --asm --bin
```



### `--metadata`

Enables the output of contract metadata. The metadata is a JSON object that contains information about the contract, such as its name, source code hash, the list of dependencies, compiler versions, and so on.

```bash
zksolc Simple.sol --metadata
```

Example output:

```json
======= Simple.sol:Simple =======
Metadata:
{"llvm_options":[],"optimizer_settings":{"is_debug_logging_enabled":false,"is_fallback_to_size_enabled":false,"is_verify_each_enabled":false,"level_back_end":"Aggressive","level_middle_end":"Aggressive","level_middle_end_size":"Zero"},"solc_version":"<masked>","solc_zkvm_edition":null,"source_metadata":{"compiler":{"version":"<masked>"},"language":"Solidity","output":{"abi":[{"inputs":[],"name":"first","outputs":[{"internalType":"uint64","name":"","type":"uint64"}],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"second","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"pure","type":"function"}],"devdoc":{"kind":"dev","methods":{},"version":1},"userdoc":{"kind":"user","methods":{},"version":1}},"settings":{"compilationTarget":{"Simple.sol":"Simple"},"evmVersion":"cancun","libraries":{},"metadata":{"bytecodeHash":"ipfs"},"optimizer":{"details":{"constantOptimizer":false,"cse":false,"deduplicate":false,"inliner":false,"jumpdestRemover":false,"orderLiterals":false,"peephole":false,"simpleCounterForLoopUncheckedIncrement":true,"yul":true,"yulDetails":{"optimizerSteps":"dhfoDgvulfnTUtnIfxa[r]EscLMVcul [j]Trpeulxa[r]cLgvifMCTUca[r]LSsTFOtfDnca[r]IulcscCTUtgvifMx[scCTUt] TOntnfDIulgvifMjmul[jul] VcTOcul jmul:fDnTOcmuO","stackAllocation":true}},"runs":200},"remappings":[]},"sources":{"Simple.sol":{"keccak256":"0x1145e81d58e9fd0859036aac4ba16cfcfbe11045e3dfd5105a2dca469f31db89","license":"MIT","urls":["bzz-raw://9d97789b5c14a95fac1e7586de6712119f4606f79d6771324c9d24417ebab0db","dweb:/ipfs/QmSZ3HNGZom6N6eb8d74Y7UQAKAGRkXgbinwVVLaiuGb3S"]}},"version":1},"zk_version":"<masked>"}
```

The *zksolc* metadata format is compatible with the [Solidity metadata format](https://soliditylang.org/docs/develop/metadata.html). This means that the metadata output can be used with other tools that support Solidity metadata. Essentially, *solc* metadata is a part of *zksolc* metadata, and it is included as `source_metadata` without any modifications.



### `--target`

Specifies the target architecture for the compiled contract. The following targets are supported:

- `eravm` (default) — EraVM.
- `evm` — Ethereum Virtual Machine (EVM). Currently under development and only available for testing.

```bash
zksolc Simple.sol --bin --target evm
```

Example output:

```shell
======= Simple.sol:Simple =======
Binary:
5b60806040523415600e575f5ffd5b630000007f630000002760808282823960808092505050f35b5f5f6080604052369150600482106032575f3560e01c9050633df4ddf48114603657635a8ac02d81811480915050605a575b5f5ffd5b3415603f575f5ffd5b60015f036004830313604f575f5ffd5b602a60805260206080f35b34156063575f5ffd5b60015f0360048303136073575f5ffd5b60405160638152602090f3
```

> [!WARNING]
> The `--target` option is experimental and must be passed as a CLI argument in all modes including combined JSON and standard JSON.



### `--yul`

Enables the Yul mode. In this mode, input is expected to be in the Yul language. The output works the same way as with Solidity input.

```bash
zksolc --yul Simple.yul --bin
```

Example output:

```shell
======= Simple.yul =======
Binary:
0000000100200190000000060000c13d0000002a01000039000000000010043f...
```

*zksolc* is able to compile Yul without *solc*. However, using *solc* is still recommended as it provides additional validation, diagnostics and better error messages:

```bash
zksolc --yul Simple.yul --bin --solc '/path/to/solc'
```

*zksolc* features its own dialect of Yul with extensions for EraVM. If such extensions (TODO) are used, it is not possible to use *solc* for validation.



### `--llvm-ir`

Enables the LLVM IR mode. In this mode, input is expected to be in the LLVM IR language. The output works the same way as with Solidity input.

```bash
zksolc --llvm-ir Simple.ll --bin
```

Example output:

```shell
======= Simple.ll =======
Binary:
000000000002004b000000070000613d0000002001000039000000000010043f...
```

Unlike *solc*, *zksolc* is a LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Yul in the ZKsync compiler toolchain IR hierarchy, so *solc* is not used for compilation.



### `--eravm-assembly`

Enables the EraVM Assembly mode. In this mode, input is expected to be in the EraVM assembly language. The output works the same way as with Solidity input.

EraVM assembly format and layout are described in the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

```bash
zksolc --eravm-assembly Simple.zasm --bin
```

Example output:

```shell
======= Simple.zasm =======
Binary:
000000000120008c000000070000613d00000020010000390000000000100435...
```

EraVM assembly is a representation the closest to EraVM bytecode. It is not recommended to write EraVM assembly manually, but it can be even more useful for debugging and optimization purposes than LLVM IR.