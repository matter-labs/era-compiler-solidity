# Combined JSON

Combined JSON is an I/O mode designed to provide a middle-ground experience between basic CLI and standard JSON. In this mode, input data is provided by the user via CLI, and JSON output can be easily read by both humans and programs calling *zksolc* as a child process.



## Usage

To use combined JSON, pass the `--combined-json` flag to *zksolc* with the desired comma-separated output selectors:

```shell
zksolc './MyContract.sol' --combined-json 'ast,abi,metadata'
```

The following selectors are supported:

|         Selector              |                 Description                 |            Type           |   Origin   |
|:------------------------------|:--------------------------------------------|:--------------------------|:-----------|
| **abi**                       | Solidity ABI                                | JSON                      |  **solc**  |
| **hashes**                    | Solidity function hashes                    | JSON                      |  **solc**  |
| **metadata**                  | Metadata                                    | Stringified JSON          |  **solc**  |
| **devdoc**                    | Developer documentation                     | JSON (NatSpec)            |  **solc**  |
| **userdoc**                   | User documentation                          | JSON (NatSpec)            |  **solc**  |
| **storage-layout**            | Solidity storage layout                     | JSON                      |  **solc**  |
| **transient-storage-layout**  | Solidity transient storage layout           | JSON                      |  **solc**  |
| **ast**                       | AST of the source file                      | JSON                      |  **solc**  |
| **asm**                       | EVM assembly                                | JSON                      |  **solc**  |
| **eravm-assembly**            | EraVM assembly                              | String                    | **zksolc** |
| **bin**                       | Deploy bytecode (always enabled)            | Hexadecimal string        | **zksolc** |
| **bin-runtime**               | Runtime bytecode (EVM-only, always enabled) | Hexadecimal string        | **zksolc** |

> **Warning:** It is only possible to use Combined JSON with Solidity input, so the path to **solc** must be always provided to **zksolc**. Support for other languages is planned for future releases.



## Output Format

The format below is a modification of the original combined JSON [output](https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description) format implemented by *solc*. It means that there are:

- *zksolc*-specific options that are not present in the original format: they are marked as *zksolc* in the specification below.
- *solc*-specific options that are not supported by *zksolc*: they are not mentioned in the specification below.

```javascript
{
  // Required: Contract outputs.
  "contracts": {
    "MyContract.sol:Test": {
      // Optional: Emitted if "hashes" selector is provided.
      "hashes": {/* ... */},
      // Optional: Emitted if "abi" selector is provided.
      "abi": [/* ... */],
      // Optional: Emitted if "metadata" selector is provided.
      "metadata": "/* ... */",
      // Optional: Emitted if "devdoc" selector is provided.
      "devdoc": {/* ... */},
      // Optional: Emitted if "userdoc" selector is provided.
      "userdoc": {/* ... */},
      // Optional: Emitted if "storage-layout" selector is provided.
      "storage-layout": {/* ... */},
      // Optional: Emitted if "transient-storage-layout" selector is provided.
      "transient-storage-layout": {/* ... */},
      // Optional: Emitted if "ast" selector is provided.
      "ast": {/* ... */},
      // Optional: Emitted if "asm" selector is provided.
      "asm": {/* ... */},

      // Optional: Emitted if "assembly" selector is provided.
      "assembly": "/* ... */",
      // Required: Bytecode is always emitted.
      "bin": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Required: Bytecode is always emitted.
      "bin-runtime": "0000008003000039000000400030043f0000000100200190000000130000c13d...",

      // Required, zksolc(eravm): All factory dependencies, both linked and unlinked.
      // This field is useful if the full list of dependencies is needed, including those that could not have been linked yet.
      // Example: [ "default.sol:Test" ].
      "factory-deps-unlinked": [/* ... */],
      // Required, zksolc(eravm): Mapping between bytecode hashes and full contract identifiers.
      // Only linked contracts are listed here due to the requirement of bytecode hash.
      // Example: { "5ab89dcf...": "default.sol:Test" }.
      "factory-deps": {/* ... */},
      // Required, zksolc(eravm): Unlinked EraVM libraries.
      // Example: [ "library.sol:Library" ].
      "missing-libraries": [/* ... */],
      // Required, zksolc: Binary object format.
      // Tells whether the bytecode has been linked.
      // Possible values: "elf" (unlinked), "raw" (linked).
      "object-format": "elf"
    }
  },
  // Optional: List of input files.
  // Only emitted if "ast" selector is provided.
  "sourceList": [
    "MyContract.sol"
  ],
  // Optional: List of input sources.
  // Only emitted if "ast" selector is provided.
  "sources": {
    "MyContract.sol": {
      // Required: Contract AST.
      "AST": {/* ... */}
      // Required: Contract index in "sourceList".
      "id": 0
    }
  },
  // Required: Version of solc.
  "version": "0.8.30+commit.acc7d8f9.Darwin.appleclang",
  // Required, zksolc: Version of zksolc.
  "zk_version": "1.5.14"
}
```