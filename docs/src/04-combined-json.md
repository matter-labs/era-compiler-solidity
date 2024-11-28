# Combined JSON

Combined JSON is an I/O mode designed to provide a middle-ground experience between basic CLI and standard JSON. In this mode, input data is provided by the user via CLI, and JSON output can be easily read by both humans and programs calling *zksolc* as a child process.



## Usage

To use combined JSON, pass the `--combined-json` flag to *zksolc* with the desired comma-separated output selectors:

```shell
zksolc './MyContract.sol' --combined-json ast,abi,metadata
```

The following selectors are supported:

|         Selector        |                 Description                 |            Type           |
|:-----------------------:|:-------------------------------------------:|:-------------------------:|
| **ast**                 | AST of the source file                      | JSON                      |
| **abi**                 | Solidity ABI                                | JSON                      |
| **hashes**              | Solidity function hashes                    | JSON                      |
| **storage-layout**      | Solidity storage layout                     | JSON                      |
| **metadata**            | Metadata                                    | Stringified JSON          |
| **devdoc**              | Developer documentation                     | JSON (NatSpec)            |
| **userdoc**             | User documentation                          | JSON (NatSpec)            |

<div class="warning">
It is only possible to use Combined JSON with Solidity input, so the path to <b>solc</b> must be always provided to *zksolc*.
Support for other languages is planned for future releases.
</div>



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
      // Required: Bytecode is always emitted.
      "bin": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Required: Bytecode is always emitted.
      "bin-runtime": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Required, zksolc: Mapping between bytecode hashes and full contract identifiers (e.g. "MyContract.sol:Test").
      "factory-deps": {/* ... */}
      // Required, zksolc: Binary object format.
      // Tells whether the bytecode has been linked.
      // Possible values: "elf" (unlinked), "raw" (linked).
      "objectFormat": "elf"
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
  "version": "0.8.28+commit.acc7d8f9.Darwin.appleclang",
  // Required, zksolc: Version of zksolc.
  "zk_version": "1.5.8"
}
```