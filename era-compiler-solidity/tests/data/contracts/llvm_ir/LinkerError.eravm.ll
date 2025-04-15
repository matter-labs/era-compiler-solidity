target datalayout = "E-p:256:256-i256:256:256-S32-a:256:256"
target triple = "eravm-unknown-unknown"
declare void @foo()                                              
define void @glob() nounwind {                                    
  call void @foo()                                                
  ret void
}