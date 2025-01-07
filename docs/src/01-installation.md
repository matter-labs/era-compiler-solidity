# Installing the ZKsync Solidity Compiler Toolchain

To compile contracts for ZKsync, you need the ZKsync Solidity compiler toolchain.
It consists of two components:

1. The main component: [*zksolc*](https://github.com/matter-labs/era-compiler-solidity/releases).
2. The additional component: [*solc*](https://docs.soliditylang.org/en/latest), which produces Solidity artifacts used by *zksolc*.

> We are using our [fork](https://github.com/matter-labs/era-solidity) of the upstream *solc* compiler.
> The fork is necessary to support several ZKsync-specific features and workarounds.



## System Requirements

It is recommended to have at least 4 GB of RAM to compile large projects. The compilation process is parallelized by default, so the number of threads used is
equal to the number of CPU cores.

> Large projects can consume a lot of RAM during compilation on machines with a high number of cores.
> If you encounter memory issues, consider reducing the number of threads using the `--threads` option.

The table below outlines the supported platforms and architectures:

| CPU/OS | MacOS | Linux | Windows |
|:------:|:-----:|:-----:|:-------:|
| x86_64 |   ✅   |   ✅   |    ✅    |
| arm64  |   ✅   |   ✅   |    ❌    |

> Please avoid using outdated distributions of operating systems, as they may lack the necessary dependencies or include outdated versions of them.
> *zksolc* is only tested on recent versions of popular distributions, such as MacOS 11.0 and Windows 10.

<div class="warning">
<a href="https://musl.libc.org">musl</a>-based builds are deprecated, but they are still supported to preserve tooling compatibility.<br>
Starting from <b>zksolc</b> v1.5.3, we are shipping builds statically linked with <a href="https://www.gnu.org/software/libc/">the GNU C library</a>.
</div>



## Installing *zksolc*

You can install the ZKsync Solidity compiler toolchain using the following methods:

1. Use Foundry, Hardhat, or other popular toolkits, so they will manage the compiler installation and their dependencies for you. See [Ethereum Development Toolkits](#ethereum-development-toolkits).
2. Download pre-built binaries of [*solc*](https://github.com/matter-labs/era-solidity/releases) and [*zksolc*](https://github.com/matter-labs/era-compiler-solidity/releases). See [Static Executables](#static-executables).
3. Build *zksolc* from sources. See [Building from Source](#building-from-source).

> For small projects, learning and research purposes, *zksolc* and *solc* executables without a toolkit are sufficient.



## Installing *solc*

Running *zksolc* requires the [fork of the Solidity compiler *solc*](https://github.com/matter-labs/era-solidity/releases) where [we fixed several issues](./eravm/02-evm-assembly-translator.md#zksync-fork-of-solc) with lowering of EVM assembly to LLVM IR. The fork is called by *zksolc* as a child process. To point *zksolc* to the location of *solc*, use one of the following methods:

1. Add the location of *solc* to the environment variable `PATH`. 
  
   For example, if you have downloaded *solc* to the directory `/home/username/opt`,
   you can execute the following command, or append it to the configuration file of your shell:

    ```shell
    export PATH="/home/username/opt:${PATH}"
    ```

2. Alternatively, when you run *zksolc*, provide the full path to *solc* using the `--solc` option.

   For example, if `solc` is located in your current working directory, you can point to it with this command:

    ```shell
    zksolc --solc './solc' --bin 'Greeter.sol'
    ```

> The second option is more convenient if you are using different versions of *solc* for different projects.
> The current version of *zksolc* supports *solc* versions from 0.4.12 to 0.8.28.



## Versioning

The *zksolc* versioning scheme does not yet follow the [Semantic Versioning](https://semver.org) specification. Instead, its major and minor versions match those of the EraVM protocol for which *zksolc* produces bytecode. The patch version is incremented with each release, regardless of whether breaking changes are introduced. Therefore, please consult the changelog before updating the compiler.

Versions of [our *solc* fork](https://github.com/matter-labs/era-solidity/releases) consist of two *semver*-compatible parts:
1. Original upstream version
2. ZKsync revision

For instance, the latest revision of the latest version of *solc* is `0.8.28-1.0.1`. Here are the ZKsync revisions released by now:

| Revision |                           Features                         |
|:---------|:-----------------------------------------------------------|
| *v1.0.0* | Fixed compatibility between EVM assembly and LLVM IR       |
| *v1.0.1* | Fixed a compiler crash with nested try-catch patterns      |

> We recommend always using the latest version of *zksolc* and *solc* to benefit from the latest features and bug fixes.
> Starting from *zksolc* v1.5.8, it is not possible to use the original *solc* with *zksolc* anymore.



## Ethereum Development Toolkits

For large codebases, it is more convenient to use the ZKsync compiler toolchain via toolkits like Foundry and Hardhat.
These tools manage the compiler executables and their dependencies, and provide additional features like incremental compilation and caching.

The ZKsync toolchain is supported by the following toolkits:
1. [Foundry](https://docs.zksync.io/zksync-era/tooling/foundry/getting-started)
2. [Hardhat](https://docs.zksync.io/build/tooling/hardhat/guides/getting-started)



## Static Executables

We ship *zksolc* binaries on the [releases page of `matter-labs/era-compiler-solidity` repository](https://github.com/matter-labs/era-compiler-solidity/releases). 
This repository maintains intuitive and stable naming for the executables and provides a changelog for each release. Tools using *zksolc* will download the binaries from this repository and cache them locally.

<div class="warning">
The <a href="https://github.com/matter-labs/era-compiler-solidity">matter-labs/era-compiler-solidity</a> repository only contains builds for versions 1.4.0 and newer.<br>
You can download older versions from <a href="https://github.com/matter-labs/zksolc-bin/tree/main">the main branch</a> or <a href="https://github.com/matter-labs/zksolc-bin/releases">the releases page</a> of the deprecated repository for zksolc executables.<br>
If any of your projects are still using the old locations, please change their download URLs to <a href="https://github.com/matter-labs/era-compiler-solidity/releases">the new one</a>.
</div>

> All binaries are statically linked and must work on all recent platforms without issues.
> *zksolc* is fully written in Rust, aiming to minimize incompatibilities with the environment.



## Building from Source

> Please consider using the pre-built executables before building from source.
> Building from source is only necessary for development, research, and debugging purposes.
> Deployment and production use cases should rely only on [the officially released executables](#static-executables).

1. Install the necessary system-wide dependencies.

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

2. Install Rust.

   The easiest way to do it is following the latest [official instructions](https://www.rust-lang.org/tools/install).

> The Rust version used for building is pinned in the [rust-toolchain.toml](../rust-toolchain.toml) file at the repository root.
> *cargo* will automatically download the pinned version of *rustc* when you start building the project.

3. Clone and checkout this repository.

   ```shell
   git clone https://github.com/matter-labs/era-compiler-solidity
   ```
    
4. Install the ZKsync LLVM framework builder. This tool clones the [repository of ZKsync LLVM Framework](https://github.com/matter-labs/era-compiler-llvm) and runs a sequence of build commands tuned for the needs of ZKsync compiler toolchain.

    ```shell
    cargo install compiler-llvm-builder
    ```

    To fine-tune your build of ZKsync LLVM framework, refer to the section [Fine tuning ZKsync LLVM build](#fine-tuning-zksync-llvm-build)

> Always use the latest version of the builder to benefit from the latest features and bug fixes.
> To check for new versions and update the builder, simply run `cargo install compiler-llvm-builder` again, even if you have already installed the builder.
> The builder is not the ZKsync LLVM framework itself, but a tool to build it.
> By default, it is installed in `~/.cargo/bin/`, which is usually added to your `PATH` during the Rust installation process.

5. Clone and build the ZKsync LLVM framework using the `zksync-llvm` tool.
  
   ```shell
   # Navigate to the root of your local copy of this repository.
   cd era-compiler-solidity
   # Clone the ZKsync LLVM framework. The branch is specified in the file `LLVM.lock`.
   zksync-llvm clone
   # Build the ZKsync LLVM framework.
   zksync-llvm build
   ```
  
   For more information and available build options, run `zksync-llvm build --help`.
   
   You can also clone and build LLVM framework outside of the repository root.
   In this case, do the following:
   
   1. Provide an `LLVM.lock` file in the directory where you run `zksync-llvm`.
      See the [default LLVM.lock for an example](../LLVM.lock).
   2. Ensure that `LLVM.lock` selects the correct branch of the [ZKsync LLVM Framework repository](https://github.com/matter-labs/era-compiler-llvm).
   3. Before proceeding to the next step, set the environment variable `LLVM_SYS_170_PREFIX` to the path of the directory with the LLVM build artifacts.
      Typically, it ends with `target-llvm/build-final`, which is the default LLVM target directory of the LLVM builder. For example:

      ```shell
      export LLVM_SYS_170_PREFIX=~/repositories/era-compiler-solidity/target-llvm/build-final 
      ```

6. Build the *zksolc* executable.

    ```shell
    cargo build --release
    ```
   
    The *zksolc* executable will appear at `./target/release/zksolc`, where you can run it directly or move it to another location.

    If *cargo* cannot find the LLVM build artifacts, return to the previous step and ensure that the `LLVM_SYS_170_PREFIX` environment variable is set to the absolute path of the directory `target-llvm/build-final`.



## Tuning the ZKsync LLVM build

* For more information and available build options, run `zksync-llvm build --help`.
* Use the `--use-ccache` option to speed up the build process if you have [ccache](https://ccache.dev) installed.
* To build ZKsync LLVM framework using specific C and C++ compilers, pass additional arguments to [CMake](https://cmake.org/) using the `--extra-args` option:

  ```shell
  # Pay special attention to character escaping.

  zksync-llvm build \
    --use-ccache \
    --extra-args \
      '\-DCMAKE_C_COMPILER=/opt/homebrew/Cellar/llvm@18/18.1.8/bin/clang' \
      '\-DCMAKE_BUILD_TYPE=Release' \
      '\-DCMAKE_CXX_COMPILER=/opt/homebrew/Cellar/llvm@18/18.1.8/bin/clang++' 
  ```

### Building LLVM manually

* If you prefer building [your ZKsync LLVM](https://github.com/matter-labs/era-compiler-llvm) manually, include the following flags in your CMake command:

  ```shell
  # We recommended using the latest version of CMake.

  -DLLVM_TARGETS_TO_BUILD='EraVM;EVM'
  -DLLVM_ENABLE_PROJECTS='lld'
  -DBUILD_SHARED_LIBS='Off'
  ```

> For most users, the [ZKsync LLVM builder](#building-from-source) is the recommended way to build the ZKsync LLVM framework.
> This section exists for the ZKsync toolchain developers and researchers with specific requirements and experience with the LLVM framework.
> We are going to present a more detailed guide for LLVM contributors in the future.
