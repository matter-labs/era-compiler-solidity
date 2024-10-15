# Installing the ZKsync Solidity Compiler Toolchain

To compile contracts for ZKsync, you need the ZKsync compiler toolchain. The main component of this toolchain is [the zksolc executable](https://github.com/matter-labs/era-compiler-solidity/releases). However, this executable is not a complete Solidity compiler. It operates on artifacts provided by [the underlying solc compiler](https://docs.soliditylang.org/en/latest), that must be made visible to *zksolc*.

## System Requirements

It is recommended to have at least 4 GB of RAM to compile large projects. The compilation process is parallelized by default, so the number of threads is equal to the number of CPU cores.

> [!NOTE]
> Large projects can consume a lot of RAM during compilation on machines with high number of cores.
> If you encounter memory issues, consider reducing the number of `--threads`.

The table below describes the supported platforms and architectures:

| CPU/OS | MacOS | Linux | Windows |
|:------:|:-----:|:-----:|:-------:|
| x86_64 |   ✅   |   ✅   |    ✅    |
| arm64  |   ✅   |   ✅   |    ❌    |

> [!WARNING]
> [musl](https://musl.libc.org)-based builds are deprecated, but still supported to preserve tooling compatibility.

## Versioning

The *zksolc* versioning scheme does not follow the [Semantic Versioning](https://semver.org) specification yet. Instead, its major and minor version match those of the EraVM protocol *zksolc* produces bytecode for. The patch version is incremented with every release, regardless of the introduction of breaking changes, so please track the changelog before updating the compiler.

> [!TIP]
> We recommend to always use the latest version of *zksolc* to benefit from the latest features and bug fixes.

## Installing the *solc* compiler

You can download *solc* from [the Solidity releases](https://github.com/ethereum/solc-bin), or install it another way following [the Solidity installation instructions](https://docs.soliditylang.org/en/latest/installing-solidity.html).

When *solc* is downloaded, add it to `${PATH}` or pass its full path to *zksolc*:

```shell
zksolc --solc './solc' --bin 'Greeter.sol'
```

> [!TIP]
> We recommend to always use the latest version of *solc* to benefit from the latest features and bug fixes.

## Ethereum Development Toolkits

For large codebases it is more convenient to use the ZKsync toolchain via toolkits like Foundry and Hardhat.
These tools manage the compiler executables and their dependencies, and provide additional features like incremental compilation and caching.

The ZKsync toolchain is supported by the following toolkits:

*TODO: Add links to the tutorials*

> [!TIP]
> For small projects, learning, and research purposes the standalone *zksolc* executable is sufficient.

## Static Executables

We ship *zksolc* binaries at [the releases page](https://github.com/matter-labs/era-compiler-solidity/releases). The repository maintains intuitive and stable naming for the executables, and provides a changelog for each release. Tools using *zksolc* shall download the binaries from the repository and cache them locally.

> [!WARNING]
> This repository only contains builds of versions 1.4.0 and later.
> Older versions can be obtained from [the main branch](https://github.com/matter-labs/zksolc-bin/tree/main) or [releases](https://github.com/matter-labs/zksolc-bin/releases) of [the deprecated repository for zksolc executables](https://github.com/matter-labs/zksolc-bin).
> We encourage everyone to change download URLs to [the new release location](https://github.com/matter-labs/era-compiler-solidity/releases).

> [!NOTE]
> All binaries are statically linked and must work on all recent platforms.
> *zksolc* is fully written in Rust, with the aim at minimizing incompatibilities with the environment.

## Building from Source

### Install the system prerequisites

**Linux (Debian)**:
```shell
apt install cmake ninja-build curl git libssl-dev pkg-config clang lld
```

**Linux (Arch)**:
```shell
pacman -Syu which cmake ninja curl git pkg-config clang lld
```

**MacOS (brew)**:
```shell
brew install cmake ninja coreutils
```

For MacOS you also need to install a recent LLVM/[Clang](https://clang.llvm.org) compiler, e.g. via [Xcode](https://developer.apple.com/xcode/), [Apple’s Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html), or your preferred package manager.

### Install Rust

Follow the latest [official instructions](https://www.rust-lang.org/tools/install).

> [!TIP]
> The Rust version used for building is pinned at the root of the repository in [rust-toolchain.toml](../rust-toolchain.toml).
> *cargo* will automatically download the pinned version of *rustc* when you start building the project.

### Clone and checkout this repository

```shell
git clone https://github.com/matter-labs/era-compiler-solidity
cd era-compiler-solidity
```

### Install the ZKsync LLVM framework builder

```shell
cargo install compiler-llvm-builder
```

> [!IMPORTANT]
> The builder is not the ZKsync LLVM framework itself, but a tool that clones its repository and runs a sequence of build commands. By default it is installed in `~/.cargo/bin/`, which is usually added to your `${PATH}` in the process of Rust installation.

### Build the ZKsync LLVM framework

Clone and build the ZKsync LLVM framework using the `zksync-llvm` tool:
```shell
zksync-llvm clone
zksync-llvm build
```

> [!IMPORTANT]
> The LLVM branch is pinned in the `LLVM.lock` file at the repository root.
> If you need a specific branch of ZKsync LLVM framework, change it before proceeding to the next steps.

> [!TIP]
> Use `--use-ccache` option to speed up the build process if you have [ccache](https://ccache.dev) installed.
> For more information and available build options, run `zksync-llvm build --help`.

### Build the zksolc executable

```shell
cargo build --release
```

> [!TIP]
> The built executable will appear at `./target/release/zksolc` directory, where you can run it directly or move to another place.
