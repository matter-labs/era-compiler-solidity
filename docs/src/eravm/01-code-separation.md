# Code Separation

On both EVM and EraVM the code is separated into two parts: deploy code and runtime code. The deploy code is executed
only once when the contract is deployed. The runtime code is executed every time the contract is called. However, on
EraVM the deploy code and runtime code are deployed together, and they are not split into two separate chunks.

The constructor is added to the contract as a regular public function which is called by System Contracts during
deployment.

Just like on EVM, the deploy code on EraVM is represented by a single constructor. The constructor is merged into
runtime code by our compiler while it is generating LLVM IR, like in the minimal example below.



## LLVM IR

In EraVM subset of LLVM IR, contract `@__entry` arguments `%0`-`%11` correspond to registers `r1`-`r12` on EraVM.

Register `r2` is represented by argument `%1`. The register contains a bit that indicates whether the call is
a deploy code call. The flag is used for branching over deploy and runtime code blocks.

```llvm
define i256 @__entry(ptr addrspace(3) nocapture readnone %0, i256 %1, i256 %2, i256 %3, i256 %4, i256 %5, i256 %6, i256 %7, i256 %8, i256 %9, i256 %10, i256 %11) local_unnamed_addr #1 personality ptr @__personality {
entry:
  %is_deploy_code_call_flag_truncated = and i256 %1, 1                                                          ; check if the call is a deploy code call
  %is_deploy_code_call_flag.not = icmp eq i256 %is_deploy_code_call_flag_truncated, 0                           ; invert the flag
  br i1 %is_deploy_code_call_flag.not, label %runtime_code_call_block, label %deploy_code_call_block            ; branch to the deploy code block if the flag is set

deploy_code_call_block:                           ; preds = %entry
  store i256 32, ptr addrspace(2) inttoptr (i256 256 to ptr addrspace(2)), align 256                            ; store the offset of the array of immutables
  store i256 0, ptr addrspace(2) inttoptr (i256 288 to ptr addrspace(2)), align 32                              ; store the length of the array of immutables
  tail call void @llvm.eravm.return(i256 53919893334301279589334030174039261352344891250716429051063678533632) ; return the array of immutables using EraVM return ABI data encoding
  unreachable

runtime_code_call_block:                          ; preds = %entry
  store i256 42, ptr addrspace(1) null, align 4294967296                                                        ; store a value to return
  tail call void @llvm.eravm.return(i256 2535301200456458802993406410752)                                      ; return the value using EraVM return ABI data encoding
  unreachable
}
```



## EraVM Assembly

In EraVM assembly, the branching looks like this:

```asm
__entry:
.func_begin0:
	and!	    1, r2, r0
	jump.ne	  @.BB0_1
	add	      r0, r0, r1
	retl	    @DEFAULT_FAR_RETURN
.BB0_1:
	add	32,   r0, r1
	stm.ah	  256, r1
	stm.ah	  288, r0
	add	      code[@CPI0_0], r0, r1
	retl	    @DEFAULT_FAR_RETURN
```
