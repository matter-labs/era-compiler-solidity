# Installing the ZKsync Solidity Toolchain

To compile contracts for ZKsync, you need the ZKsync compiler toolchain. The main component of this toolchain is [the zksolc executable](https://github.com/matter-labs/era-compiler-solidity/releases). However, this executable is not a complete Solidity compiler. It operates on artifacts provided by [the underlying solc compiler](https://docs.soliditylang.org/en/latest), that must be made visible to *zksolc*.

## System Requirements

Supported platforms:
- **Linux: x86_64, ARM64**
   * Users are encouraged to adopt GNU libc builds, which offer the same compatibility, but are substantially faster.
   * [musl](https://musl.libc.org)-based builds are deprecated, but still supported to preserve tooling compatibility. 
- **MacOS 11+: x86_64, ARM64 (Apple silicon)**
- **Windows: x86_64**
   * Only Windows 10 has been tested so far, but other versions should work without issues.

> [!NOTE]
> Large projects can consume a lot of RAM during compilation on machines with high core counts.
> If you encounter memory issues, consider reducing the number of `--threads`.

## Versioning

The *zksolc* versioning scheme does not follow the [Semantic Versioning](https://semver.org) specification yet. Instead, its major and minor version match those of the EraVM protocol *zksolc* produces bytecode for. The patch version is incremented with every release, regardless of the introduction of breaking changes, so please track the changelog before updating the compiler.

> [!TIP]
> It is recommend to always use the latest version of *zksolc* to benefit from the latest features and bug fixes.

## Installing the *solc* compiler

You can download *solc* from [the Solidity releases](https://github.com/ethereum/solc-bin), or install it another way following [the Solidity installation instructions](https://docs.soliditylang.org/en/latest/installing-solidity.html).

When *solc* is downloaded, add it to `${PATH}` or pass its full path to *zksolc*:

```shell
zksolc --solc '../solc-0.8.26' --bin 'greeter.sol'
```

## Ethereum Development Toolkits

For large codebases it is more convenient to use the ZKsync toolchain via toolkits like Foundry and Hardhat.
These tools manage the compiler executables and their dependencies, and provide additional features like incremental compilation and caching.

The ZKsync toolchain is supported by the following toolkits:

TODO: Add links to the tutorials.

> [!TIP]
> For small projects, learning, and research purposes the standalone *zksolc* executable is sufficient.

## Static *zksolc* Binaries

We ship *zksolc* binaries at [the zksolc repository](https://github.com/matter-labs/era-compiler-solidity/releases). The repository maintains intuitive and stable naming for the executables, and provides a changelog for each release. Tools using *zksolc* shall download the binaries from the repository and cache them locally.

> [!WARNING]
> This repository only contains *zksolc* of version 1.4.0 and later.
> Older versions can be obtained from [the main branch](https://github.com/matter-labs/zksolc-bin/tree/main) or [releases](https://github.com/matter-labs/zksolc-bin/releases) of [the deprecated repository for zksolc executables](https://github.com/matter-labs/zksolc-bin).
> We encourage everyone to change download URLs to [the new release location](https://github.com/matter-labs/era-compiler-solidity/releases).

> [!NOTE]
> All binaries are statically linked and must work on all recent platforms.
> *zksolc* is fully written in Rust, with the aim at minimizing incompatibilities with the environment.

## Building from Source

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
<summary>3. Clone and checkout this repository.</summary>

   Use the following commands to clone and checkout the ZKsync Solidity compiler repository:
   ```shell
   git clone https://github.com/matter-labs/era-compiler-solidity
   cd era-compiler-solidity
   git checkout <ref>
   ```

   > Replace `<ref>` with the tag, branch, or commit you want to build or skip this step to use default branch of the repository.

</details>

<details>
<summary>4. Install the ZKsync LLVM framework builder.</summary>

   * Install the builder using `cargo`:
      ```shell
      cargo install compiler-llvm-builder
      ```

      > The builder is not the ZKsync LLVM framework itself, but a tool that clones its repository and runs a sequence of build commands. By default it is installed in `~/.cargo/bin/`, which is recommended to be added to your `$PATH`.

</details>

<details>
<summary>5. Build the ZKsync LLVM framework.</summary>

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
<summary>6. Build the *zksolc* executable.</summary>

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