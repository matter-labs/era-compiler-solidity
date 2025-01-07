# EraVM Binary Layout

This page describes how assembler listing looks like and how it is transformed to bytecode that can be deployed to the chain.



## Definitions

- A directive is a command issued to the assembler, which is not translated into an executable bytecode instruction.
    Their names start with a period, for example, `.cell`.
    Directives are used to regulate the translation process.
- An instruction constitutes the smallest executable segment of bytecode.
    In EraVM, each instruction is exactly eight bytes long.
- A word is a 256-bit unsigned integer in a big-endian format.



## Structure of Assembly File

This section describes the structure of an EraVM assembly file, a text file typically with the extension `.zasm`.

### Data Types

- `U256` word, a 256-bit unsigned integer number, big-endian.
- `U16` 16-bit unsigned integer number, big-endian.

### Sections

The source code within an EraVM assembly is organized into several sections. The start of a section is denoted by one of the following directives:

- `.rodata`: constant, read-only data.
- `.data`: global mutable data.
- `.text`: executable code.

The description of any section may be spread across the file:

```asm
.rodata
    .cell 0
.text
    <some instruction>
.rodata
    .cell 1
```

In this example, multiple `.rodata` sections appear, but in the resulting binary file they will be merged into a single contiguous region of memory. The same principle applies to other sections.

### Defining Data

The `.cell` directive defines data:

```asm
.rodata
    .cell -1
    .cell 23090
.data
    .cell 1213
```

Notes:

- Using `.cell` in the `.data` section is deprecated and will not be supported in the future versions of assembly.
- The value of cell is provided as a signed 256-bit decimal number.
- Negative numbers will be encoded as 256-bit 2's complement, e.g. `-1` is encoded as `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff`.
- An optional `+` sign before positive numbers is allowed, e.g. `.cell +123`.
- Hexadecimal integer literals are not supported.

Symbols (label names) are supported, for example:

```asm
.text

f:
   add r0, r0, r0

g:
   add r0, r0, r0

.rodata

my_cells:
    .cell @lab1
    .cell @lab2
    .cell -1
```

A single `@` is prefixing the label name.

Each `.cell` is 256-bit wide, even though an address such as `@lab1` or `@lab2` is just 16-bit wide.
Addresses are padded with zeroes to fit in the word.

### Overall Structure

The structure of an assembly file is described as follows:

```asm
<file> := <section>*

<section> :=
    | ".rodata" <eol>  <const-element>*
    | ".data" <eol> <data-element> *
    | ".text" <eol>  <code-element> *

<const-element> := <label> | <cell>
<label> ::= [a-zA-Z_.@][0-9a-zA-Z_.@]

<data-element> := <label> | <cell>
<cell> :=
    ".cell" <256-bit signed or unsigned constant>

<comment> ::= ";" .*
<labels> ::= (<label> ":" ) *
<code-element> ::= <labels> <instruction> <operand-list> <comment>? EOL
```

