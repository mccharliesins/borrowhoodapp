; ModuleID = 'autocfg_91ed2c81f4608257_4.609d5b177d1222ae-cgu.0'
source_filename = "autocfg_91ed2c81f4608257_4.609d5b177d1222ae-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

@alloc_e6758488a51c40069ade2309416f0500 = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"<anon>" }>, align 1
@alloc_5760cc517ece796840acf2ae2b53a0b1 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_e6758488a51c40069ade2309416f0500, [12 x i8] c"\06\00\00\00\01\00\00\00\1F\00\00\00" }>, align 4

; autocfg_91ed2c81f4608257_4::probe
; Function Attrs: nounwind
define dso_local void @_ZN26autocfg_91ed2c81f4608257_45probe17he950b47dced67434E() unnamed_addr #0 {
start:
  ret void
}

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17hd6cfbd7a1ed0448cE(ptr align 4) unnamed_addr #1

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { cold noinline noreturn nounwind "target-cpu"="generic" }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.86.0 (05f9846f8 2025-03-31)"}
