# ZKsync Era: Solidity Compiler

[![Logo](eraLogo.svg)](https://zksync.io/)

ZKsync Era is a layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security
or decentralization. As it’s EVM-compatible (with Solidity/Vyper), 99% of Ethereum projects can redeploy without
needing to refactor or re-audit any code. ZKsync Era also uses an LLVM-based compiler that will eventually enable
developers to write smart contracts in popular languages such as C++ and Rust.

This repository contains the ZKsync Solidity compiler.

## Installation

To install the *zksolc* compiler, visit the [installation guide](./docs/01-installation.md).

1. **Install via npm**:
   - Use [ZKsync CLI](https://docs.zksync.io/build/tooling/zksync-cli/) to obtain a compiler package and prepare a project environment. After the installation you can modify a hardhat configuration file in the project and specify `zksolc` version there. Use `npx hardhat compile` or `yarn hardhat compile` to compile. [@matterlabs/hardhat-zksync-solc](https://docs.zksync.io/build/tooling/hardhat/getting-started) package will be used from npm repo.
2. **Download prebuilt binaries**:
   - Download [solc](https://github.com/matter-labs/era-solidity/releases) and [zksolc](https://github.com/matter-labs/zksolc-bin) binaries directly from GitHub. Use the CLI or Hardhat to compile contracts.
3. **Build binaries from sources**:
   - Build binaries using the guide below. Use the CLI or Hardhat to compile contracts.

## Unit and e2e/CLI testing

Run `cargo test` from the repository root.

## Troubleshooting

- Unset any LLVM-related environment variables you may have set, especially `LLVM_SYS_<version>_PREFIX` (see e.g. [llvm-sys](https://crates.io/crates/llvm-sys) and [https://llvm.org/docs/GettingStarted.html#local-llvm-configuration](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration)). To make sure: `set | grep LLVM`

## License

The Solidity compiler is distributed under the terms of either

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Resources

- [ZKsync Era compiler toolchain documentation](https://docs.zksync.io/zk-stack/components/compiler/toolchain)
- [Solidity documentation](https://docs.soliditylang.org/en/latest/)

## Official Links

- [Website](https://zksync.io/)
- [GitHub](https://github.com/matter-labs)
- [Twitter](https://twitter.com/zksync)
- [Twitter for Devs](https://twitter.com/ZKsyncDevs)
- [Discord](https://join.zksync.dev/)

## Disclaimer

ZKsync Era has been through extensive testing and audits, and although it is live, it is still in alpha state and
will undergo further audits and bug bounty programs. We would love to hear our community's thoughts and suggestions
about it!
It's important to note that forking it now could potentially lead to missing important
security updates, critical features, and performance improvements.
