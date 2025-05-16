; ModuleID = 'autocfg_91ed2c81f4608257_7.cf57c6f891357574-cgu.0'
source_filename = "autocfg_91ed2c81f4608257_7.cf57c6f891357574-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nounwind
define internal void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17hcb36a1192b2d168aE"(ptr sret([8 x i8]) align 1 %_0, double %self) unnamed_addr #0 {
start:
  store double %self, ptr %_0, align 1
  ret void
}

; autocfg_91ed2c81f4608257_7::probe
; Function Attrs: nounwind
define dso_local void @_ZN26autocfg_91ed2c81f4608257_75probe17h8c721a7838cc801dE() unnamed_addr #1 {
start:
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  call void @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17hcb36a1192b2d168aE"(ptr sret([8 x i8]) align 1 %_1, double 3.140000e+00) #2
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.86.0 (05f9846f8 2025-03-31)"}
