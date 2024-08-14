# ZKsync Era: Solidity Compiler

[![Logo](eraLogo.svg)](https://zksync.io/)

ZKsync Era is a layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security
or decentralization. As it’s EVM-compatible (with Solidity/Vyper), 99% of Ethereum projects can redeploy without
needing to refactor or re-audit any code. ZKsync Era also uses an LLVM-based compiler that will eventually enable
developers to write smart contracts in popular languages such as C++ and Rust.

This repository contains the ZKsync Solidity compiler.

## System Requirements

Supported platforms:
- **Linux: x86_64, ARM64**
   * Users are encouraged to adopt GNU libc builds, which offer the same compatibility and are substantially faster.
   * [musl](https://musl.libc.org)-based builds are deprecated, but still supported to preserve tooling compatibility. 
- **MacOS 11+: x86_64, ARM64 (Apple silicon)**
- **Windows: x86_64**
   * Only Windows 10 has been tested so far, but other versions should be OK as well.

We recommend at least 4 GB of RAM available for the build process.

## Delivery Methods

1. **Install via npm**:
   - Use [ZKsync CLI](https://docs.zksync.io/build/tooling/zksync-cli/) to obtain a compiler package and prepare a project environment. After the installation you can modify a hardhat configuration file in the project and specify `zksolc` version there. Use `npx hardhat compile` or `yarn hardhat compile` to compile. [@matterlabs/hardhat-zksync-solc](https://docs.zksync.io/build/tooling/hardhat/getting-started) package will be used from npm repo.
2. **Download prebuilt binaries**:
   - Download [solc](https://github.com/matter-labs/era-solidity/releases) and [zksolc](https://github.com/matter-labs/zksolc-bin) binaries directly from GitHub. Use the CLI or Hardhat to compile contracts.
3. **Build binaries from sources**:
   - Build binaries using the guide below. Use the CLI or Hardhat to compile contracts.

## Building

<details>
<summary>1. Install the system prerequisites.</summary>

   * Linux (Debian):

      Install the following packages:
      ```shell
      apt install cmake ninja-build curl git libssl-dev pkg-config clang lld
      ```

      > Additionally install `musl-tools` if you are building for the `x86_64-unknown-linux-musl` or `aarch64-unknown-linux-musl` targets.
   * Linux (Arch):

      Install the following packages:
      ```shell
      pacman -Syu which cmake ninja curl git pkg-config clang lld
      ```
   * MacOS:

      * Install the [HomeBrew](https://brew.sh) package manager.
      * Install the following packages:

         ```shell
         brew install cmake ninja coreutils
         ```

      * Install your choice of a recent LLVM/[Clang](https://clang.llvm.org) compiler, e.g. via [Xcode](https://developer.apple.com/xcode/), [Apple’s Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html), or your preferred package manager.
</details>

<details>
<summary>2. Install Rust.</summary>

   * Follow the latest [official instructions](https://www.rust-lang.org/tools/install):
      ```shell
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      . ${HOME}/.cargo/env
      ```

      > Currently we are not pinned to any specific version of Rust, so just install the latest stable build for your   platform.

   * If you would like to use `musl` binaries on Linux, install the target for your platform:

      For `x86_64`:
      ```shell
      rustup target add x86_64-unknown-linux-musl
      ```

      For `arm64(aarch64)`:
      ```shell
      rustup target add aarch64-unknown-linux-musl
      ```
</details>

<details>
<summary>3. Download `solc` compiler.</summary>

   [Download a version](https://github.com/ethereum/solc-bin) of [the solc compiler](https://docs.soliditylang.org/en/v0.8.21/) compiler.

   > If it is not named exactly `solc` and in your `$PATH`, see the `--solc` option below.

</details>

<details>
<summary>4. Clone and checkout repository.</summary>

   Use the following commands to clone and checkout the ZKsync Solidity compiler repository:
   ```shell
   git clone https://github.com/matter-labs/era-compiler-solidity.git
   cd era-compiler-solidity
   git checkout <ref>
   ```

   > Replace `<ref>` with the tag, branch, or commit you want to build or skip this step to use default branch of the repository.

</details>

<details>
<summary>5. Install the ZKsync LLVM framework builder.</summary>

   * Install the builder using `cargo`:
      ```shell
      cargo install compiler-llvm-builder
      ```

      > The builder is not the ZKsync LLVM framework itself, but a tool that clones its repository and runs a sequence of build commands. By default it is installed in `~/.cargo/bin/`, which is recommended to be added to your `$PATH`.

</details>

<details>
<summary>6. Build the ZKsync LLVM framework.</summary>

   * Clone and build the ZKsync LLVM framework using the `zksync-llvm` tool:
      ```shell
      zksync-llvm clone
      zksync-llvm build
      ```

      The build artifacts will end up in the `./target-llvm/target-final/` directory.
      You may set the `LLVM_SYS_170_PREFIX` shell variable to the absolute path to that directory to use this build as a compiler dependency.
      If built with the `--enable-tests` option, test tools will be in the `./target-llvm/build-final/` directory, along   with copies of the build artifacts. For all supported build options, run `zksync-llvm build --help`.

      > If you need a specific branch of ZKsync LLVM framework, change it in the `LLVM.lock` file at the root of the repository.

   * If you are building on Linux for distribution  targeting `x86_64-unknown-linux-musl` or `aarch64-unknown-linux-musl`, use the following commands:
      ```shell
      zksync-llvm clone --target-env musl
      zksync-llvm build --target-env musl
      ```

   > You could use `--use-ccache` option to speed up the build process if you have [ccache](https://ccache.dev) installed. For more information and available build options, run `zksync-llvm build --help`.

</details>

<details>
<summary>7. Build the Solidity compiler executable.</summary>


```shell
cargo build --release
```

   * On Linux with musl:

      For `x86_64`:
      ```shell
      cargo build --release --target x86_64-unknown-linux-musl
      ```

      For `ARM64 (aarch64)`:
      ```shell
      cargo build --release --target aarch64-unknown-linux-musl
      ```

      > The resulting binary will be in the `./target/release/zksolc` directory. For `*-musl` targets, the binary will be in the `./target/x86_64-unknown-linux-musl/release/zksolc` or `./target/aarch64-unknown-linux-musl/release/zksolc` directory.

</details>

## Usage

Check `./target/*/zksolc --help` for compiler usage.

The `solc` compiler must be available in `$PATH`, or its path must be passed explicitly with the `--solc` option.

For big projects it is more convenient to use the compiler via the Hardhat plugin. For single-file contracts, or small
projects, the CLI suffices.

## Unit testing

For running unit tests, `zksolc` itself must also be available in `$PATH`, because it calls itself recursively to allow
compiling each contract in a separate process. To successfully run unit tests:

1. Run `cargo build --release`.
2. Move the binary from `./target/release/zksolc` to a directory from `$PATH`, or add the target directory itself to `$PATH`.
3. Run `cargo test`.

## CLI testing

For running command line interface tests, `zksolc` itself and `solc` must also be available in `$PATH`, because it calls itself recursively to allow compiling each contract in a separate processes. To successfully run CLI tests:

1. Go to `cli-tests`.
2. Make `npm i`.
3. Add `solc` and `zksolc` to `$PATH`.
4. Run `npm test`.

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