- `EOL` stands for &ldquo;end of line&rdquo;.
- `<instruction>`, `<operand-list>` depend on the specific instruction.
   See the [EraVM specification](https://matter-labs.github.io/eravm-spec/spec.html).



## Execution model

This section provides some elements of the execution environment, the Era Virtual Machine.
Full execution model is described in [EraVM specification](https://matter-labs.github.io/eravm-spec/spec.html).

### Registers

EraVM has 16 general-purpose registers and 2 special registers:

- `PC` is a 16-bit program counter register; it holds the address of the next instruction to be executed.
- `SP` is a 16-bit stack pointer register. It points to the address following the top of the stack.

### Memory

EraVM memory is divided into **pages**. When a contract is launched, EraVM assigns several pages to it:

- **Code** page.
  - Immutable.
  - Contains `2^{16}` words.
  - Used to store both instructions and the constants of type `U256`.
  - Each word may contain 4 instructions or one constant.
  - Instructions and constants are indistinguishable.
  - Code page is addressable in two ways:

    - When EraVM fetches instruction from this page using `PC`, it addresses 8-byte chunks.
    - When EraVM fetches constants from this page, it addresses 32-byte (word-sized) chunks.

      For example, reading a constant by the address 0 will yield a word composed of binary
      encoded instructions number 0, 1, 2 and 3; reading a constant by
      the address 1 from this page will yield a binary encoding for the
      instructions number 4,5,6,7, and so on.

- **Heap** page.
  - Contains `2^{32}` bytes and is byte-addressable.
  - However, it is only possible to read words from heap, not the individual
        bytes.

- **Data stack** page.
  - Contains `2^{16}` words.
  - Grows towards higher addresses, so every push-like instruction advances `SP` by at least one.
  - Reserving space on stack is therefore incrementing the value of `SP`.
  - Each word has an additional tag. If the tag is set, the word contains a
    pointer to a heap page, either of this contract or belonging to a different
    contract.
  - Data on stack page can be addressed by their absolute addresses, or relative
    to `SP`.
  - Global mutable variables are allocated on stack.

### Callstack

EraVM has a separate call stack, a utility data structure that holds information about call frames.
There are two kinds of call frames in the EraVM, corresponding to near and far calls:

- Far call frame corresponds to a call to a different contract.
- Near call frame corresponds to a near call to the code inside the same
  contract. Near calls are a low-level mechanism that is used mostly in system
  contracts.

Call stack differs from the data stack pages, described in section **Memory**.



## Binary layout

The binary file published on chain and passed to EraVM has no structure. It is an image loaded at the beginning of the **code** page (with offset 0).

The initial value of `PC` is zero, therefore the execution will start at the first instruction on the code page.
Instructions or functions in `.text` section are not reordered, so the first instruction appearing in the assembly file will be executed first, regardless of labels.

The length of the binary should be an odd number of words, that is, `32 * (2N+1)` bytes.

The last word in the binary file is the metadata hash, see section **Metadata Hash**.



## Symbols

There are three default predefined symbols:

1. `DEFAULT_UNWIND`: default exception handler / stack unroller for near call instruction `call`.
2. `DEFAULT_FAR_RETURN`:  default stack unroller for returns (see **Landing Pads**).
3. `DEFAULT_FAR_REVERT`:  default stack unroller for reverts (see **Landing Pads**).

If the user did not define one of these labels, the assembler will define it and emit a corresponding landing pad (see **Landing Pads**).



## Linking and loading

This section details how the assembly file structure is flattened into a loadable image.

The binary file is divided into three regions:

1. Initializer.
2. Instructions.
3. Constant pool.

The following subsections describe these regions.

### Initializer region

Mutable global variables are allocated in the beginning of the stack page, not in code.
The stack page supports absolute addressing, therefore the global variables can be accessed directly by their addresses.

If the assembly file defines global variables, the assembler will emit a special initializer code in the beginning of the program; otherwise, initializer region is skipped and we pass to the code region immediately.

The first instruction of the initializer region is `incsp <number of globals>`. It allocates one word on a data stack per global mutable variable.

For each global that is initialized with a non-zero value, assembler does the following:

- Copies its initializer to `.rodata`, which will be loaded to the code page.
- Emits an instruction:

```asm
add code[INIT], r0, stack[IDX]
```

where:

- `INIT` is the address of the initializer in the `.rodata`.
- `IDX`  is the index of the global variable.

For example, the following program:

```asm
.text

some_label:
  sub!   r0, r0, r0
  jump @some_label

.data
    my_globals:
    .cell 32

.rodata
    .cell 0
```

Will be translated as if it were written this way:

```asm
.text
init_globals:
    incsp 1
    add code[@global_init_0], r0, stack[0]

some_label:
    sub! r0, r0, r0
    jump @some_label

.rodata
    .cell 0
    global_init_0:
    .cell 32
```

### Code region

The `.text` section is emitted after the initializer region or, if there are no globals, right in the start of the binary file.
It is followed by the landing pads and the padding, before the start of the constant pool region.

#### Landing Pads

After emitting the instructions provided in the `.text` section of the assembly file, the assembler may emit the landing pads for near calls, returns and reverts.
This happens for three predefined symbols: `DEFAULT_UNWIND`, `DEFAULT_FAR_RETURN` and `DEFAULT_FAR_REVERT`.

For example, if the symbol `DEFAULT_FAR_RETURN` is not explicitly defined, it will be defined automatically and the following landing pad will be appended to the executable code:

```asm
;; landing pad for returns
DEFAULT_FAR_RETURN:
    retl @DEFAULT_FAR_RETURN
```

If the contract executes an instruction `retl @DEFAULT_FAR_RETURN`, the control is passed to the address `DEFAULT_FAR_RETURN`, which hosts the same instruction.
This starts a loop, popping all near call frames from the callstack. The last `retl` will perform a far return from the contract.
This allows emitting `retl @DEFAULT_FAR_RETURN` to return from any place inside the contract, no matter how many near calls are currently active.

If neither of the predefined symbols `DEFAULT_UNWIND`, `DEFAULT_FAR_RETURN`, `DEFAULT_FAR_REVERT` was defined explicitly, the following code will be emitted after the `.text` section.

```asm
;; landing pad for near calls
DEFAULT_UNWIND:
    ret.panic.to_label r0, @DEFAULT_UNWIND

;; landing pad for returns
DEFAULT_FAR_RETURN:
    ret.ok.to_label r1, @DEFAULT_FAR_RETURN

;; landing pad for reverts
DEFAULT_FAR_REVERT:
    ret.revert.to_label r1, @DEFAULT_FAR_REVERT
```

#### Code padding

The code section starts at 0, if we count the initializing code as its part. Therefore, it is aligned on a 32 byte boundary.
If the total number of instructions, with the landing pads, is not divisible by `4`, the assembler emits 1 to 3 `INVALID` instructions as a padding.
This way, the instructions will fill a certain number of words completely, and the following region (constant pool region) is aligned on a 32-byte boundary as well.

### Constant pool region

The constant pool region is aligned on a `32` byte boundary.
It is placed immediately after the code region and contains:

- Constants defined in `.rodata` section.
- Initializers for mutable globals.
- Padding: nothing or a zero-word to ensure, that the total length of the binary file, including the following hash, equals to an odd number of words.
- Optionally, **metadata hash**.

### Metadata Hash

An optional, implementation-defined hash of the contract metadata, which may include its source.
Depending on the initial layer where the compilation starts (a Solidity contract, its Yul code, assembly), the hash value may be different.

- [Metadata usage and definition.](../02-command-line-interface.md#--metadata)
- [Supported metadata hash types.](../02-command-line-interface.md#--metadata-hash)
