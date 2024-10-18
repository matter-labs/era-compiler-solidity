# Installing the ZKsync Solidity Compiler Toolchain


To compile contracts for ZKsync, you need the ZKsync Solidity compiler toolchain.
It consists of two components:

1. The main component is [*zksolc* executable](https://github.com/matter-labs/era-compiler-solidity/releases). 
2. Internally, *zksolc* uses some functionality provided by the Solidity compiler, [*solc*](https://docs.soliditylang.org/en/latest). Note that we are using a [fork](https://github.com/matter-labs/era-solidity) of the original *solc* compiler.


The [fork of the Solidity compiler *solc*](https://github.com/matter-labs/era-solidity/releases) must be installed separately and made visible to *zksolc* in one of two ways:

1. Add the full path to *solc* to the environment variable `PATH`.
2. Alternatively, provide the full path to *solc* through the `--solc` option, for example:

    ```shell
    zksolc --solc './solc' --bin 'Greeter.sol'
    ```

Refer to the section [Installation](#Installation) for more detailed instructions.

## System Requirements

It is recommended to have at least 4 GB of RAM to compile large projects. The
compilation process is parallelized by default, so the number of threads used is
equal to the number of CPU cores.

> [!IMPORTANT]
> Large projects can consume a lot of RAM during compilation on machines with a
> high number of cores. If you encounter memory issues, consider reducing the
> number of threads using the `--threads` option.

The table below outlines the supported platforms and architectures:

| CPU/OS | MacOS | Linux | Windows |
|:------:|:-----:|:-----:|:-------:|
| x86_64 |   ✅   |   ✅   |    ✅    |
| arm64  |   ✅   |   ✅   |    ❌    |

> [!IMPORTANT]
> Please avoid using outdated distributions of operating systems, as they may lack the necessary dependencies or include outdated versions of them.
> *zksolc* is only tested on recent versions of popular distributions, such as MacOS 11.0 and Windows 10.

> [!WARNING]
> [musl](https://musl.libc.org)-based builds are deprecated, but they are still
> supported to preserve tooling compatibility.

## Versioning

The *zksolc* versioning scheme does not yet follow the [Semantic
Versioning](https://semver.org) specification. Instead, its major and minor
versions match those of the EraVM protocol for which *zksolc* produces bytecode.
The patch version is incremented with each release, regardless of whether
breaking changes are introduced. Therefore, please consult the changelog before
updating the compiler.

> [!IMPORTANT]
> We recommend always using the latest version of *zksolc* to benefit from the newest features and bug fixes.

## Installation

You can install the ZKsync Solidity compiler toolchain using the following methods:

1. If you are using Foundry, Hardhat, or other popular toolkits, you can
   let them manage the compiler installation and their dependencies. See [Ethereum Development Toolkits](#ethereum-development-toolkits).
2. You can download pre-built binaries of
   [*solc*](https://github.com/matter-labs/era-solidity/releases) and [*zksolc*](https://github.com/matter-labs/era-compiler-solidity/releases)
   yourself. See [Static Executables](#static-executables).
3. You can build *zksolc* from sources. See [Building from Source](#building-from-source).


For small projects, learning, and research purposes, *zksolc* and *solc*
executables are sufficient without a toolkit.

> [!IMPORTANT]
> We recommend always using the latest version of *zksolc* and *solc* to benefit from the latest features and bug fixes.

Running *zksolc* requires the [fork of the Solidity compiler *solc*](https://github.com/matter-labs/era-solidity/releases). To point *zksolc* to the location of *solc*, use  one of the following methods:

1. Add the location of *solc* to the environment variable `PATH`. 
  
   For example, if you saved *solc* in the directory `/home/username/distrib`,
   you can execute the following command, or append this line to the
   configuration file of your shell:

    ```shell
    export PATH="/home/username/distrib:$PATH"
    ```

2. Alternatively, when you launch *zksolc*, provide the full path to *solc* using the `--solc` option.

   For example, if `solc` is stored in your current working directory, you can 
   point to it like this:

    ```shell
    zksolc --solc './solc' --bin 'Greeter.sol'
    ```



### Ethereum Development Toolkits

For large codebases, it is more convenient to use the ZKsync toolchain via toolkits like Foundry and Hardhat.
These tools manage the compiler executables and their dependencies, and provide additional features like incremental compilation and caching.

The ZKsync toolchain is supported by the following toolkits:

*TODO: Add links to the tutorials*

### Static Executables

We ship *zksolc* binaries on the [releases
page of `matter-labs/era-compiler-solidity` repository](https://github.com/matter-labs/era-compiler-solidity/releases). 
This repository  maintains intuitive and stable naming for the executables and
provides a changelog for each release. Tools using *zksolc* will download the
binaries from this repository and cache them locally.

> [!WARNING]
> The `matter-labs/era-compiler-solidity` repository only contains builds only for versions 1.4.0 and newer.
> You can download older versions from the [main
> branch](https://github.com/matter-labs/zksolc-bin/tree/main) or the [releases
> page of the deprecated repository for zksolc executables](https://github.com/matter-labs/zksolc-bin/releases).
> Please, change download URLs to the [new release location](https://github.com/matter-labs/era-compiler-solidity/releases).

> [!NOTE]
> All binaries are statically linked and should work on all recent platforms.
> *zksolc* is fully written in Rust, aiming to minimize incompatibilities with the environment.

### Building from Source

> [!IMPORTANT]
> Please consider using the pre-built executables before building from source.
> Building from source is only necessary for development, research, and debugging purposes.
> Deployment and production use cases should rely only on [the officially released executables](#static-executables).

1. First, install the necessary system-wide dependencies:

   * For Linux (Debian):

    ```shell
    apt install cmake ninja-build curl git libssl-dev pkg-config clang lld
    ```

   * For Linux (Arch):

    ```shell
    pacman -Syu which cmake ninja curl git pkg-config clang lld
    ```

   * For MacOS:

     1. Install the *Homebrew* package manager by following the instructions at [brew.sh](https://brew.sh).
     2. Install the necessary system-wide dependencies:

        ```shell
        brew install cmake ninja coreutils
        ```

     3. Install a recent build of the LLVM/[Clang](https://clang.llvm.org) compiler using one of the following tools:
        * [Xcode](https://developer.apple.com/xcode/)
        * [Apple’s Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html)
        * Your preferred package manager.

2. Install Rust. Follow the latest [official instructions](https://www.rust-lang.org/tools/install).

> [!TIP]
> The Rust version used for building is pinned in the file [rust-toolchain.toml](../rust-toolchain.toml) at the root of the repository.
> *cargo* will automatically download the pinned version of *rustc* when you start building the project.

3. Clone and checkout this repository:

   ```shell
   git clone https://github.com/matter-labs/era-compiler-solidity
   ```
    
4. Install the ZKsync LLVM framework builder. This tool clones the
   [repository of ZKsync LLVM
   Framework](https://github.com/matter-labs/era-compiler-llvm) and runs a
   sequence of build commands.

    ```shell
    cargo install compiler-llvm-builder
    ```

> [!IMPORTANT]
> - Always use the latest version of the builder to benefit from the latest features and bug fixes.
>   To check for new versions and update the builder, simply run `cargo install compiler-llvm-builder`
>   again, even if you have already installed the builder.
> - The builder is not the ZKsync LLVM framework itself, but a tool to build it.
>   By default, it is installed in `~/.cargo/bin/`, which is usually added to
>   your `PATH` during the Rust installation process.

5. Clone and build the ZKsync LLVM framework using the `zksync-llvm` tool:
  
   ```shell
   # Navigate to the root of your local copy of this repository 
   cd era-compiler-solidity
   # Clone ZKsync LLVM framework. The branch is selected in the file `LLVM.lock`.
   zksync-llvm clone
   # Build LLVM framework
   zksync-llvm build
   ```
  
   You can also clone and build LLVM framework outside of the repository root.
   In this case:
   
   - Provide an `LLVM.lock` file in the directory where you run `zksync-llvm`.
     See the [default LLVM.lock for an example](../LLVM.lock).
   - Ensure that `LLVM.lock` selects the correct branch of the [ZKsync LLVM Framework repository](https://github.com/matter-labs/era-compiler-llvm).
   - Before proceeding to the next step, set the environment variable
     `LLVM_SYS_{version}_PREFIX` to the path of the directory with the LLVM
     build artifacts. Typically, it ends with `target-llvm/build-final`. For
     example:

     ```shell
     export LLVM_SYS_170_PREFIX=~/repositories/era-llvm/target-llvm/build-final 
     ```

> [!TIP]
> Use the `--use-ccache` option to speed up the build process if you have [ccache](https://ccache.dev) installed.
> For more information and available build options, run `zksync-llvm build --help`.

6. Build the *zksolc* executable.

    ```shell
    cargo build --release
    ```
   
    The *zksolc* executable will appear at `./target/release/zksolc`, where you
    can run it directly or move it to another location.

    If *cargo* can't find the LLVM build artifacts, return to the previous step and ensure that the `LLVM_SYS_{version}_PREFIX` environment variable is set to the absolute path of the directory `build-final`.
