; ModuleID = 'probe4.1eb39152d7b6932d-cgu.0'
source_filename = "probe4.1eb39152d7b6932d-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

@alloc_e4248cc565c5235fd8af4e2be28f06f8 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/df871fbf053de3a855398964cd05fadbe91cf4fd\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc_03f9311b37d9dfc659338866c8d97833 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e4248cc565c5235fd8af4e2be28f06f8, [16 x i8] c"K\00\00\00\00\00\00\00v\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h28a96e340bf23b9dE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hcbd98a8fe37a7e52E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hdce64ae5ff0d7abcE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_03f9311b37d9dfc659338866c8d97833) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17hcbd98a8fe37a7e52E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hdce64ae5ff0d7abcE(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0-nightly (df871fbf0 2023-10-24)"}
