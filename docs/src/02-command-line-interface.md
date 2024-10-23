# Command Line Interface (CLI)

The CLI of *zksolc* is designed with resemblance to the CLI of *solc*. There are several main input/output (I/O) modes in the *zksolc* interface:

- [Basic CLI](#basic-cli)
- [Combined JSON](./03-combined-json.md)
- [Standard JSON](./04-standard-json.md)

The basic CLI and combined JSON modes are more light-weight and suitable for calling from the shell. The standard JSON mode is similar to client-server interaction, thus more suitable for using from other applications, such as [Foundry](https://github.com/matter-labs/foundry-zksync).

> [!IMPORTANT]
> All toolkits using *zksolc* must be operating in standard JSON mode and follow [its specification](./04-standard-json.md).
> It will make the toolkits more robust and future-proof, as the standard JSON mode is the most versatile and used for the majority of popular projects.

This page focuses on the basic CLI mode. For more information on the other modes, see the corresponding [combined JSON](./03-combined-json.md) and [standard JSON](./04-standard-json.md) pages.



## Basic CLI

Basic CLI mode is the simplest way to compile a file with the source code.

To compile a basic Solidity contract, make sure that [the *solc* compiler](#--solc) is present in your environment and run the example from [the *--bin* section](#--bin).

The rest of this section describes the available CLI options and their usage.



### `--solc`

Specifies the path to the *solc* compiler. Useful when the *solc* compiler is not available in the system path.

Usage:

```bash
zksolc Simple.sol --bin --solc '/path/to/solc'
```

> [!IMPORTANT]
> Examples in the subsequent sections assume that *solc* [is installed and available](./01-installation.md#installing-solc) in the system path.
> If you prefer specifying the full path to *solc*, use the `--solc` option with the examples below.



### `--bin`

Enables the output of compiled bytecode. The following command compiles a Solidity file and prints the bytecode:

```bash
zksolc './Simple.sol' --bin
```

Output:

```
======= Simple.sol:Simple =======
Binary:
0000008003000039000000400030043f0000000100200190000000130000c13d...
```

It is possible to dry-run the compilation without writing any output. To do this, simply omit `--bin` and other output options:

```bash
zksolc './Simple.sol'
```

Output:

```
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

```
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

The *zksolc* metadata format is compatible with the [Solidity metadata format](https://soliditylang.org/docs/develop/metadata.html). This means that the metadata output can be used with other tools that support Solidity metadata. Essentially, *solc* metadata is a part of *zksolc* metadata, and it is included as `source_metadata` without any modifications.

Usage:

```bash
zksolc Simple.sol --metadata
```

Output:

```
======= Simple.sol:Simple =======
Metadata:
{"llvm_options":[],"optimizer_settings":{"is_debug_logging_enabled":false,"is_fallback_to_size_enabled":false,"is_verify_each_enabled":false,"level_back_end":"Aggressive","level_middle_end":"Aggressive","level_middle_end_size":"Zero"},"solc_version":"<masked>","solc_zkvm_edition":null,"source_metadata":{...},"zk_version":"<masked>"}
```



## Other I/O Modes



### `--combined-json`

For the combined JSON mode usage, see the [Combined JSON](./03-combined-json.md) page.



### `--standard-json`

For the standard JSON mode usage, see the [Standard JSON](./04-standard-json.md) page.



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
zksolc --yul Simple.yul --bin
```

Output:

```
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

Unlike *solc*, *zksolc* is an LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Yul in the ZKsync compiler toolchain IR hierarchy, so *solc* is not used for compilation.

Usage:

```bash
zksolc --llvm-ir Simple.ll --bin
```

Output:

```
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
zksolc --eravm-assembly Simple.zasm --bin
```

Output:

```
======= Simple.zasm =======
Binary:
000000000120008c000000070000613d00000020010000390000000000100435...
```



## Integrated Tooling

*zksolc* includes several tools provided by the LLVM framework out of the box, such as disassembler and linker. The following sections describe the usage of these tools.

> [!NOTE]
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

The disassembler input can be either a binary file or a file with a hexadecimal string. The disassembler output is a human-readable representation of the bytecode, also known as EraVM assembly.

Usage with a hexadecimal string file:

```bash
cat './input.hex'
```

Output:

```
0x0000008003000039000000400030043f0000000100200190000000140000c13d00000000020...
```

> [!NOTE]
> The `0x` prefix in the front of the hexadecimal string is optional.

```bash
zksolc --disassemble './input.hex'
```

Output:

```
File `input.hex` disassembly:

       0: 00 00 00 80 03 00 00 39       add     128, r0, r3
       8: 00 00 00 40 00 30 04 3f       stm.h   64, r3
      10: 00 00 00 01 00 20 01 90       and!    1, r2, r0
      18: 00 00 00 14 00 00 c1 3d       jump.ne 20
      20: 00 00 00 00 02 01 00 19       add     r1, r0, r2
      28: 00 00 00 0b 00 20 01 98       and!    code[11], r2, r0
      30: 00 00 00 23 00 00 61 3d       jump.eq 35
      38: 00 00 00 00 01 01 04 3b       ldp     r1, r1
```

Usage with a binary file:

```bash
zksolc --disassemble input.bin
```



## Multi-Target Support

*zksolc* is an LLVM-based compiler toolchain, so it is easily extensible to support multiple target architectures. The following targets are supported:

- `eravm` — [EraVM](https://docs.zksync.io/zk-stack/components/zksync-evm) (default).
- `evm` — [EVM](https://ethereum.org/en/developers/docs/evm/) (under development and only available for testing).

### `--target`

Specifies the target architecture for the compiled contract.

> [!WARNING]
> The `--target` option is experimental and must be passed as a CLI argument in all modes including combined JSON and standard JSON.

Usage:

```bash
zksolc Simple.sol --bin --target evm
```

Output:

```
======= Simple.sol:Simple =======
Binary:
5b60806040523415600e575f5ffd5b630000007f630000002760808282823960808092505050f35b5f5f6080604052369150600482106032575f3560e01c9050633df4ddf48114603657635a8ac02d81811480915050605a575b5f5ffd5b3415603f575f5ffd5b60015f036004830313604f575f5ffd5b602a60805260206080f35b34156063575f5ffd5b60015f0360048303136073575f5ffd5b60405160638152602090f3
```



### `--link`

Enables the linker mode.

*zksolc* includes an LLVM-based linker that can be used for post-compile-time linking of libraries.

Such linking is happening in several steps:

1. A contract with a library dependency is compiled to bytecode:

```solidity
// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

library GreeterHelper {
    function addPrefix(Greeter greeter, string memory great) public view returns (string memory) {
        return string.concat(greeter._prefix(), great);
    }
}

contract Greeter {
    string public greeting;
    string public _prefix;

    constructor(string memory _greeting) {
        greeting = _greeting;
        _prefix = "The greating is:";
    }

    function greet() public view returns (string memory) {
        return GreeterHelper.addPrefix(this, greeting);
    }
}
```

```bash
zksolc './Greeter.sol' --output-dir './output' --bin
```

For unlinked bytecode, the ZKsync compiler toolchain uses [an ELF wrapper](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) which is the standard of the LLVM framework. ELF-wrapped bytecode is not a valid bytecode that can be deployed to the blockchain. Before deployment, all library references must be resolved. Upon the resolution of the library references, the ELF wrapper is stripped and only the raw bytecode ready for deployment remains. This also means that unlinked and linked bytecodes are not equal in size.

2. Check for unlinked library references.

It can be done with the following command, where the `--library` argument is intentionally omitted:

```bash
zksolc --link './output/Greeter.sol/Greeter.zbin' | jq .
```

Output:

```json
{
  "ignored": {},
  "linked": {},
  "unlinked": {
    "./output/Greeter.sol/Greeter.zbin": [
      "Greeter.sol:GreeterHelper"
    ]
  }
}
```

3. Provide library addresses to the linker.

The library addresses must be provided in the `--libraries` argument:

```bash
zksolc --link './output/Greeter.sol/Greeter.zbin' --libraries 'Greeter.sol:GreaterHelper=0x1234567812345678123456781234567812345678' | jq .
```

Output:

```json
{
  "ignored": {},
  "linked": {
    "./output/Greeter.sol/Greeter.zbin": "010000bd2bcef5602ae1ebc0b812cc65d88655a8d972ac10227f142e1838093c"
  },
  "unlinked": {}
}
```

The `linked` field tells lists all bytecode files where all library references were successfully resolved. The values next to file paths are the hashes of the bytecode that are used to identify EraVM dependencies during deployment.

If `unlinked` is empty, the bytecode in the file `./output/Greeter.sol/Greeter.zbin` is ready for deployment. The bytecode file is modified in-place, so the original file with unlinked bytecode is overwritten.

The `ignored` field list files that have been already linked before, so they are not processed by the linker in the current call. For instance, if you run the command above once again:

```json
{
  "ignored": {
    "./output/Greeter.sol/Greeter.zbin": "010000bd2bcef5602ae1ebc0b812cc65d88655a8d972ac10227f142e1838093c"
  },
  "linked": {},
  "unlinked": {}
}
```
