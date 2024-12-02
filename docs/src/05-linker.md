# Linker

*zksolc* includes an LLVM-based linker that can be used for post-compile-time library linking.

For unlinked bytecode, the ZKsync compiler toolchain uses [an ELF wrapper](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format), which is the standard in the LLVM framework. ELF-wrapped bytecode cannot be deployed to the blockchain as-is; all library references must first be resolved. Once they are resolved, the ELF wrapper is stripped, leaving only the raw bytecode ready for deployment. This approach also results in unlinked and linked bytecode differing in size.

> When compiling to EraVM, provide all build artifacts to the linker. Unlike EVM ones, EraVM dependencies are linked using the bytecode hash, so if you simply provide all the bytecode files of your project, the linker will automatically resolve all dependencies.

The *zksolc* linker can be used in several ways:

- [JSON Protocol](#json-protocol)
- [Basic CLI](#basic-cli)

## JSON Protocol

This mode is suitable for integration with tooling such as Foundry. The linker features its own JSON protocol with input and output formats which are described in [input](#input) and [output](#output) sections below.

Input JSON can be provided by-value via the `--standard-json` option:

```shell
zksolc --link --standard-json './input.json'
```

Alternatively, the input JSON can be fed to *zksolc* via *stdin*:

```shell
cat './input.json' | zksolc --link --standard-json
```

### Input

```javascript
{
  // Input bytecode files mapping.
  "bytecodes": {
    // Input bytecode must be a valid ELF object.
    "tests/data/bytecodes/linker.zbin": "7f454c46010101ff000000000000000001000401010000000000000000000000..."
  },
  // Library specifiers array.
  "libraries": [
    // The format is following that of solc: "filename:libraryName=address".
    "Greeter.sol:GreeterHelper=0x1234567890abcdef1234567890abcdef12345678"
  ]
}
```

### Output

```javascript
{
  // Bytecode files where all library references have been successfully resolved.
  "linked": {
    "tests/data/bytecodes/linked.zbin": {
      // Linked EraVM bytecode, stripped of the ELF wrapper.
      "bytecode": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Hash of the bytecode used to identify EraVM dependencies during deployment.
      "hash": "010000d5bf4dd6262304eb67a95a76e6e4b0e9f1dc3d2c524c129c6464939407"
    }
  },
  // Lists of unresolved symbols, such as those not provided to the linker.
  // The linker caller must add the missing specifiers and call the linker again.
  "unlinked": {
    "tests/data/bytecodes/linker.zbin": [
      // Unresolved library specifier.
      // The format is following that of solc: "filename:libraryName".
      "Greeter.sol:GreeterHelper"
    ]
  },
  // Linked raw bytecode files that do not require linking, so they were not processed in the current call.
  "ignored": {
    "tests/data/bytecodes/ignored.zbin": {
      // Linked raw EraVM bytecode.
      "bytecode": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Hash of the bytecode used to identify EraVM dependencies during deployment.
      "hash": "010000d5bf4dd6262304eb67a95abcdefc3d2c524c129c6464939407"
    }
  }
}
```

## Basic CLI

This mode is suitable for experiments and quick checks. Linking is done in several steps:

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

2. Check for unlinked library and factory dependency references.

It can be done with the following command, where the `--library` argument is intentionally omitted:

```bash
zksolc --link './output/Greeter.sol/Greeter.zbin'
```

Output:

```json
{
  "linked": {},
  "unlinked": {
    "./output/Greeter.sol/Greeter.zbin": {
      "linker_symbols": ["Greeter.sol:GreeterHelper"],
      "factory_dependencies": []
    }
  },
  "ignored": {}
}
```

3. Provide library addresses to the linker.

The library addresses must be provided in the `--libraries` argument:

```bash
zksolc --link './output/Greeter.sol/Greeter.zbin' --libraries 'Greeter.sol:GreeterHelper=0x1234567812345678123456781234567812345678'
```

Output:

```json
{
  "linked": {
    "./output/Greeter.sol/Greeter.zbin": {
      "bytecode": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      "hash": "010000bd2bcef5602ae1ebc0b812cc65d88655a8d972ac10227f142e1838093c"
    }
  },
  "unlinked": {},
  "ignored": {}
}
```

If you run the last command above once again, nothing will happen, and the previously linked file will show up as `ignored`:

```json
{
  "linked": {},
  "unlinked": {},
  "ignored": {
    "./output/Greeter.sol/Greeter.zbin": {
      "bytecode": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      "hash": "010000bd2bcef5602ae1ebc0b812cc65d88655a8d972ac10227f142e1838093c"
    }
  }
}
```
