# Command Line Interface (CLI)

The CLI of *zksolc* is designed with resemblance to the CLI of *solc*. There are several main input/output (I/O) modes in the *zksolc* interface:

- [Basic CLI](#basic-cli)
- [Standard JSON](./03-standard-json.md)
- [Combined JSON](./04-combined-json.md)

The basic CLI and combined JSON modes are more light-weight and suitable for calling from the shell. The standard JSON mode is similar to client-server interaction, thus more suitable for using from other applications, such as [Foundry](https://github.com/matter-labs/foundry-zksync).

> All toolkits using *zksolc* must be operating in standard JSON mode and follow [its specification](./03-standard-json.md).
> It will make the toolkits more robust and future-proof, as the standard JSON mode is the most versatile and used for the majority of popular projects.

This page focuses on the basic CLI mode. For more information on the other modes, see the corresponding [combined JSON](./04-combined-json.md) and [standard JSON](./03-standard-json.md) pages.



## Basic CLI

Basic CLI mode is the simplest way to compile a file with the source code.

To compile a basic Solidity contract, make sure that [the *solc* compiler](#--solc) is present in your environment and run the example from [the *--bin* section](#--bin).

The rest of this section describes the available CLI options and their usage. You may also check out `zksolc --help` for a quick reference.



### `--solc`

Specifies the path to the *solc* compiler. Useful when the *solc* compiler is not available in the system path.

Usage:

```bash
zksolc 'Simple.sol' --bin --solc '/path/to/solc'
```

> Examples in the subsequent sections assume that *solc* [is installed and available](./01-installation.md#installing-solc) in the system path.
> If you prefer specifying the full path to *solc*, use the `--solc` option with the examples below.



### `--bin`

Enables the output of compiled bytecode. The following command compiles a Solidity file and prints the bytecode:

```bash
zksolc 'Simple.sol' --bin
```

Output:

```text
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```

It is possible to dry-run the compilation without writing any output. To do this, simply omit `--bin` and other output options:

```bash
zksolc 'Simple.sol'
```

Output:

```text
Compiler run successful. No output requested. Use flags --metadata, --asm, --bin.
```



### Input Files

*zksolc* supports multiple input files. The following command compiles two Solidity files and prints the bytecode:

```bash
zksolc 'Simple.sol' 'Complex.sol' --bin
```

[Solidity import remappings](https://docs.soliditylang.org/en/latest/path-resolution.html#import-remapping) are passed in the way as input files, but they are distinguished by a `=` symbol between source and destination. The following command compiles a Solidity file with a remapping and prints the bytecode:

```bash
zksolc 'Simple.sol' 'github.com/ethereum/dapp-bin/=/usr/local/lib/dapp-bin/' --bin
```

*zksolc* does not handle remappings itself, but only passes them through to *solc*.
Visit [the *solc* documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#base-path-and-import-remapping) to learn more about the processing of remappings.



### `--libraries`

Specifies the libraries to link with compiled contracts. The option accepts multiple string arguments. The safest way is to wrap each argument in single quotes, and separate them with a space.

The specifier has the following format: `<ContractPath>:<ContractName>=<LibraryAddress>`.

Usage:

```bash
zksolc 'Simple.sol' --bin --libraries 'Simple.sol:Test=0x1234567890abcdef1234567890abcdef12345678'
```

There are two ways of linking libraries:
1. At compile time, immediately after the contract is compiled.
2. At deploy time (a.k.a. post-compile time), right before the contract is deployed.

The use case above describes linking at compile time. For linking at deploy time, see the [linker documentation](./05-linker.md).



### `--base-path`, `--include-path`, `--allow-paths`

These options are used to specify Solidity import resolution settings. They are not used by *zksolc* and only passed through to *solc* like import remappings.

Visit [the *solc* documentation](https://docs.soliditylang.org/en/latest/path-resolution.html) to learn more about the processing of these options.



### `--asm`

Enables the output of contract assembly. The assembly format depends on the [*--target*](#--target) architecture the contract is compiled for.

For the EraVM assembly specification, visit the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

EVM assembly is not supported yet.

Usage:

```bash
zksolc Simple.sol --asm
```

Output:

```text
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
zksolc 'Simple.sol' --asm --bin
```



### `--metadata`

Enables the output of contract metadata. The metadata is a JSON object that contains information about the contract, such as its name, source code hash, the list of dependencies, compiler versions, and so on.

The *zksolc* metadata format is compatible with the [Solidity metadata format](https://docs.soliditylang.org/en/latest/metadata.html#contract-metadata). This means that the metadata output can be used with other tools that support Solidity metadata. Essentially, *solc* metadata is a part of *zksolc* metadata, and it is included as `source_metadata` without any modifications.

Usage:

```bash
zksolc 'Simple.sol' --metadata
```

Output:

```text
======= Simple.sol:Simple =======
Metadata:
{"llvm_options":[],"optimizer_settings":{"is_debug_logging_enabled":false,"is_fallback_to_size_enabled":false,"is_verify_each_enabled":false,"level_back_end":"Aggressive","level_middle_end":"Aggressive","level_middle_end_size":"Zero"},"solc_version":"x.y.z","solc_zkvm_edition":null,"source_metadata":{...},"zk_version":"x.y.z"}
```



### `--output-dir`

Specifies the output directory for build artifacts. Can only be used in [basic CLI](#basic-cli) and [combined JSON](./04-combined-json.md) modes.

Usage in basic CLI mode:

```bash
zksolc 'Simple.sol' --bin --asm --metadata --output-dir './build/'
ls './build/Simple.sol'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
Test.zasm       Test.zbin       Test_meta.json
```

Usage in combined JSON mode:

```bash
zksolc 'Simple.sol' --combined-json 'bin,asm,metadata' --output-dir './build/'
ls './build/'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
combined.json
```



### `--overwrite`

Overwrites the output files if they already exist in the output directory. By default, *zksolc* does not overwrite existing files.

Can only be used in combination with the [`--output-dir`](#--output-dir) option.

Usage:

```bash
zksolc 'Simple.sol' --combined-json 'bin,asm,metadata' --output-dir './build/' --overwrite
```

If the `--overwrite` option is not specified and the output files already exist, *zksolc* will print an error message and exit:

```text
Error: Refusing to overwrite an existing file "build/combined.json" (use --overwrite to force).
```



### `--version`

Prints the version of *zksolc* and the hash of the LLVM commit it was built with.

Usage:

```bash
zksolc --version
```



### `--help`

Prints the help message.

Usage:

```bash
zksolc --help
```



## Other I/O Modes

> The mode-altering CLI options are mutually exclusive. This means that only one of the options below can be enabled at a time:
> - `--standard-json`
> - `--combined-json`
> - `--yul`
> - `--llvm-ir`
> - `--eravm-assembly`
> - `--disassemble`
> - `--link`



### `--standard-json`

For the standard JSON mode usage, see the [Standard JSON](./03-standard-json.md) page.



### `--combined-json`

For the combined JSON mode usage, see the [Combined JSON](./04-combined-json.md) page.



## *zksolc* Compilation Settings

The options in this section are only configuring the *zksolc* compiler and do not affect the underlying *solc* compiler.



### `--optimization / -O`

Sets the optimization level of the LLVM optimizer. Available values are:

| Level | Meaning                      | Hints                                            |
|:------|:-----------------------------|:-------------------------------------------------|
| 0     | No optimization              | Best compilation speed: for active development
| 1     | Performance: basic           | For optimization research
| 2     | Performance: default         | For optimization research
| 3     | Performance: aggressive      | Default value. Best performance: for production
| s     | Size: default                | For optimization research
| z     | Size: aggressive             | Best size: for contracts with size constraints

For most cases, it is fine to use the default value of `3`. You should only use the level `z` if you are ready to deliberately sacrifice performance and optimize for size.

> Large contracts may hit the EraVM or EVM bytecode size limit. In this case, it is recommended to use the [`--fallback-Oz`](#--fallback-oz) option rather than set the `z` level.



### `--fallback-Oz`

Sets the optimization level to `z` for contracts that failed to compile due to overrunning the bytecode size constraints.

Under the hood, this option automatically triggers recompilation of contracts with level `z`. Contracts that were successfully compiled with [the original `--optimization` setting](#--optimization---o) are not recompiled.

> It is recommended to have this option always enabled to prevent compilation failures due to bytecode size constraints. There are no known downsides to using this option.



### `--metadata-hash`

Specifies the hash function used for project metadata appended to the end of bytecode.

The following values are allowed: `none`, `ipfs`.

The default value is `ipfs`.

> EraVM requires its bytecode size to be an odd number of 32-byte words. If the size after appending the hash does not satisfy this requirement, the metadata is *prepended* with zeros.

Usage:

```bash
zksolc 'Simple.sol' --bin --metadata-hash 'ipfs'
```

Output:

```text
======= Simple.sol:Simple =======
Binary:
00000001002001900000000c0000613d0000008001000039000000400010043f0000000001000416000000000001004b0000000c0000c13d00000020010000390000010000100443000001200000044300000005010000
...
a2646970667358221220ba14ea4e52366f139a845913d41e98933393bd1c1126331611687003d4aa92de64736f6c6378247a6b736f6c633a312e352e31333b736f6c633a302e382e32393b6c6c766d3a312e302e310055
```

The byte array starting with `a2` at the end of the bytecode is a CBOR-encoded compiler version data and an optional metadata hash.

JSON representation of a CBOR payload:

```javascript
{
    // Optional: included if `--metadata-hash` is set to `ipfs`.
    "ipfs": "1220ba14ea4e52366f139a845913d41e98933393bd1c1126331611687003d4aa92de",

    // Required: consists of semicolon-separated pairs of colon-separated compiler names and versions.
    // `zksolc:<version>` is always included.
    // `solc:<version>;llvm:<version>` is only included for Solidity contracts and Yul contracts if solc is used for validation,
    // but not included for LLVM IR and EraVM assembly contracts.
    // `llvm` stands for the revision of ZKsync fork of solc. It is not possible to use the upstream build of solc with zksolc anymore.
    "solc": "zksolc:1.5.13;solc:0.8.29;llvm:1.0.2"
}
```

For more information on these formats, see the [CBOR](https://cbor.io/) and [IPFS](https://docs.ipfs.tech/) documentation.



### `--no-cbor-metadata`

Disables the CBOR metadata that is appended at the end of bytecode. This option is useful for debugging and research purposes.

> It is not recommended to use this option in production, as it is not possible to verify contracts deployed without metadata.

Usage:

```shell
zksolc 'Simple.sol' --no-cbor-metadata
```



### `--enable-eravm-extensions`

Enables the EraVM extensions.

If this flag is set, calls to addresses `0xFFFF` and below are substituted by special EraVM instructions.

In Yul mode, the `verbatim_*` instruction family becomes available.

The full list of EraVM extensions and their usage can be found [here](./06-eravm-extensions.md).

Usage:

```bash
zksolc 'Simple.sol' --bin --enable-eravm-extensions
```



### `--suppress-errors`

Tells the compiler to suppress specified errors. The option accepts multiple string arguments, so make sure they are properly separated by whitespace.

Errors that can be suppressed:

- [`sendtransfer`](https://docs.zksync.io/build/developer-reference/best-practices#use-call-over-send-or-transfer)
- [`ripemd160`](https://docs.zksync.io/zksync-protocol/differences/pre-compiles#available-precompiles)

Usage:

```bash
zksolc 'Simple.sol' --bin --suppress-errors 'sendtransfer'
```



### `--suppress-warnings`

Tells the compiler to suppress specified warnings. The option accepts multiple string arguments, so make sure they are properly separated by whitespace.

Warnings that can be suppressed:

- [`txorigin`](https://docs.zksync.io/zksync-era/tooling/foundry/migration-guide/testing#origin-address)
- [`assemblycreate`](https://docs.zksync.io/zksync-protocol/differences/evm-instructions#create-create2)

Usage:

```bash
zksolc 'Simple.sol' --bin --suppress-warnings 'txorigin'
```



### `--llvm-options`

Specifies additional options for the LLVM framework. The argument must be a single quoted string following a `=` separator.

Usage:

```bash
zksolc 'Simple.sol' --bin --llvm-options='-eravm-jump-table-density-threshold=10'
```

> The `--llvm-options` option is experimental and must only be used by experienced users. All supported options will be documented in the future.



## *solc* Compilation Settings

The options in this section are only configuring *solc*, so they are passed directly to its child process, and do not affect the *zksolc* compiler.



### `--codegen`

Specifies the *solc* codegen. The following values are allowed:

| Value | Description                  | Defaults                           |
|:------|:-----------------------------|:-----------------------------------|
| evmla | EVM legacy assembly          | *solc* default for EVM/L1          |
| yul   | Yul a.k.a. IR                | *zksolc* default for ZKsync        |

> *solc* uses the `evmla` codegen by default. However, *zksolc* uses the `yul` codegen by default for historical reasons.
> Codegens are not equivalent and may lead to different behavior in production.
> Make sure that this option is set to `evmla` if you want your contracts to behave as they would on L1.
> For codegen differences, visit the [solc IR breaking changes page](https://docs.soliditylang.org/en/latest/ir-breaking-changes.html).
> *zksolc* is going to switch to the `evmla` codegen by default in the future in order to have more parity with L1.

Usage:

```bash
zksolc 'Simple.sol' --bin --codegen 'evmla'
```



### `--evm-version`

Specifies the EVM version *solc* will produce artifacts for. Only artifacts such as Yul and EVM assembly are known to be affected by this option. For instance, if the EVM version is set to *cancun*, then Yul and EVM assembly may contain `MCOPY` instructions, so no calls to the Identity precompile (address `0x04`) will be made.

> EVM version only affects IR artifacts produced by *solc* and does not affect EraVM bytecode produced by *zksolc*.

The default value is chosen by *solc*. For instance, *solc* v0.8.24 and older use *shanghai* by default, whereas newer ones use *cancun*.

The following values are allowed, however have in mind that newer EVM versions are only supported by newer versions of *solc*:
- homestead
- tangerineWhistle
- spuriousDragon
- byzantium
- constantinople
- petersburg
- istanbul
- berlin
- london
- paris
- shanghai
- cancun
- prague

Usage:

```bash
zksolc 'Simple.sol' --bin --evm-version 'cancun'
```

For more information on how *solc* handles EVM versions, see its [EVM version documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#setting-the-evm-version-to-target).



### `--metadata-literal`

Tells *solc* to store referenced sources as literal data in the metadata output.

> This option only affects the contract metadata output produced by *solc*, and does not affect artifacts produced by *zksolc*.

Usage:

```bash
zksolc 'Simple.sol' --bin --metadata-literal
```



## Multi-Language Support

*zksolc* supports input in multiple programming languages:

- [Solidity](https://soliditylang.org/)
- [Yul](https://docs.soliditylang.org/en/latest/yul.html)
- [LLVM IR](https://llvm.org/docs/LangRef.html)
- [EraVM assembly](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout)

The following sections outline how to use *zksolc* with these languages.



### `--yul`

Enables the Yul mode. In this mode, input is expected to be in the Yul language. The output works the same way as with Solidity input.

Usage:

```bash
zksolc --yul 'Simple.yul' --bin
```

Output:

```text
======= Simple.yul =======
Binary:
0000000100200190000000060000c13d0000002a01000039000000000010043f...
```

*zksolc* is able to compile Yul without *solc*. However, using *solc* is still recommended as it provides additional validation, diagnostic and better error messages:

```bash
zksolc --yul 'Simple.yul' --bin --solc '/path/to/solc'
```

*zksolc* features its own dialect of Yul with extensions for EraVM. If [the extensions](./06-eravm-extensions.md) are enabled, it is not possible to use *solc* for validation.



### `--llvm-ir`

Enables the LLVM IR mode. In this mode, input is expected to be in the LLVM IR language. The output works the same way as with Solidity input.

Unlike *solc*, *zksolc* is an LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Yul in the ZKsync compiler toolchain IR hierarchy, so *solc* is not used for compilation.

Usage:

```bash
zksolc --llvm-ir 'Simple.ll' --bin
```

Output:

```text
======= Simple.ll =======
Binary:
000000000002004b000000070000613d0000002001000039000000000010043f...
```



### `--eravm-assembly`

Enables the EraVM Assembly mode. In this mode, input is expected to be in the EraVM assembly language. The output works the same way as with Solidity input.

EraVM assembly is a representation the closest to EraVM bytecode. It is not recommended to write EraVM assembly manually, but it can be even more useful for debugging and optimization purposes than LLVM IR.

For the EraVM assembly specification, visit the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

Usage:

```bash
zksolc --eravm-assembly 'Simple.zasm' --bin
```

Output:

```text
======= Simple.zasm =======
Binary:
000000000120008c000000070000613d00000020010000390000000000100435...
```



## Multi-Target Support

*zksolc* is an LLVM-based compiler toolchain, so it is easily extensible to support multiple target architectures. The following targets are supported:

- `eravm` — [EraVM](https://docs.zksync.io/zk-stack/components/zksync-evm) (default).
- `evm` — [EVM](https://ethereum.org/en/developers/docs/evm/) (under development and only available for testing).

### `--target`

Specifies the target architecture for the compiled contract.

<div class="warning">
The <code>--target</code> option is experimental and must be passed as a CLI argument in all modes including combined JSON and standard JSON.
</div>

Usage:

```bash
zksolc Simple.sol --bin --target evm
```

Output:

```text
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```



## Integrated Tooling

*zksolc* includes several tools provided by the LLVM framework out of the box, such as disassembler and linker. The following sections describe the usage of these tools.



### `--disassemble`

Enables the disassembler mode.

*zksolc* includes an LLVM-based disassembler that can be used to disassemble compiled bytecode.

The disassembler input must be files with a hexadecimal string. The disassembler output is a human-readable representation of the bytecode, also known as EraVM assembly.

Usage:

```bash
cat 'input.zbin'
```

Output:

```text
0x0000008003000039000000400030043f0000000100200190000000140000c13d00000000020...
```

```bash
zksolc --disassemble 'input.zbin'
```

Output:

```text
File `input.zbin` disassembly:

       0: 00 00 00 80 03 00 00 39       add     128, r0, r3
       8: 00 00 00 40 00 30 04 3f       stm.h   64, r3
      10: 00 00 00 01 00 20 01 90       and!    1, r2, r0
      18: 00 00 00 14 00 00 c1 3d       jump.ne 20
      20: 00 00 00 00 02 01 00 19       add     r1, r0, r2
      28: 00 00 00 0b 00 20 01 98       and!    code[11], r2, r0
      30: 00 00 00 23 00 00 61 3d       jump.eq 35
      38: 00 00 00 00 01 01 04 3b       ldp     r1, r1
```



### `--link`

Enables the linker mode.

For the linker usage, visit [the linker documentation](./05-linker.md).



## Debugging



### `--debug-output-dir`

Specifies the directory to store intermediate build artifacts. The artifacts can be useful for debugging and research.

The directory is created if it does not exist. If artifacts are already present in the directory, they are overwritten.

The intermediate build artifacts can be:

| Name            | Codegen         | File extension   |
|:----------------|:----------------|:-----------------|
| EVM Assembly    | evmla           | *evmla*          |
| EthIR           | evmla           | *ethir*          |  
| Yul             | yul             | *yul*            |
| LLVM IR         | evmla, yul      | *ll*             |
| EraVM Assembly  | evmla, yul      | *zasm*           |

Usage:

```bash
zksolc 'Simple.sol' --bin --debug-output-dir './debug/'
ls './debug/'
```

Output:

```text
Compiler run successful. No output requested. Use flags --metadata, --asm, --bin.
...
Simple.sol.C.runtime.optimized.ll
Simple.sol.C.runtime.unoptimized.ll
Simple.sol.C.yul
Simple.sol.C.zasm
Simple.sol.Test.runtime.optimized.ll
Simple.sol.Test.runtime.unoptimized.ll
Simple.sol.Test.yul
Simple.sol.Test.zasm
```

The output file name is constructed as follows: `<ContractPath>.<ContractName>.<Modifiers>.<Extension>`.



### `--llvm-verify-each`

Enables the verification of the LLVM IR after each optimization pass. This option is useful for debugging and research purposes.

Usage:

```bash
zksolc 'Simple.sol' --bin --llvm-verify-each
```



### `--llvm-debug-logging`

Enables the debug logging of the LLVM IR optimization passes. This option is useful for debugging and research purposes.

Usage:

```bash
zksolc 'Simple.sol' --bin --llvm-debug-logging
```
