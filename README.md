# ZKsync Era: Solidity Compiler

[![Logo](eraLogo.svg)](https://zksync.io/)

ZKsync Era is a Layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security or decentralization. As it is EVM-compatible (with Solidity/Vyper), the majority of Ethereum projects can be redeployed without refactoring and re-auditing. ZKsync Era uses an LLVM-based compiler toolchain that allows developers to design, implement, and optimize efficient language-specific features while benefiting from the extensive LLVM ecosystem.

This repository contains the ZKsync Compiler for Solidity and Yul.

## Installation

To install the *zksolc* compiler, follow our [installation guide](https://matter-labs.github.io/era-compiler-solidity/latest/01-installation.html).

For local development, [build zksolc from its source code](https://matter-labs.github.io/era-compiler-solidity/latest/01-installation.html#building-from-source).

## Usage

For the detailed usage guide, see the [comprehensive documentation](https://matter-labs.github.io/era-compiler-solidity/latest/).

Alternatively, you may check out its Markdown representation in [this repository](./docs/src/).

## Testing

To run the unit and CLI tests, execute `cargo test` from the repository root.

## Documentation

Documentation is using [mdBook](https://github.com/rust-lang/mdBook) utility and its sources available in the `docs/` directory.
To build the documentation, follow the [instructions](./docs/README.md).

## Troubleshooting

If you have multiple LLVM builds in your system, ensure that you choose the correct one to build the compiler.
The environment variable `LLVM_SYS_170_PREFIX` sets the path to the directory with LLVM build artifacts, which typically ends with `target-llvm/build-final`.
For example:

```shell
export LLVM_SYS_170_PREFIX=~/repositories/era-llvm/target-llvm/build-final 
```

If you suspect that the compiler is not using the correct LLVM build, check by running `set | grep LLVM`, and reset all LLVM-related environment variables.

For reference, see [llvm-sys](https://crates.io/crates/llvm-sys) and [Local LLVM Configuration Guide](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration).

## License

The Solidity compiler is distributed under the terms of either

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Resources

- [zksolc documentation](https://matter-labs.github.io/era-compiler-solidity/latest/)
- [ZKsync Era compiler toolchain documentation](https://docs.zksync.io/zk-stack/components/compiler/toolchain)
- [Solidity documentation](https://docs.soliditylang.org/en/latest/)

## Official Links

- [Website](https://zksync.io/)
- [GitHub](https://github.com/matter-labs)
- [Twitter](https://x.com/zksync)
- [Twitter for Devs](https://x.com/ZKsyncDevs)
- [Discord](https://join.zksync.dev/)

## Disclaimer

ZKsync Era has been through extensive testing and audits, and although it is live, it is still in alpha state and
will undergo further audits and bug bounty programs. We would love to hear our community's thoughts and suggestions
about it!
It's important to note that forking it now could potentially lead to missing important
security updates, critical features, and performance improvements.
