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
zksolc './Simple.sol' --bin --solc '/path/to/solc'
```

> Examples in the subsequent sections assume that *solc* [is installed and available](./01-installation.md#installing-solc) in the system path.
> If you prefer specifying the full path to *solc*, use the `--solc` option with the examples below.



### `--bin`

Enables the output of compiled bytecode. The following command compiles a Solidity file and prints the bytecode:

```bash
zksolc './Simple.sol' --bin
```

Output:

```text
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```

It is possible to dry-run the compilation without writing any output. To do this, simply omit `--bin` and other output options:

```bash
zksolc './Simple.sol'
```

Output:

```text
Compiler run successful. No output requested. Use flags --metadata, --asm, --bin.
```



### Input Files

*zksolc* supports multiple input files. The following command compiles two Solidity files and prints the bytecode:

```bash
zksolc './Simple.sol' './Complex.sol' --bin
```

[Solidity import remappings](https://docs.soliditylang.org/en/latest/path-resolution.html#import-remapping) are passed in the way as input files, but they are distinguished by a `=` symbol between source and destination. The following command compiles a Solidity file with a remapping and prints the bytecode:

```bash
zksolc './Simple.sol' 'github.com/ethereum/dapp-bin/=/usr/local/lib/dapp-bin/' --bin
```

*zksolc* does not handle remappings itself, but only passes them through to *solc*.
Visit [the *solc* documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#base-path-and-import-remapping) to learn more about the processing of remappings.



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
zksolc './Simple.sol' --asm --bin
```



### `--metadata`

Enables the output of contract metadata. The metadata is a JSON object that contains information about the contract, such as its name, source code hash, the list of dependencies, compiler versions, and so on.

The *zksolc* metadata format is compatible with the [Solidity metadata format](https://soliditylang.org/docs/develop/metadata.html). This means that the metadata output can be used with other tools that support Solidity metadata. Essentially, *solc* metadata is a part of *zksolc* metadata, and it is included as `source_metadata` without any modifications.

Usage:

```bash
zksolc './Simple.sol' --metadata
```

Output:

```text
======= Simple.sol:Simple =======
Metadata:
{"llvm_options":[],"optimizer_settings":{"is_debug_logging_enabled":false,"is_fallback_to_size_enabled":false,"is_verify_each_enabled":false,"level_back_end":"Aggressive","level_middle_end":"Aggressive","level_middle_end_size":"Zero"},"solc_version":"<masked>","solc_zkvm_edition":null,"source_metadata":{...},"zk_version":"<masked>"}
```



## Other I/O Modes



### `--standard-json`

For the standard JSON mode usage, see the [Standard JSON](./03-standard-json.md) page.



### `--combined-json`

For the combined JSON mode usage, see the [Combined JSON](./04-combined-json.md) page.



## Compilation Settings



### `--evm-version`

Specifies the EVM version *solc* will produce artifacts for. Only artifacts such as Yul and EVM assembly are known to be affected by this option. For instance, if the EVM version is set to *cancun*, then Yul and EVM assembly may contain `MCOPY` instructions.

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
zksolc './Simple.sol' --bin --evm-version 'cancun'
```

For more information on how *solc* handles EVM versions, see its [EVM version documentation](https://docs.soliditylang.org/en/latest/using-the-compiler.html#setting-the-evm-version-to-target).



### `--metadata-hash`

Specifies the hash function used for contract metadata.

The following values are allowed:

|     Value    |  Size  | Padding | Reference |
|:------------:|:------:|:-------:|:---------:|
| none         |  0 B   | 0-32 B  | 
| keccak256    | 32 B   | 0-32 B  | [SHA-3 Wikipedia Page](https://en.wikipedia.org/wiki/SHA-3)
| ipfs         | 44 B   | 20-52 B | [IPFS Documentation](https://docs.ipfs.tech/)

The default value is `keccak256`.

> EraVM requires its bytecode size to be an odd number of 32-byte words. If the size after appending the hash does not satisfy this requirement, the hash is *prepended* with zeros according to the *Padding* column in the table above.

Usage:

```bash
zksolc './Simple.sol' --bin --metadata-hash 'ipfs'
```



### `--metadata-literal`

Tells *solc* to store referenced sources as literal data in the metadata output.

> This option only affects the contract metadata output produced by *solc*, and does not affect artifacts produced by *zksolc*.

Usage:

```bash
zksolc './Simple.sol' --bin --metadata-literal
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
zksolc --yul './Simple.yul' --bin
```

Output:

```text
======= Simple.yul =======
Binary:
0000000100200190000000060000c13d0000002a01000039000000000010043f...
```

*zksolc* is able to compile Yul without *solc*. However, using *solc* is still recommended as it provides additional validation, diagnostics and better error messages:

```bash
zksolc --yul './Simple.yul' --bin --solc '/path/to/solc'
```

*zksolc* features its own dialect of Yul with extensions for EraVM. If such extensions (TODO) are used, it is not possible to use *solc* for validation.



### `--llvm-ir`

Enables the LLVM IR mode. In this mode, input is expected to be in the LLVM IR language. The output works the same way as with Solidity input.

Unlike *solc*, *zksolc* is an LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Yul in the ZKsync compiler toolchain IR hierarchy, so *solc* is not used for compilation.

Usage:

```bash
zksolc --llvm-ir './Simple.ll' --bin
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
zksolc --eravm-assembly './Simple.zasm' --bin
```

Output:

```text
======= Simple.zasm =======
Binary:
000000000120008c000000070000613d00000020010000390000000000100435...
```



## Integrated Tooling

*zksolc* includes several tools provided by the LLVM framework out of the box, such as disassembler and linker. The following sections describe the usage of these tools.

> The mode-altering CLI options are mutually exclusive. This means that only one of the options below can be enabled at a time:
> - `--standard-json`
> - `--combined-json`
> - `--yul`
> - `--llvm-ir`
> - `--eravm-assembly`
> - `--disassemble`
> - `--link`



### `--disassemble`

Enables the disassembler mode.

*zksolc* includes an LLVM-based disassembler that can be used to disassemble compiled bytecode.

The disassembler input must be files with a hexadecimal string. The disassembler output is a human-readable representation of the bytecode, also known as EraVM assembly.

Usage:

```bash
cat './input.zbin'
```

Output:

```text
0x0000008003000039000000400030043f0000000100200190000000140000c13d00000000020...
```

```bash
zksolc --disassemble './input.zbin'
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



### `--link`

Enables the linker mode.

For the linker usage, visit [the linker documentation](./05-linker.md).