# ZKsync Era: Solidity Compiler Toolchain

[![Logo](eraLogo.svg)](https://zksync.io/)

ZKsync Era is a Layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security
or decentralization. As it is EVM-compatible (with Solidity/Vyper), the majority of Ethereum projects can be redeployed without
refactoring and re-auditing. ZKsync Era uses an LLVM-based compiler toolchain that allows developers to design, implement, and optimize
efficient language-specific features while benefiting from the extensive LLVM ecosystem.

This repository contains the ZKsync Compiler Toolchain for Solidity and Yul.

## Installation

In order to install the *zksolc* compiler, visit the [installation guide](./docs/01-installation.md).

For local development, [build zksolc from sources](./docs/01-installation.md#building-from-source).

## Testing

In order to run the unit and CLI tests, run this command from the repository root:

```shell
cargo test
```

## Troubleshooting

During development, it can be tricky to get the `LLVM_SYS_{version}_PREFIX` variable to point to the correct LLVM build if multiple builds are present in the system.
If you suspect that the compiler is not using the correct LLVM build, check `set | grep LLVM` and reset all LLVM-related environment variables.

For the reference, see [llvm-sys](https://crates.io/crates/llvm-sys) and [https://llvm.org/docs/GettingStarted.html#local-llvm-configuration](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration).

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
