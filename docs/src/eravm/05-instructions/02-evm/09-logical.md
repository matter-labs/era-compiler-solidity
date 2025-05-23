# Logical



## LT

Original [EVM](https://www.evm.codes/#10?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp ult i256 %value1, %value2
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r0, r1
ptr.add.s       36, r1, r2
ld      r2, r2
ptr.add.s       4, r1, r1
ld      r1, r1
sub!    r1, r2, r1
add     0, r0, r1
add.lt  1, r0, r1
st.1    128, r1
```



## GT

Original [EVM](https://www.evm.codes/#11?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp ugt i256 %value1, %value2
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r0, r1
ptr.add.s       36, r1, r2
ld      r2, r2
ptr.add.s       4, r1, r1
ld      r1, r1
sub!    r1, r2, r1
add     0, r0, r1
add.gt  1, r0, r1
st.1    128, r1
```



## SLT

Original [EVM](https://www.evm.codes/#12?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp slt i256 %value1, %value2
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r0, r1
ptr.add.s       36, r1, r2
ld      r2, r2
ptr.add.s       4, r1, r1
ld      r1, r1
add     @CPI0_4[0], r0, r3
sub!    r1, r2, r4
add     r0, r0, r4
add.lt  r3, r0, r4
and     @CPI0_4[0], r2, r2
and     @CPI0_4[0], r1, r1
sub!    r1, r2, r5
add.le  r0, r0, r3
xor     r1, r2, r1
sub.s!  @CPI0_4[0], r1, r1
add     r4, r0, r1
add.eq  r3, r0, r1
sub!    r1, r0, r1
add     0, r0, r1
add.ne  1, r0, r1
st.1    128, r1
```



## SGT

Original [EVM](https://www.evm.codes/#13?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp sgt i256 %value1, %value2
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r0, r1
ptr.add.s       36, r1, r2
ld      r2, r2
ptr.add.s       4, r1, r1
ld      r1, r1
add     @CPI0_4[0], r0, r3
sub!    r1, r2, r4
add     r0, r0, r4
add.gt  r3, r0, r4
and     @CPI0_4[0], r2, r2
and     @CPI0_4[0], r1, r1
sub!    r1, r2, r5
add.ge  r0, r0, r3
xor     r1, r2, r1
sub.s!  @CPI0_4[0], r1, r1
add     r4, r0, r1
add.eq  r3, r0, r1
sub!    r1, r0, r1
add     0, r0, r1
add.ne  1, r0, r1
st.1    128, r1
```



## EQ

Original [EVM](https://www.evm.codes/#14?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp eq i256 %value1, %value2
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r0, r1
ptr.add.s       36, r1, r2
ld      r2, r2
ptr.add.s       4, r1, r1
ld      r1, r1
sub!    r1, r2, r1
add     0, r0, r1
add.eq  1, r0, r1
st.1    128, r1
```



## ISZERO

Original [EVM](https://www.evm.codes/#15?fork=shanghai) instruction.

### LLVM IR

```llvm
%comparison_result = icmp eq i256 %value, 0
%comparison_result_extended = zext i1 %comparison_result to i256
```

[LLVM IR instruction documentation](https://releases.llvm.org/15.0.0/docs/LangRef.html#icmp-instruction)

### EraVM Assembly

```asm
ptr.add stack[@ptr_calldata], r1, r1
ld      r1, r1
sub!    r1, r0, r1
add     0, r0, r1
add.eq  1, r0, r1
st.1    128, r1
```
