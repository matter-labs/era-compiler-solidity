# Standard JSON

Standard JSON is a protocol for interaction with the *zksolc* and *solc* compilers. This protocol must be implemented by toolkits such as Hardhat and Foundry.

The protocol uses two data formats for communication: [input JSON](#input-json) and [output JSON](#output-json).



## Usage

Input JSON can be provided by-value via the `--standard-json` option:

```shell
zksolc --standard-json './input.json'
```

Alternatively, the input JSON can be fed to *zksolc* via *stdin*:

```shell
cat './input.json' | zksolc --standard-json
```

After receiving output JSON, the calling program can process it according to its needs. For projects with deployable libraries, [calling the linker](./02-command-line-interface.md#--link) is usually required before compiled contracts are ready for deployment.

> For the sake of interface unification, *zksolc* will always return with exit code 0 and have its standard JSON output printed to *stdout*.
> It differs from *solc* that may return with exit code 1 and a free-formed error in some cases, such as when the standard JSON input file is missing, even though [the *solc* documentation claims otherwise](https://docs.soliditylang.org/en/latest/using-the-compiler.html#compiler-input-and-output-json-description).

The formats below are modifications of the original standard JSON [input](https://docs.soliditylang.org/en/latest/using-the-compiler.html#input-description) and [output](https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description) formats implemented by *solc*. It means that there are:
- *zksolc*-specific options that are not present in the original format: they are marked as `zksolc` in the specifications below.
- *solc*-specific options that are not supported by *zksolc*: they are not mentioned in the specifications below.



## Input JSON

The input JSON provides the compiler with the source code and settings for the compilation. The example below serves as the specification of the input JSON format.

Internally, *zksolc* extracts all *zksolc*-specific options and converts the input JSON to the subset expected by *solc* before calling it.

```javascript
{
  // Required: Source code language.
  // Currently supported: "Solidity", "Yul", "LLVM IR", "EraVM Assembly".
  "language": "Solidity",
  // Required: Source code files to compile.
  // The keys here are the "global" names of the source files. Imports can be using other file paths via remappings.
  "sources": {
    // In source file entry, either but not both "urls" and "content" must be specified.
    "myFile.sol": {
      // Required (unless "content" is used): URL(s) to the source file.
      "urls": [
        // In Solidity mode, directories must be added to the command-line via "--allow-paths <path>" for imports to work.
        // It is possible to specify multiple URLs for a single source file. In this case the first successfully resolved URL will be used.
        "/tmp/path/to/file.sol"
      ],
      // Required (unless "urls" is used): Literal contents of the source file.
      "content": "contract settable is owned { uint256 private x = 0; function set(uint256 _x) public { if (msg.sender == owner) x = _x; } }"
    }
  },

  // Required: Compilation settings.
  "settings": {
    // Optional: Optimizer settings.
    "optimizer": {
      // Optional, zksolc: Set the zksolc LLVM optimizer level.
      // Available options:
      // -0: do not optimize
      // -1: basic optimizations for gas usage
      // -2: advanced optimizations for gas usage
      // -3: all optimizations for gas usage
      // -s: basic optimizations for deployment cost
      // -z: all optimizations for deployment cost
      // Default: 3.
      "mode": "3",
      // Optional, zksolc: Re-run the compilation with "mode": "z" if the compilation with "mode": "3" fails due to EraVM bytecode size limit.
      // Used on a per-contract basis and applied automatically, so some contracts will end up compiled with "mode": "3", and others with "mode": "z".
      // Default: false.
      "sizeFallback": false
    },

    // Optional: Sorted list of remappings.
    // Important: Only used with Solidity input.
    "remappings": [ ":g=/dir" ],
    // Optional: Addresses of the libraries.
    // If not all library addresses are provided here, it will result in unlinked bytecode files that will require post-compile-time linking before deployment.
    // Important: Only used with Solidity, Yul, and LLVM IR input.
    "libraries": {
      // The top level key is the name of the source file where the library is used.
      // If remappings are used, this source file should match the global path after remappings were applied.
      "myFile.sol": {
        // Source code library name and address where it is deployed.
        "MyLib": "0x123123..."
      }
    },

    // Optional: Version of the EVM solc will produce IR for.
    // Affects type checking and code generation.
    // Can be "homestead", "tangerineWhistle", "spuriousDragon", "byzantium", "constantinople", "petersburg", "istanbul", "berlin", "london", "paris", "shanghai", "cancun" or "prague" (experimental).
    // Only used with Solidity, and only affects Yul and EVM assembly codegen. For instance, with version "cancun", solc will produce `MCOPY` instructions, whereas with older EVM versions it will not.
    // Default: chosen by solc, is version-dependent.
    "evmVersion": "cancun",
    // Optional: Select the desired output.
    // Important: zksolc does not support per-file and per-contract selection.
    // Default: no flags are selected, so only bytecode is emitted.
    "outputSelection": {
      "*": {
        // Available file-level options, must be listed under "*"."":
        "": [
          "ast"
        ],
        // Available contract-level options, must be listed under "*"."*":
        "*": [
          // Solidity ABI.
          "abi",
          // Solidity function hashes.
          "evm.methodIdentifiers",
          // Slots, offsets and types of the contract's state variables in storage.
          "storageLayout",
          // Slots, offsets and types of the contract's state variables in transient storage.
          "transientStorageLayout",
          // Developer documentation (natspec).
          "devdoc",
          // User documentation (natspec).
          "userdoc",
          // Metadata.
          "metadata",
          // EVM assembly produced by solc.
          "evm.legacyAssembly",
          // Yul produced by solc.
          "irOptimized",
          // EraVM assembly produced by zksolc.
          "eravm.assembly"
        ]
      }
    },
    // Optional: Metadata settings.
    "metadata": {
      // Optional: Use the given hash method for the metadata hash that is appended to the bytecode.
      // Available options: "none", "ipfs".
      // Default: "ipfs".
      "bytecodeHash": "ipfs",
      // Optional: Use only literal content and not URLs.
      // Passed through to solc and does not affect the zksolc-specific metadata.
      // Default: false.
      "useLiteralContent": false,
      // Optional: Whether to include CBOR-encoded metadata at the end of bytecode.
      // Default: true.
      "appendCBOR": true
    },

    // Optional, zksolc: Solidity codegen.
    // Can be "evmla" or "yul".
    // In contrast to solc, zksolc uses "Yul" codegen by default for solc v0.8.0 and newer. It will be fixed soon, so solc and zksolc defaults become the same.
    // Default: "evmla" for solc <0.8.0, "yul" for solc >=0.8.0.
    "codegen": "yul",
    // Optional, zksolc: Enables the EraVM extensions in Solidity and Yul modes.
    // The extensions include EraVM-specific opcodes and features, such as call forwarding and additional memory spaces.
    // Default: false.
    "enableEraVMExtensions": true,

    // Optional, zksolc: extra LLVM settings.
    "llvmOptions": [
      "-eravm-jump-table-density-threshold", "10",
      "-tail-dup-size", "6",
      "-eravm-enable-split-loop-phi-live-ranges",
      "-tail-merge-only-bbs-without-succ",
      "-join-globalcopies",
      "-disable-early-taildup"
    ],
    // Optional, zksolc: suppressed errors, all listed below.
    "suppressedErrors": [
      "sendtransfer",
      "ripemd160"
    ],
    // Optional, zksolc: suppressed warnings, all listed below.
    "suppressedWarnings": [
      "txorigin",
      "assemblycreate"
    ]
  }
}
```



## Output JSON

The output JSON contains all artifacts produced by both *zksolc* and *solc* compilers. The example below serves as the specification of the output JSON format.

If *solc* is provided to *zksolc*, the output JSON is initially generated by *solc*, and ZKsync-specific data is appended by *zksolc* afterwards. If *solc* is not provided, the output JSON is generated by *zksolc* alone.

```javascript
{
  // Required: File-level outputs.
  "sources": {
    "sourceFile.sol": {
      // Required: Identifier of the source.
      "id": 1,
      // Optional: The AST object.
      // Corresponds to "ast" in the outputSelection settings.
      "ast": {/* ... */}
    }
  },

  // Required: Contract-level outputs.
  "contracts": {
    // The source name.
    "sourceFile.sol": {
      // The contract name.
      // If the language only supports one contract per file, this field equals to the source name.
      "ContractName": {
        // Optional: The Ethereum Contract ABI (object).
        // See https://docs.soliditylang.org/en/develop/abi-spec.html.
        // Corresponds to "abi" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "abi": [/* ... */],
        // Optional: Storage layout (object).
        // Corresponds to "storageLayout" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "storageLayout": {/* ... */},
        // Optional: Transient storage layout (object).
        // Corresponds to "transientStorageLayout" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "transientStorageLayout": {/* ... */},
        // Optional: Developer documentation (natspec object).
        // Corresponds to "devdoc" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "devdoc": {/* ... */},
        // Optional: User documentation (natspec object).
        // Corresponds to "userdoc" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "userdoc": {/* ... */},
        // Optional: Contract metadata (object).
        // Corresponds to "metadata" in the outputSelection settings.
        // Provided by solc and wrapped with additional data by zksolc.
        "metadata": {/* ... */},
        // Optional: Yul produced by solc (string).
        // Corresponds to "irOptimized" in the outputSelection settings.
        // Provided by solc and passed through by zksolc.
        "irOptimized": "/* ... */",
        // Required: EraVM target outputs.
        "eravm": {
          // Required: EraVM bytecode (string).
          "bytecode": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
          // Optional: EraVM assembly produced by zksolc (string).
          // Corresponds to "eravm.assembly" in the outputSelection settings.
          "assembly": "/* ... */"
        },
        // Required: EVM target outputs.
        // Warning: EraVM artifacts "bytecode" and "assembly" are still returned here within the "evm" object for backward compatibility, but all new applications must be reading from the "eravm" object.
        "evm": {
          // Required, Deprecated(EraVM): EVM bytecode.
          "bytecode": {
            // Required: Bytecode (string).
            "object": "0000008003000039000000400030043f0000000100200190000000130000c13d..."
          },
          // Optional: List of function hashes (object).
          // Corresponds to "evm.methodIdentifiers" in the outputSelection settings.
          // Provided by solc and passed through by zksolc.
          "methodIdentifiers": {
            // Mapping between the function signature and its hash.
            "delegate(address)": "5c19a95c"
          },
          // Optional: EVM assembly produced by solc (object).
          // Corresponds to "evm.legacyAssembly" in the outputSelection settings.
          // Provided by solc and passed through by zksolc.
          "legacyAssembly": {/* ... */},

          // Optional, Deprecated: EraVM assembly produced by zksolc (string).
          // Corresponds to "eravm.assembly" in the outputSelection settings.
          "assembly": "/* ... */"
        },

        // Required, zksolc(eravm): Bytecode hash.
        // Used to identify bytecode on ZKsync chains.
        "hash": "5ab89dcf...",
        // Required, zksolc(eravm): All factory dependencies, both linked and unlinked.
        // This field is useful if the full list of dependencies is needed, including those that could not have been linked yet.
        // Example: [ "default.sol:Test" ].
        "factoryDependenciesUnlinked": [/* ... */],
        // Required, zksolc(eravm): Mapping between bytecode hashes and full contract identifiers.
        // Only linked contracts are listed here due to the requirement of bytecode hash.
        // Example: { "5ab89dcf...": "default.sol:Test" }.
        "factoryDependencies": {/* ... */},
        // Required, zksolc(eravm): Mapping between full contract identifiers and library identifiers that must be linked after compilation.
        // Only unlinked libraries are listed here.
        // Example: { "default.sol:Test": "library.sol:Library" }.
        "missingLibraries": {/* ... */},
        // Required, zksolc: Binary object format.
        // Tells whether the bytecode has been linked.
        // Possible values: "elf" (unlinked), "raw" (linked).
        "objectFormat": "elf"
      }
    }
  },

  // Optional: Unset if no messages were emitted.
  "errors": [
    {
      // Optional: Location within the source file.
      // Unset if the error is unrelated to input sources.
      "sourceLocation": {
        /// Required: The source path.
        "file": "sourceFile.sol",
        /// Required: The source location start. Equals -1 if unknown.
        "start": 0,
        /// Required: The source location end. Equals -1 if unknown.
        "end": 100
      },
      // Required: Message type.
      // zksolc only produces "Error" and "Warning" types.
      // *solc* are listed at https://docs.soliditylang.org/en/latest/using-the-compiler.html#error-types.
      "type": "Error",
      // Required: Component the error originates from.
      // zksolc only produces "general".
      // *solc* may produce other values as well.
      "component": "general",
      // Required: Message severity.
      // zksolc only produces "Error" and "Warning" types.
      // *solc* "error", "warning" or "info". May be extended in the future.
      "severity": "error",
      // Optional: Unique code for the cause of the error.
      // Only *solc* produces error codes for now.
      // zksolc error classification is coming soon.
      "errorCode": "3141",
      // Required: Message.
      "message": "Invalid keyword",
      // Required: Message formatted using the source location.
      "formattedMessage": "sourceFile.sol:100: Invalid keyword"
    }
  ],

  // Required: Short semver-compatible solc compiler version.
  "version": "0.8.30",
  // Required: Full solc compiler version.
  "long_version": "0.8.30+commit.7893614a.Darwin.appleclang",
  // Required: Short semver-compatible zksolc compiler version.
  "zk_version": "1.5.14"
}
```
