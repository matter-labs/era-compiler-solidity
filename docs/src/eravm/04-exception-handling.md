# Exception Handling

This page highlights specific nuances of exception handling (EH) in the EraVM architecture.

In essence, EraVM uses two EH mechanisms: [contract-level](#contract-level) and [function-level](#function-level). The former is inherited from the EVM architecture, while the latter aligns more closely with general-purpose languages.

|              | Contract Level  | Function Level               |
|:------------:|:---------------:|:----------------------------:|
| Yul Example  | `revert(0, 0)`  | `verbatim("throw")`          |
| Native to    | EVM             | General-purpose languages    |
| Handled by   | EraVM           | Compiler                     |
| Caught by    | Caller contract | Caller function              |
| Efficiency   | High            | Low                          |



## Contract Level

This type of exception is inherited from the EVM architecture. In EVM, instructions like `REVERT` and `INVALID` immediately terminate the contract’s execution and return control to the callee. It is impossible to catch them within the contract; only the callee can detect them by checking the call status code.

```solidity
// callee
revert(0, 0)

// caller
let success = call(...)
if iszero(success) {
    // option 1: rethrow on the contract level
    returndatacopy(...)
    revert(...)

    // option 2: rethrow on the function level
    verbatim("throw") // only available in the Yul mode
}
```

EraVM’s behavior is fully equivalent: the VM unwinds the call stack all the way to the contract’s top-level function frame, leaving no possibility to intercept or handle the exception along the way.

These types of exceptions are more efficient, as you can revert at any point of the execution without propagating the exception all the way up.

### Implementation

In EraVM, contracts invoke one another via [the `far_call` instruction](https://matter-labs.github.io/eravm-spec/spec.html#FarCalls), which includes [the exception handler’s address](https://matter-labs.github.io/eravm-spec/spec.html#OpFarCall) among its arguments.



## Function Level

This type of exception handling is common in general-purpose languages such as C++. As a result, it integrates naturally into LLVM, even though it is not supported by the smart contract languages our compilers handle. This is also why the two EH mechanisms are treated separately and do not interact within high-level code.

In general-purpose languages, a range of EH operators (e.g. `try` , `throw`, and `catch`) typically indicates which code sections can throw exceptions and how they should be handled. These tools are absent in Solidity and its EVM Yul dialect, so we introduced extensions to the [EraVM Yul dialect](./01-extensions.md) supported by *zksolc*.

If the contract does not define an EH function named `ZKSYNC_CATCH_NEAR_CALL`, there is no need to generate `catch` blocks. Panics will simply propagate to the callee contract by EraVM without any extra overhead.

Several constraints arise from Yul’s structure and the nature of smart contracts:

1. Any function beginning with `ZKSYNC_NEAR_CALL` is implicitly wrapped with `try`. If there is an exception handler defined, the following will happen:
    - A panic will be caught by the caller of such function.
    - Control then transfers to the EH function.
    - After the EH function finishes, control returns to the caller of `ZKSYNC_NEAR_CALL`.
2. Every operation can be considered `throw`.
    - Any instruction may panic due to out-of-gas, so all instructions can potentially throw.
    - This reduces optimization opportunities.
3. The `catch` block is represented by the `ZKSYNC_CATCH_NEAR_CALL` function in Yul.
    - A panic in `ZKSYNC_NEAR_CALL` makes **its caller** catch the exception and call the EH function.
    - Once the EH function completes, control returns to the caller of `ZKSYNC_NEAR_CALL`.
4. Only one EH function is allowed, and it must be named `ZKSYNC_CATCH_NEAR_CALL`.
    - This approach is not very efficient because every function must include an LLVM IR `catch` block to capture and propagate exceptions to the EH function.

```solidity
// Follow the numbers for the order of execution. The call order is:
// 1. caller
// 2. ZKSYNC_NEAR_CALL_callee
// 3. callee_even_deeper
// 4. ZKSYNC_CATCH_NEAR_CALL
// 5. caller

function ZKSYNC_NEAR_CALL_callee() -> value {    // 03
    value := callee_even_deeper()                // 04
}

function callee_even_deeper() -> value {         // 05
    verbatim("throw")                            // 06
}

// Each LLVM IR function automatically includes an implicit 'catch' block,
// which performs the following actions:
//     1. If a return value is expected, keep it zero-initialized ('zero').
//     2. Call the EH function ('ZKSYNC_CATCH_NEAR_CALL').
//     3. Resume execution with the next instruction (e.g., 'value := 42').
function caller() -> value {                      // 01
    let zero := ZKSYNC_NEAR_CALL_callee()         // 02
    value := 42                                   // 09
}

// This handler can also revert execution. Reverts in EH functions cannot be caught,
// so they immediately terminate the execution and return control to the callee contract.
function ZKSYNC_CATCH_NEAR_CALL() {               // 07
    log0(...)                                     // 08
}
```
