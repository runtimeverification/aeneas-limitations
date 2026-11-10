# Appendix A — Loop and iterator experiments (full)

Every case below was produced by `experiments/run_charon.sh` and `experiments/run_aeneas.sh` (default options). Lean output is shown with each `/-- ... -/` doc comment collapsed to a one-line `--` comment and blank lines removed; nothing else is edited. Long outputs are truncated (marked).

### L01_while  —  **CLEAN**

```rust
pub fn sum_to(n: u32) -> u32 {
    let mut i = 0;
    let mut s = 0;
    while i < n {
        s += i;
        i += 1;
    }
    s
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l01_while
-- [l01_while::sum_to]: loop body 0:
@[rust_loop_body]
def sum_to_loop.body
  (n : Std.U32) (i : Std.U32) (s : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32)
  := do
  if i < n
  then let s1 ← s + i
       let i1 ← i + 1#u32
       ok (cont (i1, s1))
  else ok (done s)
-- [l01_while::sum_to]: loop 0:
@[rust_loop]
def sum_to_loop
  (n : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  loop
    (fun (i1, s1) => sum_to_loop.body n i1 s1)
    (i, s)
-- [l01_while::sum_to]:
@[reducible]
def sum_to (n : Std.U32) : Result Std.U32 := do
  sum_to_loop n 0#u32 0#u32
end l01_while
```


### L02_loop_break  —  **CLEAN**

```rust
pub fn first_pow2_ge(n: u32) -> u32 {
    let mut p = 1;
    loop {
        if p >= n {
            break;
        }
        p *= 2;
    }
    p
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l02_loop_break
-- [l02_loop_break::first_pow2_ge]: loop body 0:
@[rust_loop_body]
def first_pow2_ge_loop.body
  (n : Std.U32) (p : Std.U32) : Result (ControlFlow Std.U32 Std.U32) := do
  if p >= n
  then ok (done p)
  else let p1 ← p * 2#u32
       ok (cont p1)
-- [l02_loop_break::first_pow2_ge]: loop 0:
@[rust_loop]
def first_pow2_ge_loop (n : Std.U32) (p : Std.U32) : Result Std.U32 := do
  loop
    (fun p1 => first_pow2_ge_loop.body n p1)
    p
-- [l02_loop_break::first_pow2_ge]:
@[reducible]
def first_pow2_ge (n : Std.U32) : Result Std.U32 := do
  first_pow2_ge_loop n 1#u32
end l02_loop_break
```


### L03_continue  —  **CLEAN**

```rust
pub fn sum_even(n: u32) -> u32 {
    let mut i = 0;
    let mut s = 0;
    while i < n {
        i += 1;
        if i % 2 == 1 {
            continue;
        }
        s += i;
    }
    s
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l03_continue
-- [l03_continue::sum_even]: loop body 0:
@[rust_loop_body]
def sum_even_loop.body
  (n : Std.U32) (i : Std.U32) (s : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32)
  := do
  if i < n
  then
    let i1 ← i + 1#u32
    let i2 ← i1 % 2#u32
    if i2 = 1#u32
    then ok (cont (i1, s))
    else let s1 ← s + i1
         ok (cont (i1, s1))
  else ok (done s)
-- [l03_continue::sum_even]: loop 0:
@[rust_loop]
def sum_even_loop
  (n : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  loop
    (fun (i1, s1) => sum_even_loop.body n i1 s1)
    (i, s)
-- [l03_continue::sum_even]:
@[reducible]
def sum_even (n : Std.U32) : Result Std.U32 := do
  sum_even_loop n 0#u32 0#u32
end l03_continue
```


### L04_early_return  —  **CLEAN**

```rust
pub fn find(s: &[u32], x: u32) -> Option<usize> {
    let mut i = 0;
    while i < s.len() {
        if s[i] == x {
            return Some(i);
        }
        i += 1;
    }
    None
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l04_early_return
-- [l04_early_return::find]: loop body 0:
@[rust_loop_body]
def find_loop.body
  (s : Slice Std.U32) (x : Std.U32) (i : Std.Usize) :
  Result (ControlFlow Std.Usize (Option Std.Usize))
  := do
  let i1 := Slice.len s
  if i < i1
  then
    let i2 ← Slice.index_usize s i
    if i2 = x
    then ok (done (some i))
    else let i3 ← i + 1#usize
         ok (cont i3)
  else ok (done none)
-- [l04_early_return::find]: loop 0:
@[rust_loop]
def find_loop
  (s : Slice Std.U32) (x : Std.U32) (i : Std.Usize) :
  Result (Option Std.Usize)
  := do
  loop
    (fun i1 => find_loop.body s x i1)
    i
-- [l04_early_return::find]:
@[reducible]
def find (s : Slice Std.U32) (x : Std.U32) : Result (Option Std.Usize) := do
  find_loop s x 0#usize
end l04_early_return
```


### L05_nested  —  **CLEAN**

```rust
pub fn count_pairs(n: u32) -> u32 {
    let mut c = 0;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < i {
            c += 1;
            j += 1;
        }
        i += 1;
    }
    c
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l05_nested
-- [l05_nested::count_pairs]: loop body 1:
@[rust_loop_body]
def count_pairs_loop0_loop0.body
  (i : Std.U32) (c : Std.U32) (j : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32)
  := do
  if j < i
  then let c1 ← c + 1#u32
       let j1 ← j + 1#u32
       ok (cont (c1, j1))
  else ok (done c)
-- [l05_nested::count_pairs]: loop 1:
@[rust_loop]
def count_pairs_loop0_loop0
  (c : Std.U32) (i : Std.U32) (j : Std.U32) : Result Std.U32 := do
  loop
    (fun (c1, j1) => count_pairs_loop0_loop0.body i c1 j1)
    (c, j)
-- [l05_nested::count_pairs]: loop body 0:
@[rust_loop_body]
def count_pairs_loop0.body
  (n : Std.U32) (c : Std.U32) (i : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32)
  := do
  if i < n
  then
    let c1 ← count_pairs_loop0_loop0 c i 0#u32
    let i1 ← i + 1#u32
    ok (cont (c1, i1))
  else ok (done c)
-- [l05_nested::count_pairs]: loop 0:
@[rust_loop]
def count_pairs_loop0
  (n : Std.U32) (c : Std.U32) (i : Std.U32) : Result Std.U32 := do
  loop
    (fun (c1, i1) => count_pairs_loop0.body n c1 i1)
    (c, i)
-- [l05_nested::count_pairs]:
@[reducible]
def count_pairs (n : Std.U32) : Result Std.U32 := do
  count_pairs_loop0 n 0#u32 0#u32
end l05_nested
```


### L05b_nested_break_outer  —  **FAIL**

```rust
pub fn find_pair(n: u32, t: u32) -> u32 {
    let mut i = 0;
    let mut r = 0;
    'outer: while i < n {
        let mut j = 0;
        while j < n {
            if i + j == t {
                r = i;
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }
    r
}
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Breaks to outer loops are not supported yet
[Error] Ignoring the body of 'l05b_nested_break_outer::find_pair' because of previous error
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l05b_nested_break_outer
-- [l05b_nested_break_outer::find_pair]:
axiom find_pair : Std.U32 → Std.U32 → Result Std.U32
end l05b_nested_break_outer
```


### L05c_nested_return  —  **FAIL**

```rust
pub fn find_pair(n: u32, t: u32) -> Option<(u32, u32)> {
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            if i + j == t {
                return Some((i, j));
            }
            j += 1;
        }
        i += 1;
    }
    None
}
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Returns inside of nested loops are not supported yet
[Error] Ignoring the body of 'l05c_nested_return::find_pair' because of previous error
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l05c_nested_return
-- [l05c_nested_return::find_pair]:
axiom find_pair : Std.U32 → Std.U32 → Result (Option (Std.U32 × Std.U32))
end l05c_nested_return
```


### L05d_nested_continue_outer  —  **FAIL**

```rust
pub fn f(n: u32) -> u32 {
    let mut i = 0;
    let mut c = 0;
    'outer: while i < n {
        i += 1;
        let mut j = 0;
        while j < i {
            j += 1;
            if j == 3 {
                continue 'outer;
            }
            c += 1;
        }
    }
    c
}
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Continue to outer loops are not supported yet
[Error] Ignoring the body of 'l05d_nested_continue_outer::f' because of previous error
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l05d_nested_continue_outer
-- [l05d_nested_continue_outer::f]:
axiom f : Std.U32 → Result Std.U32
end l05d_nested_continue_outer
```


### L06_for_range  —  **CLEAN**

```rust
pub fn sum_range(n: u32) -> u32 {
    let mut s = 0;
    for i in 0..n {
        s += i;
    }
    s
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l06_for_range
-- [l06_for_range::sum_range]: loop body 0:
@[rust_loop_body]
def sum_range_loop.body
  (iter : core.ops.range.Range Std.U32) (s : Std.U32) :
  Result (ControlFlow ((core.ops.range.Range Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ←
    core.iter.range.IteratorRange.next core.iter.range.StepU32 iter
  match o with
  | none => ok (done s)
  | some i => let s1 ← s + i
              ok (cont (iter1, s1))
-- [l06_for_range::sum_range]: loop 0:
@[rust_loop]
def sum_range_loop
  (iter : core.ops.range.Range Std.U32) (s : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, s1) => sum_range_loop.body iter1 s1)
    (iter, s)
-- [l06_for_range::sum_range]:
@[reducible]
def sum_range (n : Std.U32) : Result Std.U32 := do
  sum_range_loop { start := 0#u32, «end» := n } 0#u32
end l06_for_range
```


### L06b_for_range_inclusive_step  —  **CLEAN**

```rust
pub fn sum_incl(n: u32) -> u32 {
    let mut s = 0;
    for i in 0..=n {
        s += i;
    }
    s
}
pub fn sum_step(n: usize) -> usize {
    let mut s = 0;
    for i in (0..n).step_by(2) {
        s += i;
    }
    s
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l06b_for_range_inclusive_step
-- [l06b_for_range_inclusive_step::sum_incl]: loop body 0:
@[rust_loop_body]
def sum_incl_loop.body
  (iter : core.ops.range.RangeInclusive Std.U32) (s : Std.U32) :
  Result (ControlFlow ((core.ops.range.RangeInclusive Std.U32) × Std.U32)
    Std.U32)
  := do
  let (o, iter1) ←
    core.ops.range.RangeInclusive.Insts.CoreIterTraitsIteratorIterator.next
      core.iter.range.StepU32 iter
  match o with
  | none => ok (done s)
  | some i => let s1 ← s + i
              ok (cont (iter1, s1))
-- [l06b_for_range_inclusive_step::sum_incl]: loop 0:
@[rust_loop]
def sum_incl_loop
  (iter : core.ops.range.RangeInclusive Std.U32) (s : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, s1) => sum_incl_loop.body iter1 s1)
    (iter, s)
-- [l06b_for_range_inclusive_step::sum_incl]:
def sum_incl (n : Std.U32) : Result Std.U32 := do
  let iter ← core.ops.range.RangeInclusive.new 0#u32 n
  sum_incl_loop iter 0#u32
-- [l06b_for_range_inclusive_step::sum_step]: loop body 0:
@[rust_loop_body]
def sum_step_loop.body
  (iter : core.iter.adapters.step_by.StepBy (core.ops.range.Range Std.Usize))
  (s : Std.Usize) :
  Result (ControlFlow ((core.iter.adapters.step_by.StepBy (core.ops.range.Range
    Std.Usize)) × Std.Usize) Std.Usize)
  := do
  let (o, iter1) ←
    core.iter.adapters.step_by.IteratorStepBy.next
      (core.iter.traits.iterator.IteratorRange core.iter.range.StepUsize) iter
  match o with
  | none => ok (done s)
  | some i => let s1 ← s + i
              ok (cont (iter1, s1))
-- [l06b_for_range_inclusive_step::sum_step]: loop 0:
@[rust_loop]
def sum_step_loop
  (iter : core.iter.adapters.step_by.StepBy (core.ops.range.Range Std.Usize))
  (s : Std.Usize) :
  Result Std.Usize
  := do
  loop
    (fun (iter1, s1) => sum_step_loop.body iter1 s1)
    (iter, s)
-- [l06b_for_range_inclusive_step::sum_step]:
def sum_step (n : Std.Usize) : Result Std.Usize := do
  let iter ←
    core.iter.traits.iterator.Iterator.step_by.trait_default
      (core.iter.traits.iterator.IteratorRange core.iter.range.StepUsize)
      { start := 0#usize, «end» := n } 2#usize
  sum_step_loop iter 0#usize
end l06b_for_range_inclusive_step
```


### L07_for_slice  —  **CLEAN**

```rust
pub fn sum_slice(s: &[u32]) -> u32 {
    let mut t = 0;
    for x in s {
        t += *x;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l07_for_slice
-- [l07_for_slice::sum_slice]: loop body 0:
@[rust_loop_body]
def sum_slice_loop.body
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((core.slice.iter.Iter Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ← core.slice.iter.IteratorSliceIter.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l07_for_slice::sum_slice]: loop 0:
@[rust_loop]
def sum_slice_loop
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, t1) => sum_slice_loop.body iter1 t1)
    (iter, t)
-- [l07_for_slice::sum_slice]:
def sum_slice (s : Slice Std.U32) : Result Std.U32 := do
  let iter ←
    SharedSlice.Insts.CoreIterTraitsCollectIntoIteratorSharedIter.into_iter s
  sum_slice_loop iter 0#u32
end l07_for_slice
```


### L07b_index_loop_slice  —  **CLEAN**

```rust
pub fn sum_slice_idx(s: &[u32]) -> u32 {
    let mut t = 0;
    let mut i = 0;
    while i < s.len() {
        t += s[i];
        i += 1;
    }
    t
}
pub fn sum_slice_for_idx(s: &[u32]) -> u32 {
    let mut t = 0;
    for i in 0..s.len() {
        t += s[i];
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l07b_index_loop_slice
-- [l07b_index_loop_slice::sum_slice_idx]: loop body 0:
@[rust_loop_body]
def sum_slice_idx_loop.body
  (s : Slice Std.U32) (t : Std.U32) (i : Std.Usize) :
  Result (ControlFlow (Std.U32 × Std.Usize) Std.U32)
  := do
  let i1 := Slice.len s
  if i < i1
  then
    let i2 ← Slice.index_usize s i
    let t1 ← t + i2
    let i3 ← i + 1#usize
    ok (cont (t1, i3))
  else ok (done t)
-- [l07b_index_loop_slice::sum_slice_idx]: loop 0:
@[rust_loop]
def sum_slice_idx_loop
  (s : Slice Std.U32) (t : Std.U32) (i : Std.Usize) : Result Std.U32 := do
  loop
    (fun (t1, i1) => sum_slice_idx_loop.body s t1 i1)
    (t, i)
-- [l07b_index_loop_slice::sum_slice_idx]:
@[reducible]
def sum_slice_idx (s : Slice Std.U32) : Result Std.U32 := do
  sum_slice_idx_loop s 0#u32 0#usize
-- [l07b_index_loop_slice::sum_slice_for_idx]: loop body 0:
@[rust_loop_body]
def sum_slice_for_idx_loop.body
  (s : Slice Std.U32) (iter : core.ops.range.Range Std.Usize) (t : Std.U32) :
  Result (ControlFlow ((core.ops.range.Range Std.Usize) × Std.U32) Std.U32)
  := do
  let (o, iter1) ←
    core.iter.range.IteratorRange.next core.iter.range.StepUsize iter
  match o with
  | none => ok (done t)
  | some i =>
    let i1 ← Slice.index_usize s i
    let t1 ← t + i1
    ok (cont (iter1, t1))
-- [l07b_index_loop_slice::sum_slice_for_idx]: loop 0:
@[rust_loop]
def sum_slice_for_idx_loop
  (iter : core.ops.range.Range Std.Usize) (s : Slice Std.U32) (t : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, t1) => sum_slice_for_idx_loop.body s iter1 t1)
    (iter, t)
-- [l07b_index_loop_slice::sum_slice_for_idx]:
def sum_slice_for_idx (s : Slice Std.U32) : Result Std.U32 := do
  let i := Slice.len s
  sum_slice_for_idx_loop { start := 0#usize, «end» := i } s 0#u32
end l07b_index_loop_slice
```


### L08_for_vec  —  **CAVEATS**

```rust
pub fn sum_vec_ref(v: &Vec<u32>) -> u32 {
    let mut t = 0;
    for x in v {
        t += *x;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l08_for_vec
-- [alloc::vec::{impl core::iter::traits::collect::IntoIterator<&'a T, core::slice::iter::Iter<'a, T>> for &'a alloc::vec::Vec<T>}::into_iter]:
@[rust_fun
  "alloc::vec::{core::iter::traits::collect::IntoIterator<&'a alloc::vec::Vec<@T>, &'a @T, core::slice::iter::Iter<'a, @T>>}::into_iter"]
axiom SharedAVec.Insts.CoreIterTraitsCollectIntoIteratorSharedATIter.into_iter
  {T : Type _} (A : Type _) :
  alloc.vec.Vec T → Result (core.slice.iter.Iter T)
-- [l08_for_vec::sum_vec_ref]: loop body 0:
@[rust_loop_body]
def sum_vec_ref_loop.body
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((core.slice.iter.Iter Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ← core.slice.iter.IteratorSliceIter.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l08_for_vec::sum_vec_ref]: loop 0:
@[rust_loop]
def sum_vec_ref_loop
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, t1) => sum_vec_ref_loop.body iter1 t1)
    (iter, t)
-- [l08_for_vec::sum_vec_ref]:
def sum_vec_ref (v : alloc.vec.Vec Std.U32) : Result Std.U32 := do
  let iter ←
    SharedAVec.Insts.CoreIterTraitsCollectIntoIteratorSharedATIter.into_iter
      alloc.alloc.Global v
  sum_vec_ref_loop iter 0#u32
end l08_for_vec
```


### L08b_for_vec_owned  —  **CLEAN**

```rust
pub fn sum_vec_owned(v: Vec<u32>) -> u32 {
    let mut t = 0;
    for x in v {
        t += x;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l08b_for_vec_owned
-- [l08b_for_vec_owned::sum_vec_owned]: loop body 0:
@[rust_loop_body]
def sum_vec_owned_loop.body
  (iter : alloc.vec.into_iter.IntoIter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((alloc.vec.into_iter.IntoIter Std.U32) × Std.U32)
    Std.U32)
  := do
  let (o, iter1) ← alloc.vec.into_iter.IteratorIntoIter.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l08b_for_vec_owned::sum_vec_owned]: loop 0:
@[rust_loop]
def sum_vec_owned_loop
  (iter : alloc.vec.into_iter.IntoIter Std.U32) (t : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, t1) => sum_vec_owned_loop.body iter1 t1)
    (iter, t)
-- [l08b_for_vec_owned::sum_vec_owned]:
def sum_vec_owned (v : alloc.vec.Vec Std.U32) : Result Std.U32 := do
  let iter ← alloc.vec.IntoIteratorVec.into_iter v
  sum_vec_owned_loop iter 0#u32
end l08b_for_vec_owned
```


### L08c_vec_iter_method  —  **CLEAN**

```rust
pub fn sum_vec_iter(v: &Vec<u32>) -> u32 {
    let mut t: u32 = 0;
    for x in v.iter() {
        t = t.wrapping_add(*x);
    }
    t
}
pub fn incr_vec(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x = x.wrapping_add(1);
    }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l08c_vec_iter_method
-- [l08c_vec_iter_method::sum_vec_iter]: loop body 0:
@[rust_loop_body]
def sum_vec_iter_loop.body
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((core.slice.iter.Iter Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ← core.slice.iter.IteratorSliceIter.next iter
  match o with
  | none => ok (done t)
  | some x =>
    let t1 ← lift (core.num.U32.wrapping_add t x)
    ok (cont (iter1, t1))
-- [l08c_vec_iter_method::sum_vec_iter]: loop 0:
@[rust_loop]
def sum_vec_iter_loop
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, t1) => sum_vec_iter_loop.body iter1 t1)
    (iter, t)
-- [l08c_vec_iter_method::sum_vec_iter]:
def sum_vec_iter (v : alloc.vec.Vec Std.U32) : Result Std.U32 := do
  let s := alloc.vec.Vec.deref v
  let iter ← core.slice.Slice.iter s
  sum_vec_iter_loop iter 0#u32
-- [l08c_vec_iter_method::incr_vec]: loop body 0:
@[rust_loop_body]
def incr_vec_loop.body
  (iter : core.slice.iter.IterMut Std.U32)
  (back : core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32)
  :
  Result (ControlFlow ((core.slice.iter.IterMut Std.U32) ×
    (core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32))
    (core.slice.iter.IterMut Std.U32))
  := do
  let (o, iter1, next_back) ← core.slice.iter.IteratorIterMut.next iter
  match o with
  | none => ok (done (let im := next_back iter1 none
                      back im))
  | some x =>
    let x1 ← lift (core.num.U32.wrapping_add x 1#u32)
    ok (cont (iter1, fun im => let im1 := next_back im (some x1)
                               back im1))
-- [l08c_vec_iter_method::incr_vec]: loop 0:
@[rust_loop]
def incr_vec_loop
  (iter : core.slice.iter.IterMut Std.U32)
  (back : core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32)
  :
  Result (core.slice.iter.IterMut Std.U32)
  := do
  loop
    (fun (iter1, back1) => incr_vec_loop.body iter1 back1)
    (iter, back)
-- [l08c_vec_iter_method::incr_vec]:
def incr_vec (v : alloc.vec.Vec Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  let (s, deref_mut_back) ← lift (alloc.vec.Vec.deref_mut v)
  let (iter, iter_mut_back) ← core.slice.Slice.iter_mut s
  let back ← incr_vec_loop iter (fun im => im)
  let s1 := iter_mut_back back
  ok (deref_mut_back s1)
end l08c_vec_iter_method
```


### L09_for_array  —  **CAVEATS**

```rust
pub fn sum_array(a: [u32; 4]) -> u32 {
    let mut t = 0;
    for x in a {
        t += x;
    }
    t
}
pub fn sum_array_ref(a: &[u32; 4]) -> u32 {
    let mut t = 0;
    for x in a {
        t += *x;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l09_for_array
-- [core::array::iter::IntoIter]
@[rust_type "core::array::iter::IntoIter"]
axiom core.array.iter.IntoIter (T : Type _) (N : Std.Usize) : Type
-- [core::array::iter::{impl core::iter::traits::collect::IntoIterator<T, core::array::iter::IntoIter<T, N>> for [T; N]}::into_iter]:
@[rust_fun
  "core::array::iter::{core::iter::traits::collect::IntoIterator<[@T; @N], @T, core::array::iter::IntoIter<@T, @N>>}::into_iter"]
axiom Array.Insts.CoreIterTraitsCollectIntoIteratorTIntoIter.into_iter
  {T : Type _} {N : Std.Usize} :
  Array T N → Result (core.array.iter.IntoIter T N)
-- [core::array::iter::{impl core::iter::traits::iterator::Iterator<T> for core::array::iter::IntoIter<T, N>}::next]:
@[rust_fun
  "core::array::iter::{core::iter::traits::iterator::Iterator<core::array::iter::IntoIter<@T, @N>, @T>}::next"]
axiom core.array.iter.IntoIter.Insts.CoreIterTraitsIteratorIterator.next
  {T : Type _} {N : Std.Usize} :
  core.array.iter.IntoIter T N → Result ((Option T) ×
    (core.array.iter.IntoIter T N))
-- [l09_for_array::sum_array]: loop body 0:
@[rust_loop_body]
def sum_array_loop.body
  (iter : core.array.iter.IntoIter Std.U32 4#usize) (t : Std.U32) :
  Result (ControlFlow ((core.array.iter.IntoIter Std.U32 4#usize) × Std.U32)
    Std.U32)
  := do
  let (o, iter1) ←
    core.array.iter.IntoIter.Insts.CoreIterTraitsIteratorIterator.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l09_for_array::sum_array]: loop 0:
@[rust_loop]
def sum_array_loop
  (iter : core.array.iter.IntoIter Std.U32 4#usize) (t : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, t1) => sum_array_loop.body iter1 t1)
    (iter, t)
-- [l09_for_array::sum_array]:
def sum_array (a : Array Std.U32 4#usize) : Result Std.U32 := do
  let iter ←
    Array.Insts.CoreIterTraitsCollectIntoIteratorTIntoIter.into_iter a
  sum_array_loop iter 0#u32
-- [l09_for_array::sum_array_ref]: loop body 0:
@[rust_loop_body]
def sum_array_ref_loop.body
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((core.slice.iter.Iter Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ← core.slice.iter.IteratorSliceIter.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l09_for_array::sum_array_ref]: loop 0:
@[rust_loop]
def sum_array_ref_loop
  (iter : core.slice.iter.Iter Std.U32) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, t1) => sum_array_ref_loop.body iter1 t1)
    (iter, t)
-- [l09_for_array::sum_array_ref]:
def sum_array_ref (a : Array Std.U32 4#usize) : Result Std.U32 := do
  let iter ←
    SharedArray.Insts.CoreIterTraitsCollectIntoIteratorSharedIter.into_iter a
  sum_array_ref_loop iter 0#u32
end l09_for_array
```


### L10_iter  —  **CLEAN**

```rust
pub fn count_gt(s: &[u32], k: u32) -> u32 {
    let mut c = 0;
    for x in s.iter() {
        if *x > k {
            c += 1;
        }
    }
    c
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l10_iter
-- [l10_iter::count_gt]: loop body 0:
@[rust_loop_body]
def count_gt_loop.body
  (k : Std.U32) (iter : core.slice.iter.Iter Std.U32) (c : Std.U32) :
  Result (ControlFlow ((core.slice.iter.Iter Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ← core.slice.iter.IteratorSliceIter.next iter
  match o with
  | none => ok (done c)
  | some x =>
    if x > k
    then let c1 ← c + 1#u32
         ok (cont (iter1, c1))
    else ok (cont (iter1, c))
-- [l10_iter::count_gt]: loop 0:
@[rust_loop]
def count_gt_loop
  (iter : core.slice.iter.Iter Std.U32) (k : Std.U32) (c : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, c1) => count_gt_loop.body k iter1 c1)
    (iter, c)
-- [l10_iter::count_gt]:
def count_gt (s : Slice Std.U32) (k : Std.U32) : Result Std.U32 := do
  let iter ← core.slice.Slice.iter s
  count_gt_loop iter k 0#u32
end l10_iter
```


### L11_iter_mut  —  **CLEAN**

```rust
pub fn incr_all(s: &mut [u32]) {
    for x in s.iter_mut() {
        *x = x.wrapping_add(1);
    }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l11_iter_mut
-- [l11_iter_mut::incr_all]: loop body 0:
@[rust_loop_body]
def incr_all_loop.body
  (iter : core.slice.iter.IterMut Std.U32)
  (back : core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32)
  :
  Result (ControlFlow ((core.slice.iter.IterMut Std.U32) ×
    (core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32))
    (core.slice.iter.IterMut Std.U32))
  := do
  let (o, iter1, next_back) ← core.slice.iter.IteratorIterMut.next iter
  match o with
  | none => ok (done (let im := next_back iter1 none
                      back im))
  | some x =>
    let x1 ← lift (core.num.U32.wrapping_add x 1#u32)
    ok (cont (iter1, fun im => let im1 := next_back im (some x1)
                               back im1))
-- [l11_iter_mut::incr_all]: loop 0:
@[rust_loop]
def incr_all_loop
  (iter : core.slice.iter.IterMut Std.U32)
  (back : core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32)
  :
  Result (core.slice.iter.IterMut Std.U32)
  := do
  loop
    (fun (iter1, back1) => incr_all_loop.body iter1 back1)
    (iter, back)
-- [l11_iter_mut::incr_all]:
def incr_all (s : Slice Std.U32) : Result (Slice Std.U32) := do
  let (iter, iter_mut_back) ← core.slice.Slice.iter_mut s
  let back ← incr_all_loop iter (fun im => im)
  ok (iter_mut_back back)
end l11_iter_mut
```


### L11b_index_loop_mut  —  **CLEAN**

```rust
pub fn incr_all_idx(s: &mut [u32]) {
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(1);
        i += 1;
    }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l11b_index_loop_mut
-- [l11b_index_loop_mut::incr_all_idx]: loop body 0:
@[rust_loop_body]
def incr_all_idx_loop.body
  (s : Slice Std.U32) (i : Std.Usize) :
  Result (ControlFlow ((Slice Std.U32) × Std.Usize) (Slice Std.U32))
  := do
  let i1 := Slice.len s
  if i < i1
  then
    let i2 ← Slice.index_usize s i
    let i3 ← lift (core.num.U32.wrapping_add i2 1#u32)
    let s1 ← Slice.update s i i3
    let i4 ← i + 1#usize
    ok (cont (s1, i4))
  else ok (done s)
-- [l11b_index_loop_mut::incr_all_idx]: loop 0:
@[rust_loop]
def incr_all_idx_loop
  (s : Slice Std.U32) (i : Std.Usize) : Result (Slice Std.U32) := do
  loop
    (fun (s1, i1) => incr_all_idx_loop.body s1 i1)
    (s, i)
-- [l11b_index_loop_mut::incr_all_idx]:
@[reducible]
def incr_all_idx (s : Slice Std.U32) : Result (Slice Std.U32) := do
  incr_all_idx_loop s 0#usize
end l11b_index_loop_mut
```


### L12_into_iter  —  **CLEAN**

```rust
pub fn sum_into_iter(v: Vec<u32>) -> u32 {
    let mut t = 0;
    for x in v.into_iter() {
        t += x;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l12_into_iter
-- [l12_into_iter::sum_into_iter]: loop body 0:
@[rust_loop_body]
def sum_into_iter_loop.body
  (iter : alloc.vec.into_iter.IntoIter Std.U32) (t : Std.U32) :
  Result (ControlFlow ((alloc.vec.into_iter.IntoIter Std.U32) × Std.U32)
    Std.U32)
  := do
  let (o, iter1) ← alloc.vec.into_iter.IteratorIntoIter.next iter
  match o with
  | none => ok (done t)
  | some x => let t1 ← t + x
              ok (cont (iter1, t1))
-- [l12_into_iter::sum_into_iter]: loop 0:
@[rust_loop]
def sum_into_iter_loop
  (iter : alloc.vec.into_iter.IntoIter Std.U32) (t : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, t1) => sum_into_iter_loop.body iter1 t1)
    (iter, t)
-- [l12_into_iter::sum_into_iter]:
def sum_into_iter (v : alloc.vec.Vec Std.U32) : Result Std.U32 := do
  let iter ← alloc.vec.IntoIteratorVec.into_iter v
  sum_into_iter_loop iter 0#u32
end l12_into_iter
```


### L13_map  —  **FAIL**

```rust
pub fn doubled(s: &[u32]) -> Vec<u32> {
    s.iter().map(|x| x * 2).collect()
}
```

Exit codes: charon=0, aeneas=0, lean=1.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'map'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'collect'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Lean errors:
```text
../out/L13_map/lean/L13Map.lean:65:2: error: `map` is not a field of structure `core.iter.traits.iterator.Iterator`
../out/L13_map/lean/L13Map.lean:71:2: error: `collect` is not a field of structure `core.iter.traits.iterator.Iterator`
```

Generated Lean:
```lean
namespace l13_map
-- [core::iter::traits::iterator::Iterator::map]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::map"]
axiom core.iter.traits.iterator.Iterator.map.default
  {Self : Type _} {B : Type _} {F : Type _} {Clause0_Item : Type _}
  (IteratorInst : core.iter.traits.iterator.Iterator Self Clause0_Item)
  (opsfunctionFnMutFTupleClause0_ItemBInst : core.ops.function.FnMut F
  Clause0_Item B) :
  Self → F → Result (core.iter.adapters.map.Map Self F)
-- [core::iter::adapters::map::{impl core::iter::traits::iterator::Iterator<B> for core::iter::adapters::map::Map<I, F>}::next]:
@[rust_fun
  "core::iter::adapters::map::{core::iter::traits::iterator::Iterator<core::iter::adapters::map::Map<@I, @F>, @B>}::next"]
axiom core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.next
  {B : Type _} {I : Type _} {F : Type _} {Clause0_Item : Type _}
  (traitsiteratorIteratorInst : core.iter.traits.iterator.Iterator I
  Clause0_Item) (opsfunctionFnMutFTupleClause0_ItemBInst :
  core.ops.function.FnMut F Clause0_Item B) :
  core.iter.adapters.map.Map I F → Result ((Option B) ×
    (core.iter.adapters.map.Map I F))
-- Trait implementation: [core::iter::adapters::map::{impl core::iter::traits::iterator::Iterator<B> for core::iter::adapters::map::Map<I, F>}]
@[reducible, rust_trait_impl
  "core::iter::traits::iterator::Iterator<core::iter::adapters::map::Map<@I, @F>, @B>"]
impl_def core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator {B :
  Type _} {I : Type _} {F : Type _} {Clause0_Item : Type _}
  (traitsiteratorIteratorInst : core.iter.traits.iterator.Iterator I
  Clause0_Item) (opsfunctionFnMutFTupleClause0_ItemBInst :
  core.ops.function.FnMut F Clause0_Item B) :
  core.iter.traits.iterator.Iterator (core.iter.adapters.map.Map I F) B := {
  next := core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.next
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst
  map := fun {B1 : Type} {F1 : Type} (opsfunctionFnMutPTupleBPInst :
    core.ops.function.FnMut F1 B B1) =>
    core.iter.traits.iterator.Iterator.map.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst)
    opsfunctionFnMutPTupleBPInst
  collect := fun {B1 : Type} (collectFromIteratorInst :
    core.iter.traits.collect.FromIterator B1 B) =>
    core.iter.traits.iterator.Iterator.collect.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst)
    collectFromIteratorInst
}
-- [core::ops::arith::{impl core::ops::arith::Mul<u32, u32> for &'_0 u32}::mul]:
@[rust_fun "core::ops::arith::{core::ops::arith::Mul<&'0 u32, u32, u32>}::mul"]
axiom Shared0U32.Insts.CoreOpsArithMulU32U32.mul
  : Std.U32 → Std.U32 → Result Std.U32
-- [l13_map::doubled::{closure}]
@[reducible]
def doubled.closure := Unit
-- [l13_map::doubled::{impl core::ops::function::FnMut<(&'_0 u32,), u32> for l13_map::doubled::{closure}<'_0>}::call_mut]:
def doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32.call_mut
  (c : doubled.closure) (tupled_args : Std.U32) :
  Result (Std.U32 × doubled.closure)
  := do
  let i ← Shared0U32.Insts.CoreOpsArithMulU32U32.mul tupled_args 2#u32
  ok (i, ())
-- [l13_map::doubled::{impl core::ops::function::FnOnce<(&'_0 u32,), u32> for l13_map::doubled::{closure}<'_0>}::call_once]:
def doubled.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32.call_once
  (c : doubled.closure) (i : Std.U32) : Result Std.U32 := do
  let (i1, _) ←
    doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32.call_mut () i
  ok i1
-- Trait implementation: [l13_map::doubled::{impl core::ops::function::FnOnce<(&'_0 u32,), u32> for l13_map::doubled::{closure}<'_0>}]
@[reducible]
def doubled.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32 :
  core.ops.function.FnOnce doubled.closure Std.U32 Std.U32 := {
  call_once :=
    doubled.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32.call_once
}
-- Trait implementation: [l13_map::doubled::{impl core::ops::function::FnMut<(&'_0 u32,), u32> for l13_map::doubled::{closure}<'_0>}]
@[reducible]
def doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32 :
  core.ops.function.FnMut doubled.closure Std.U32 Std.U32 := {
  FnOnceInst := doubled.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32
  call_mut :=
    doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32.call_mut
}
-- [l13_map::doubled]:
def doubled (s : Slice Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  let i ← core.slice.Slice.iter s
  let m ←
    core.iter.traits.iterator.Iterator.map.default
      (core.iter.traits.iterator.IteratorSliceIter Std.U32)
      doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32 i ()
  core.iter.traits.iterator.Iterator.collect.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    (core.iter.traits.iterator.IteratorSliceIter Std.U32)
    doubled.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32)
    (core.iter.traits.collect.FromIteratorVec Std.U32) m
-- ... (1 more lines, see experiments/out/)
```


### L14_filter  —  **CAVEATS**

```rust
pub fn count_even(s: &[u32]) -> usize {
    s.iter().filter(|x| **x % 2 == 0).count()
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'count'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'filter'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l14_filter
-- [core::iter::adapters::filter::Filter]
@[rust_type "core::iter::adapters::filter::Filter"]
axiom core.iter.adapters.filter.Filter (I : Type _) (P : Type _) : Type
-- [core::iter::adapters::filter::{impl core::iter::traits::iterator::Iterator<Clause0_Item> for core::iter::adapters::filter::Filter<I, P>}::count]:
@[rust_fun
  "core::iter::adapters::filter::{core::iter::traits::iterator::Iterator<core::iter::adapters::filter::Filter<@I, @P>, @Clause0_Item>}::count"]
axiom
  core.iter.adapters.filter.Filter.Insts.CoreIterTraitsIteratorIterator.count
  {I : Type _} {P : Type _} {Clause0_Item : Type _} (traitsiteratorIteratorInst
  : core.iter.traits.iterator.Iterator I Clause0_Item)
  (opsfunctionFnMutPTupleSharedClause0_ItemBoolInst : core.ops.function.FnMut P
  Clause0_Item Bool) :
  core.iter.adapters.filter.Filter I P → Result Std.Usize
-- [core::iter::traits::iterator::Iterator::count]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::count"]
axiom core.iter.traits.iterator.Iterator.count.default
  {Self : Type _} {Clause0_Item : Type _} (IteratorInst :
  core.iter.traits.iterator.Iterator Self Clause0_Item) :
  Self → Result Std.Usize
-- [core::iter::traits::iterator::Iterator::filter]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::filter"]
axiom core.iter.traits.iterator.Iterator.filter.default
  {Self : Type _} {P : Type _} {Clause0_Item : Type _} (IteratorInst :
  core.iter.traits.iterator.Iterator Self Clause0_Item)
  (opsfunctionFnMutPTupleSharedClause0_ItemBoolInst : core.ops.function.FnMut P
  Clause0_Item Bool) :
  Self → P → Result (core.iter.adapters.filter.Filter Self P)
-- [core::slice::iter::{impl core::iter::traits::iterator::Iterator<&'a T> for core::slice::iter::Iter<'a, T>}::count]:
@[rust_fun
  "core::slice::iter::{core::iter::traits::iterator::Iterator<core::slice::iter::Iter<'a, @T>, &'a @T>}::count"]
axiom core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.count
  {T : Type _} : core.slice.iter.Iter T → Result Std.Usize
-- [l14_filter::count_even::{closure}]
@[reducible]
def count_even.closure := Unit
-- [l14_filter::count_even::{impl core::ops::function::FnMut<(&'_1 &'_0 u32,), bool> for l14_filter::count_even::{closure}<'_0>}::call_mut]:
def
  count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool.call_mut
  (c : count_even.closure) (tupled_args : Std.U32) :
  Result (Bool × count_even.closure)
  := do
  let i ← tupled_args % 2#u32
  ok (i = 0#u32, ())
-- [l14_filter::count_even::{impl core::ops::function::FnOnce<(&'_1 &'_0 u32,), bool> for l14_filter::count_even::{closure}<'_0>}::call_once]:
def
  count_even.closure.Insts.CoreOpsFunctionFnOnceTupleShared1Shared0U32Bool.call_once
  (c : count_even.closure) (i : Std.U32) : Result Bool := do
  let (b, _) ←
    count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool.call_mut
      () i
  ok b
-- Trait implementation: [l14_filter::count_even::{impl core::ops::function::FnOnce<(&'_1 &'_0 u32,), bool> for l14_filter::count_even::{closure}<'_0>}]
@[reducible]
def count_even.closure.Insts.CoreOpsFunctionFnOnceTupleShared1Shared0U32Bool :
  core.ops.function.FnOnce count_even.closure Std.U32 Bool := {
  call_once :=
    count_even.closure.Insts.CoreOpsFunctionFnOnceTupleShared1Shared0U32Bool.call_once
}
-- Trait implementation: [l14_filter::count_even::{impl core::ops::function::FnMut<(&'_1 &'_0 u32,), bool> for l14_filter::count_even::{closure}<'_0>}]
@[reducible]
def count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool :
  core.ops.function.FnMut count_even.closure Std.U32 Bool := {
  FnOnceInst :=
    count_even.closure.Insts.CoreOpsFunctionFnOnceTupleShared1Shared0U32Bool
  call_mut :=
    count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool.call_mut
}
-- [l14_filter::count_even]:
def count_even (s : Slice Std.U32) : Result Std.Usize := do
  let i ← core.slice.Slice.iter s
  let f ←
    core.iter.traits.iterator.Iterator.filter.default
      (core.iter.traits.iterator.IteratorSliceIter Std.U32)
      count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool i
      ()
  core.iter.adapters.filter.Filter.Insts.CoreIterTraitsIteratorIterator.count
    (core.iter.traits.iterator.IteratorSliceIter Std.U32)
    count_even.closure.Insts.CoreOpsFunctionFnMutTupleShared1Shared0U32Bool f
end l14_filter
```


### L15_enumerate  —  **CLEAN**

```rust
pub fn weighted(s: &[u32]) -> usize {
    let mut t = 0;
    for (i, x) in s.iter().enumerate() {
        t += i * (*x as usize);
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l15_enumerate
-- [l15_enumerate::weighted]: loop body 0:
@[rust_loop_body]
def weighted_loop.body
  (iter : core.iter.adapters.enumerate.Enumerate (core.slice.iter.Iter
  Std.U32)) (t : Std.Usize) :
  Result (ControlFlow ((core.iter.adapters.enumerate.Enumerate
    (core.slice.iter.Iter Std.U32)) × Std.Usize) Std.Usize)
  := do
  let (o, iter1) ←
    core.iter.adapters.enumerate.IteratorEnumerate.next
      (core.iter.traits.iterator.IteratorSliceIter Std.U32) iter
  match o with
  | none => ok (done t)
  | some p =>
    let (i, x) := p
    let i1 ← lift (UScalar.cast .Usize x)
    let i2 ← i * i1
    let t1 ← t + i2
    ok (cont (iter1, t1))
-- [l15_enumerate::weighted]: loop 0:
@[rust_loop]
def weighted_loop
  (iter : core.iter.adapters.enumerate.Enumerate (core.slice.iter.Iter
  Std.U32)) (t : Std.Usize) :
  Result Std.Usize
  := do
  loop
    (fun (iter1, t1) => weighted_loop.body iter1 t1)
    (iter, t)
-- [l15_enumerate::weighted]:
def weighted (s : Slice Std.U32) : Result Std.Usize := do
  let i ← core.slice.Slice.iter s
  let iter ←
    core.iter.traits.iterator.Iterator.enumerate.trait_default
      (core.iter.traits.iterator.IteratorSliceIter Std.U32) i
  weighted_loop iter 0#usize
end l15_enumerate
```


### L16_zip  —  **CAVEATS**

```rust
pub fn dot(a: &[u32], b: &[u32]) -> u32 {
    let mut t = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        t += x * y;
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'zip'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l16_zip
-- [core::ops::arith::{impl core::ops::arith::Mul<&'_0 u32, u32> for &'_1 u32}::mul]:
@[rust_fun
  "core::ops::arith::{core::ops::arith::Mul<&'1 u32, &'0 u32, u32>}::mul"]
axiom Shared1U32.Insts.CoreOpsArithMulShared0U32U32.mul
  : Std.U32 → Std.U32 → Result Std.U32
-- [l16_zip::dot]: loop body 0:
@[rust_loop_body]
def dot_loop.body
  (iter : core.iter.adapters.zip.Zip (core.slice.iter.Iter Std.U32)
  (core.slice.iter.Iter Std.U32)) (t : Std.U32) :
  Result (ControlFlow ((core.iter.adapters.zip.Zip (core.slice.iter.Iter
    Std.U32) (core.slice.iter.Iter Std.U32)) × Std.U32) Std.U32)
  := do
  let (o, iter1) ←
    core.iter.adapters.zip.Zip.Insts.CoreIterTraitsIteratorIteratorPair.next
      (core.iter.traits.iterator.IteratorSliceIter Std.U32)
      (core.iter.traits.iterator.IteratorSliceIter Std.U32) iter
  match o with
  | none => ok (done t)
  | some p =>
    let (x, y) := p
    let i ← Shared1U32.Insts.CoreOpsArithMulShared0U32U32.mul x y
    let t1 ← t + i
    ok (cont (iter1, t1))
-- [l16_zip::dot]: loop 0:
@[rust_loop]
def dot_loop
  (iter : core.iter.adapters.zip.Zip (core.slice.iter.Iter Std.U32)
  (core.slice.iter.Iter Std.U32)) (t : Std.U32) :
  Result Std.U32
  := do
  loop
    (fun (iter1, t1) => dot_loop.body iter1 t1)
    (iter, t)
-- [l16_zip::dot]:
def dot (a : Slice Std.U32) (b : Slice Std.U32) : Result Std.U32 := do
  let i ← core.slice.Slice.iter a
  let i1 ← core.slice.Slice.iter b
  let iter ←
    core.iter.traits.iterator.Iterator.zip.trait_default
      (core.iter.traits.iterator.IteratorSliceIter Std.U32)
      (core.iter.traits.collect.IntoIterator.Blanket
      (core.iter.traits.iterator.IteratorSliceIter Std.U32)) i i1
  dot_loop iter 0#u32
end l16_zip
```


### L17_rev  —  **CLEAN**

```rust
pub fn last_nonzero(n: usize, s: &[u32]) -> Option<usize> {
    for i in (0..n).rev() {
        if s[i] != 0 {
            return Some(i);
        }
    }
    None
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'rev'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l17_rev
-- [l17_rev::last_nonzero]: loop body 0:
@[rust_loop_body]
def last_nonzero_loop.body
  (s : Slice Std.U32)
  (iter : core.iter.adapters.rev.Rev (core.ops.range.Range Std.Usize)) :
  Result (ControlFlow (core.iter.adapters.rev.Rev (core.ops.range.Range
    Std.Usize)) (Option Std.Usize))
  := do
  let (o, iter1) ←
    core.iter.adapters.rev.Rev.Insts.CoreIterTraitsIteratorIterator.next
      (core.ops.range.Range.Insts.DoubleEndedIterator
      core.iter.range.StepUsize) iter
  match o with
  | none => ok (done none)
  | some i =>
    let i1 ← Slice.index_usize s i
    if i1 != 0#u32
    then ok (done o)
    else ok (cont iter1)
-- [l17_rev::last_nonzero]: loop 0:
@[rust_loop]
def last_nonzero_loop
  (iter : core.iter.adapters.rev.Rev (core.ops.range.Range Std.Usize))
  (s : Slice Std.U32) :
  Result (Option Std.Usize)
  := do
  loop
    (fun iter1 => last_nonzero_loop.body s iter1)
    iter
-- [l17_rev::last_nonzero]:
def last_nonzero
  (n : Std.Usize) (s : Slice Std.U32) : Result (Option Std.Usize) := do
  let iter ←
    core.iter.traits.iterator.Iterator.rev.trait_default
      (core.iter.traits.iterator.IteratorRange core.iter.range.StepUsize)
      (core.ops.range.Range.Insts.DoubleEndedIterator
      core.iter.range.StepUsize) { start := 0#usize, «end» := n }
  last_nonzero_loop iter s
end l17_rev
```


### L18_fold  —  **CAVEATS**

```rust
pub fn sum_fold(s: &[u32]) -> u32 {
    s.iter().fold(0, |acc, x| acc + x)
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l18_fold
-- [core::ops::arith::{impl core::ops::arith::Add<&'_0 u32, u32> for u32}::add]:
@[rust_fun "core::ops::arith::{core::ops::arith::Add<u32, &'0 u32, u32>}::add"]
axiom U32.Insts.CoreOpsArithAddShared0U32U32.add
  : Std.U32 → Std.U32 → Result Std.U32
-- [core::slice::iter::{impl core::iter::traits::iterator::Iterator<&'a T> for core::slice::iter::Iter<'a, T>}::fold]:
@[rust_fun
  "core::slice::iter::{core::iter::traits::iterator::Iterator<core::slice::iter::Iter<'a, @T>, &'a @T>}::fold"]
axiom core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.fold
  {T : Type _} {B : Type _} {F : Type _} (opsfunctionFnMutFPairBSharedATBInst :
  core.ops.function.FnMut F (B × T) B) :
  core.slice.iter.Iter T → B → F → Result B
-- [l18_fold::sum_fold::{closure}]
@[reducible]
def sum_fold.closure := Unit
-- [l18_fold::sum_fold::{impl core::ops::function::FnMut<(u32, &'_0 u32), u32> for l18_fold::sum_fold::{closure}<'_0>}::call_mut]:
def sum_fold.closure.Insts.CoreOpsFunctionFnMutPairU32Shared0U32U32.call_mut
  (c : sum_fold.closure) (tupled_args : (Std.U32 × Std.U32)) :
  Result (Std.U32 × sum_fold.closure)
  := do
  let (acc, x) := tupled_args
  let i ← U32.Insts.CoreOpsArithAddShared0U32U32.add acc x
  ok (i, ())
-- [l18_fold::sum_fold::{impl core::ops::function::FnOnce<(u32, &'_0 u32), u32> for l18_fold::sum_fold::{closure}<'_0>}::call_once]:
def sum_fold.closure.Insts.CoreOpsFunctionFnOncePairU32Shared0U32U32.call_once
  (c : sum_fold.closure) (p : (Std.U32 × Std.U32)) : Result Std.U32 := do
  let (i, _) ←
    sum_fold.closure.Insts.CoreOpsFunctionFnMutPairU32Shared0U32U32.call_mut ()
      p
  ok i
-- Trait implementation: [l18_fold::sum_fold::{impl core::ops::function::FnOnce<(u32, &'_0 u32), u32> for l18_fold::sum_fold::{closure}<'_0>}]
@[reducible]
def sum_fold.closure.Insts.CoreOpsFunctionFnOncePairU32Shared0U32U32 :
  core.ops.function.FnOnce sum_fold.closure (Std.U32 × Std.U32) Std.U32 := {
  call_once :=
    sum_fold.closure.Insts.CoreOpsFunctionFnOncePairU32Shared0U32U32.call_once
}
-- Trait implementation: [l18_fold::sum_fold::{impl core::ops::function::FnMut<(u32, &'_0 u32), u32> for l18_fold::sum_fold::{closure}<'_0>}]
@[reducible]
def sum_fold.closure.Insts.CoreOpsFunctionFnMutPairU32Shared0U32U32 :
  core.ops.function.FnMut sum_fold.closure (Std.U32 × Std.U32) Std.U32 := {
  FnOnceInst :=
    sum_fold.closure.Insts.CoreOpsFunctionFnOncePairU32Shared0U32U32
  call_mut :=
    sum_fold.closure.Insts.CoreOpsFunctionFnMutPairU32Shared0U32U32.call_mut
}
-- [l18_fold::sum_fold]:
def sum_fold (s : Slice Std.U32) : Result Std.U32 := do
  let i ← core.slice.Slice.iter s
  core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.fold
    sum_fold.closure.Insts.CoreOpsFunctionFnMutPairU32Shared0U32U32 i 0#u32 ()
end l18_fold
```


### L19_sum  —  **CAVEATS**

```rust
pub fn sum_sum(s: &[u32]) -> u32 {
    s.iter().sum()
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'sum'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l19_sum
-- [core::iter::traits::accum::{impl core::iter::traits::accum::Sum<&'a u32> for u32}::sum]:
@[rust_fun
  "core::iter::traits::accum::{core::iter::traits::accum::Sum<u32, &'a u32>}::sum"]
axiom U32.Insts.CoreIterTraitsAccumSumSharedAU32.sum
  {I : Type _} (iteratorIteratorISharedAU32Inst :
  core.iter.traits.iterator.Iterator I Std.U32) :
  I → Result Std.U32
-- Trait implementation: [core::iter::traits::accum::{impl core::iter::traits::accum::Sum<&'a u32> for u32}]
@[reducible, rust_trait_impl "core::iter::traits::accum::Sum<u32, &'a u32>"]
def U32.Insts.CoreIterTraitsAccumSumSharedAU32 : core.iter.traits.accum.Sum
  Std.U32 Std.U32 := {
  sum := fun {I : Type} (iteratorIteratorPSharedU32Inst :
    core.iter.traits.iterator.Iterator I Std.U32) =>
    U32.Insts.CoreIterTraitsAccumSumSharedAU32.sum
    iteratorIteratorPSharedU32Inst
}
-- [core::iter::traits::iterator::Iterator::sum]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::sum"]
axiom core.iter.traits.iterator.Iterator.sum.default
  {Self : Type _} {S : Type _} {Clause0_Item : Type _} (IteratorInst :
  core.iter.traits.iterator.Iterator Self Clause0_Item) (accumSumInst :
  core.iter.traits.accum.Sum S Clause0_Item) :
  Self → Result S
-- [l19_sum::sum_sum]:
def sum_sum (s : Slice Std.U32) : Result Std.U32 := do
  let i ← core.slice.Slice.iter s
  core.iter.traits.iterator.Iterator.sum.default
    (core.iter.traits.iterator.IteratorSliceIter Std.U32)
    U32.Insts.CoreIterTraitsAccumSumSharedAU32 i
end l19_sum
```


### L20_collect  —  **CLEAN**

```rust
pub fn range_vec(n: u32) -> Vec<u32> {
    (0..n).collect()
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'collect'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l20_collect
-- [l20_collect::range_vec]:
def range_vec (n : Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  core.iter.traits.iterator.Iterator.collect.default
    (core.iter.traits.iterator.IteratorRange core.iter.range.StepU32)
    (core.iter.traits.collect.FromIteratorVec Std.U32)
    { start := 0#u32, «end» := n }
end l20_collect
```


### L21_closure_capture_iter  —  **FAIL**

```rust
pub fn add_k(s: &[u32], k: u32) -> Vec<u32> {
    s.iter().map(|x| x + k).collect()
}
pub fn any_gt(s: &[u32], k: u32) -> bool {
    s.iter().any(|x| *x > k)
}
```

Exit codes: charon=0, aeneas=0, lean=1.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'map'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'collect'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::traits::iterator::Iterator', could not find the information for item 'any'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Lean errors:
```text
../out/L21_closure_capture_iter/lean/L21ClosureCaptureIter.lean:77:2: error: `map` is not a field of structure `core.iter.traits.iterator.Iterator`
../out/L21_closure_capture_iter/lean/L21ClosureCaptureIter.lean:83:2: error: `collect` is not a field of structure `core.iter.traits.iterator.Iterator`
../out/L21_closure_capture_iter/lean/L21ClosureCaptureIter.lean:89:2: error: `any` is not a field of structure `core.iter.traits.iterator.Iterator`
```

Generated Lean:
```lean
namespace l21_closure_capture_iter
-- [core::iter::traits::iterator::Iterator::any]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::any"]
axiom core.iter.traits.iterator.Iterator.any.default
  {Self : Type _} {F : Type _} {Clause0_Item : Type _} (IteratorInst :
  core.iter.traits.iterator.Iterator Self Clause0_Item)
  (opsfunctionFnMutFTupleClause0_ItemBoolInst : core.ops.function.FnMut F
  Clause0_Item Bool) :
  Self → F → Result (Bool × Self)
-- [core::iter::traits::iterator::Iterator::map]:
@[trait_default, rust_fun "core::iter::traits::iterator::Iterator::map"]
axiom core.iter.traits.iterator.Iterator.map.default
  {Self : Type _} {B : Type _} {F : Type _} {Clause0_Item : Type _}
  (IteratorInst : core.iter.traits.iterator.Iterator Self Clause0_Item)
  (opsfunctionFnMutFTupleClause0_ItemBInst : core.ops.function.FnMut F
  Clause0_Item B) :
  Self → F → Result (core.iter.adapters.map.Map Self F)
-- [core::iter::adapters::map::{impl core::iter::traits::iterator::Iterator<B> for core::iter::adapters::map::Map<I, F>}::next]:
@[rust_fun
  "core::iter::adapters::map::{core::iter::traits::iterator::Iterator<core::iter::adapters::map::Map<@I, @F>, @B>}::next"]
axiom core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.next
  {B : Type _} {I : Type _} {F : Type _} {Clause0_Item : Type _}
  (traitsiteratorIteratorInst : core.iter.traits.iterator.Iterator I
  Clause0_Item) (opsfunctionFnMutFTupleClause0_ItemBInst :
  core.ops.function.FnMut F Clause0_Item B) :
  core.iter.adapters.map.Map I F → Result ((Option B) ×
    (core.iter.adapters.map.Map I F))
-- Trait implementation: [core::iter::adapters::map::{impl core::iter::traits::iterator::Iterator<B> for core::iter::adapters::map::Map<I, F>}]
@[reducible, rust_trait_impl
  "core::iter::traits::iterator::Iterator<core::iter::adapters::map::Map<@I, @F>, @B>"]
impl_def core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator {B :
  Type _} {I : Type _} {F : Type _} {Clause0_Item : Type _}
  (traitsiteratorIteratorInst : core.iter.traits.iterator.Iterator I
  Clause0_Item) (opsfunctionFnMutFTupleClause0_ItemBInst :
  core.ops.function.FnMut F Clause0_Item B) :
  core.iter.traits.iterator.Iterator (core.iter.adapters.map.Map I F) B := {
  next := core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.next
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst
  map := fun {B1 : Type} {F1 : Type} (opsfunctionFnMutPTupleBPInst :
    core.ops.function.FnMut F1 B B1) =>
    core.iter.traits.iterator.Iterator.map.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst)
    opsfunctionFnMutPTupleBPInst
  collect := fun {B1 : Type} (collectFromIteratorInst :
    core.iter.traits.collect.FromIterator B1 B) =>
    core.iter.traits.iterator.Iterator.collect.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst)
    collectFromIteratorInst
  any := fun {F1 : Type} (opsfunctionFnMutPTupleFBoolInst :
    core.ops.function.FnMut F1 B Bool) =>
    core.iter.traits.iterator.Iterator.any.default
    (core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator
    traitsiteratorIteratorInst opsfunctionFnMutFTupleClause0_ItemBInst)
    opsfunctionFnMutPTupleFBoolInst
}
-- [core::slice::iter::{impl core::iter::traits::iterator::Iterator<&'a T> for core::slice::iter::Iter<'a, T>}::any]:
@[rust_fun
  "core::slice::iter::{core::iter::traits::iterator::Iterator<core::slice::iter::Iter<'a, @T>, &'a @T>}::any"]
axiom core.slice.iter.Iter.Insts.CoreIterTraitsIteratorIteratorSharedAT.any
  {T : Type _} {F : Type _} (opsfunctionFnMutFTupleSharedATBoolInst :
  core.ops.function.FnMut F T Bool) :
  core.slice.iter.Iter T → F → Result (Bool × (core.slice.iter.Iter T))
-- [l21_closure_capture_iter::add_k::{closure}]
@[reducible]
def add_k.closure := Std.U32
-- [l21_closure_capture_iter::add_k::{impl core::ops::function::FnMut<(&'_0 u32,), u32> for l21_closure_capture_iter::add_k::{closure}<'_0, '_1>}::call_mut]:
def add_k.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32.call_mut
  (c : add_k.closure) (tupled_args : Std.U32) :
  Result (Std.U32 × add_k.closure)
  := do
  let i ← SharedU32.Insts.CoreOpsArithAddU32U32.add tupled_args c
  ok (i, c)
-- [l21_closure_capture_iter::add_k::{impl core::ops::function::FnOnce<(&'_0 u32,), u32> for l21_closure_capture_iter::add_k::{closure}<'_0, '_1>}::call_once]:
def add_k.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32.call_once
  (c : add_k.closure) (i : Std.U32) : Result Std.U32 := do
  let (i1, _) ←
    add_k.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32.call_mut c i
  ok i1
-- Trait implementation: [l21_closure_capture_iter::add_k::{impl core::ops::function::FnOnce<(&'_0 u32,), u32> for l21_closure_capture_iter::add_k::{closure}<'_0, '_1>}]
@[reducible]
def add_k.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32 :
  core.ops.function.FnOnce add_k.closure Std.U32 Std.U32 := {
  call_once :=
    add_k.closure.Insts.CoreOpsFunctionFnOnceTupleShared0U32U32.call_once
}
-- Trait implementation: [l21_closure_capture_iter::add_k::{impl core::ops::function::FnMut<(&'_0 u32,), u32> for l21_closure_capture_iter::add_k::{closure}<'_0, '_1>}]
@[reducible]
def add_k.closure.Insts.CoreOpsFunctionFnMutTupleShared0U32U32 :
-- ... (56 more lines, see experiments/out/)
```


### L22_loop_mut_borrow  —  **CLEAN**

```rust
pub fn zero_out(v: &mut Vec<u32>) {
    let mut i = 0;
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
pub struct Counter { pub n: u32 }
pub fn bump(c: &mut Counter, k: u32) {
    let mut i = 0;
    while i < k {
        c.n = c.n.wrapping_add(1);
        i += 1;
    }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l22_loop_mut_borrow
-- [l22_loop_mut_borrow::zero_out]: loop body 0:
@[rust_loop_body]
def zero_out_loop.body
  (v : alloc.vec.Vec Std.U32) (i : Std.Usize) :
  Result (ControlFlow ((alloc.vec.Vec Std.U32) × Std.Usize) (alloc.vec.Vec
    Std.U32))
  := do
  let i1 := alloc.vec.Vec.len v
  if i < i1
  then
    let (_, index_mut_back) ←
      alloc.vec.Vec.index_mut (core.slice.index.SliceIndexUsizeSlice Std.U32) v
        i
    let i2 ← i + 1#usize
    let v1 := index_mut_back 0#u32
    ok (cont (v1, i2))
  else ok (done v)
-- [l22_loop_mut_borrow::zero_out]: loop 0:
@[rust_loop]
def zero_out_loop
  (v : alloc.vec.Vec Std.U32) (i : Std.Usize) :
  Result (alloc.vec.Vec Std.U32)
  := do
  loop
    (fun (v1, i1) => zero_out_loop.body v1 i1)
    (v, i)
-- [l22_loop_mut_borrow::zero_out]:
@[reducible]
def zero_out (v : alloc.vec.Vec Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  zero_out_loop v 0#usize
-- [l22_loop_mut_borrow::Counter]
structure Counter where
  n : Std.U32
-- [l22_loop_mut_borrow::bump]: loop body 0:
@[rust_loop_body]
def bump_loop.body
  (k : Std.U32) (c : Counter) (i : Std.U32) :
  Result (ControlFlow (Counter × Std.U32) Counter)
  := do
  if i < k
  then
    let i1 ← lift (core.num.U32.wrapping_add c.n 1#u32)
    let i2 ← i + 1#u32
    ok (cont ({ n := i1 }, i2))
  else ok (done c)
-- [l22_loop_mut_borrow::bump]: loop 0:
@[rust_loop]
def bump_loop (c : Counter) (k : Std.U32) (i : Std.U32) : Result Counter := do
  loop
    (fun (c1, i1) => bump_loop.body k c1 i1)
    (c, i)
-- [l22_loop_mut_borrow::bump]:
@[reducible]
def bump (c : Counter) (k : Std.U32) : Result Counter := do
  bump_loop c k 0#u32
end l22_loop_mut_borrow
```


### L23_loop_return_borrow  —  **CLEAN**

```rust
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
// Loop that walks a list and returns a mutable borrow into it.
pub fn nth_mut<T>(mut l: &mut List<T>, mut i: u32) -> Option<&mut T> {
    while let List::Cons(x, tl) = l {
        if i == 0 {
            return Some(x);
        }
        i -= 1;
        l = tl;
    }
    None
}
// Loop that returns a shared borrow into a slice.
pub fn first_ge(s: &[u32], k: u32) -> Option<&u32> {
    let mut i = 0;
    while i < s.len() {
        if s[i] >= k {
            return Some(&s[i]);
        }
        i += 1;
    }
    None
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l23_loop_return_borrow
-- [l23_loop_return_borrow::List]
@[discriminant isize]
inductive List (T : Type _) where
| Cons : T → List T → List T
| Nil : List T
-- [l23_loop_return_borrow::nth_mut]: loop 0:
@[rust_loop]
def nth_mut_loop
  {T : Type _} (l : List T) (i : Std.U32) :
  Result ((Option T) × (Option T → List T))
  := do
  match l with
  | List.Cons x tl =>
    if i = 0#u32
    then
      ok (some x,
        fun o => let t := match o with
                          | some t1 => t1
                          | _ => x
                 List.Cons t tl)
    else
      let i1 ← i - 1#u32
      let (o, back) ← nth_mut_loop tl i1
      let back1 := fun o1 => let l1 := back o1
                             List.Cons x l1
      ok (o, back1)
  | List.Nil => ok (none, fun o => List.Nil)
partial_fixpoint
-- [l23_loop_return_borrow::nth_mut]:
@[reducible]
def nth_mut
  {T : Type _} (l : List T) (i : Std.U32) :
  Result ((Option T) × (Option T → List T))
  := do
  nth_mut_loop l i
-- [l23_loop_return_borrow::first_ge]: loop body 0:
@[rust_loop_body]
def first_ge_loop.body
  (s : Slice Std.U32) (k : Std.U32) (i : Std.Usize) :
  Result (ControlFlow Std.Usize (Option Std.U32))
  := do
  let i1 := Slice.len s
  if i < i1
  then
    let i2 ← Slice.index_usize s i
    if i2 >= k
    then ok (done (some i2))
    else let i3 ← i + 1#usize
         ok (cont i3)
  else ok (done none)
-- [l23_loop_return_borrow::first_ge]: loop 0:
@[rust_loop]
def first_ge_loop
  (s : Slice Std.U32) (k : Std.U32) (i : Std.Usize) :
  Result (Option Std.U32)
  := do
  loop
    (fun i1 => first_ge_loop.body s k i1)
    i
-- [l23_loop_return_borrow::first_ge]:
@[reducible]
def first_ge (s : Slice Std.U32) (k : Std.U32) : Result (Option Std.U32) := do
  first_ge_loop s k 0#usize
end l23_loop_return_borrow
```


### L24_while_let_vec_pop  —  **CAVEATS**

```rust
pub fn drain_sum(mut v: Vec<u32>) -> u32 {
    let mut t: u32 = 0;
    while let Some(x) = v.pop() {
        t = t.wrapping_add(x);
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l24_while_let_vec_pop
-- [alloc::vec::{alloc::vec::Vec<T>}::pop]:
@[rust_fun "alloc::vec::{alloc::vec::Vec<@T>}::pop"]
axiom alloc.vec.Vec.pop
  {T : Type _} (A : Type _) :
  alloc.vec.Vec T → Result ((Option T) × (alloc.vec.Vec T))
-- [l24_while_let_vec_pop::drain_sum]: loop body 0:
@[rust_loop_body]
def drain_sum_loop.body
  (v : alloc.vec.Vec Std.U32) (t : Std.U32) :
  Result (ControlFlow ((alloc.vec.Vec Std.U32) × Std.U32) Std.U32)
  := do
  let (o, v1) ← alloc.vec.Vec.pop alloc.alloc.Global v
  match o with
  | none => ok (done t)
  | some x =>
    let t1 ← lift (core.num.U32.wrapping_add t x)
    ok (cont (v1, t1))
-- [l24_while_let_vec_pop::drain_sum]: loop 0:
@[rust_loop]
def drain_sum_loop
  (v : alloc.vec.Vec Std.U32) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (v1, t1) => drain_sum_loop.body v1 t1)
    (v, t)
-- [l24_while_let_vec_pop::drain_sum]:
@[reducible]
def drain_sum (v : alloc.vec.Vec Std.U32) : Result Std.U32 := do
  drain_sum_loop v 0#u32
end l24_while_let_vec_pop
```


### L25_custom_iterator  —  **CLEAN**

```rust
pub struct Countdown { pub n: u32 }
impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n == 0 { None } else { self.n -= 1; Some(self.n) }
    }
}
pub fn total(n: u32) -> u32 {
    let mut t: u32 = 0;
    for x in (Countdown { n }) {
        t = t.wrapping_add(x);
    }
    t
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l25_custom_iterator
-- [l25_custom_iterator::Countdown]
structure Countdown where
  n : Std.U32
-- [l25_custom_iterator::{impl core::iter::traits::iterator::Iterator<u32> for l25_custom_iterator::Countdown}::next]:
def Countdown.Insts.CoreIterTraitsIteratorIteratorU32.next
  (self : Countdown) : Result ((Option Std.U32) × Countdown) := do
  if self.n = 0#u32
  then ok (none, self)
  else let i ← self.n - 1#u32
       ok (some i, { n := i })
-- Trait implementation: [l25_custom_iterator::{impl core::iter::traits::iterator::Iterator<u32> for l25_custom_iterator::Countdown}]
@[reducible]
def Countdown.Insts.CoreIterTraitsIteratorIteratorU32 :
  core.iter.traits.iterator.Iterator Countdown Std.U32 := {
  next := Countdown.Insts.CoreIterTraitsIteratorIteratorU32.next
}
-- [l25_custom_iterator::total]: loop body 0:
@[rust_loop_body]
def total_loop.body
  (iter : Countdown) (t : Std.U32) :
  Result (ControlFlow (Countdown × Std.U32) Std.U32)
  := do
  let (o, iter1) ←
    Countdown.Insts.CoreIterTraitsIteratorIteratorU32.next iter
  match o with
  | none => ok (done t)
  | some x =>
    let t1 ← lift (core.num.U32.wrapping_add t x)
    ok (cont (iter1, t1))
-- [l25_custom_iterator::total]: loop 0:
@[rust_loop]
def total_loop (iter : Countdown) (t : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, t1) => total_loop.body iter1 t1)
    (iter, t)
-- [l25_custom_iterator::total]:
@[reducible]
def total (n : Std.U32) : Result Std.U32 := do
  total_loop { n } 0#u32
end l25_custom_iterator
```


### L26_infinite_loop  —  **FAIL**

```rust
pub fn spin() -> ! {
    loop {}
}
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] (Infinite) loops which do not contain breaks are not supported yet
[Warn] Could not translate the body of function 'l26_infinite_loop::spin because of previous error
```

Generated Lean:
```lean
namespace l26_infinite_loop
-- [l26_infinite_loop::spin]:
def spin : Result Never := do
  sorry
end l26_infinite_loop
```


### L27_loop_exit_only_return  —  **CLEAN**

```rust
pub fn find_zero(s: &[u32]) -> usize {
    let mut i = 0;
    loop {
        if i >= s.len() || s[i] == 0 {
            return i;
        }
        i += 1;
    }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l27_loop_exit_only_return
-- [l27_loop_exit_only_return::find_zero]: loop body 0:
@[rust_loop_body]
def find_zero_loop.body
  (s : Slice Std.U32) (i : Std.Usize) :
  Result (ControlFlow Std.Usize Std.Usize)
  := do
  let i1 := Slice.len s
  if i >= i1
  then ok (done i)
  else
    let i2 ← Slice.index_usize s i
    if i2 = 0#u32
    then ok (done i)
    else let i3 ← i + 1#usize
         ok (cont i3)
-- [l27_loop_exit_only_return::find_zero]: loop 0:
@[rust_loop]
def find_zero_loop (s : Slice Std.U32) (i : Std.Usize) : Result Std.Usize := do
  loop
    (fun i1 => find_zero_loop.body s i1)
    i
-- [l27_loop_exit_only_return::find_zero]:
@[reducible]
def find_zero (s : Slice Std.U32) : Result Std.Usize := do
  find_zero_loop s 0#usize
end l27_loop_exit_only_return
```


### L28_return_and_break_then_code  —  **CLEAN**

```rust
pub fn f(s: &[u32]) -> u32 {
    let mut i = 0;
    let mut r = 0;
    while i < s.len() {
        if s[i] == 0 {
            return 0;
        }
        if s[i] == 1 {
            break;
        }
        r += s[i];
        i += 1;
    }
    r + 1
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace l28_return_and_break_then_code
-- [l28_return_and_break_then_code::f]: loop body 0:
@[rust_loop_body]
def f_loop.body
  (s : Slice Std.U32) (i : Std.Usize) (r : Std.U32) :
  Result (ControlFlow (Std.Usize × Std.U32) Std.U32)
  := do
  let i1 := Slice.len s
  if i < i1
  then
    let i2 ← Slice.index_usize s i
    if i2 = 0#u32
    then ok (done 0#u32)
    else
      if i2 = 1#u32
      then let i3 ← r + 1#u32
           ok (done i3)
      else let r1 ← r + i2
           let i3 ← i + 1#usize
           ok (cont (i3, r1))
  else let i2 ← r + 1#u32
       ok (done i2)
-- [l28_return_and_break_then_code::f]: loop 0:
@[rust_loop]
def f_loop
  (s : Slice Std.U32) (i : Std.Usize) (r : Std.U32) : Result Std.U32 := do
  loop
    (fun (i1, r1) => f_loop.body s i1 r1)
    (i, r)
-- [l28_return_and_break_then_code::f]:
@[reducible]
def f (s : Slice Std.U32) : Result Std.U32 := do
  f_loop s 0#usize 0#u32
end l28_return_and_break_then_code
```


### L29_two_loops_with_return  —  **FAIL**

```rust
pub fn f(s: &[u32]) -> Option<usize> {
    let mut i = 0;
    while i < s.len() {
        if s[i] == 0 {
            return Some(i);
        }
        i += 1;
    }
    let mut j = 0;
    while j < s.len() {
        if s[j] == 1 {
            return Some(j);
        }
        j += 1;
    }
    None
}
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Breaks to outer loops are not supported yet
[Error] Ignoring the body of 'l29_two_loops_with_return::f' because of previous error
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace l29_two_loops_with_return
-- [l29_two_loops_with_return::f]:
axiom f : Slice Std.U32 → Result (Option Std.Usize)
end l29_two_loops_with_return
```


### L30_for_range_count  —  **CLEAN**

```rust
pub fn count_range(n: u32) -> u32 {
    let mut c = 0;
    for _i in 0..n {
        c += 1;
    }
    c
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
[Warn] When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'backward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field.
```

Generated Lean:
```lean
namespace l30_for_range_count
-- [l30_for_range_count::count_range]: loop body 0:
@[rust_loop_body]
def count_range_loop.body
  (iter : core.ops.range.Range Std.U32) (c : Std.U32) :
  Result (ControlFlow ((core.ops.range.Range Std.U32) × Std.U32) Std.U32)
  := do
  let (o, iter1) ←
    core.iter.range.IteratorRange.next core.iter.range.StepU32 iter
  match o with
  | none => ok (done c)
  | some _ => let c1 ← c + 1#u32
              ok (cont (iter1, c1))
-- [l30_for_range_count::count_range]: loop 0:
@[rust_loop]
def count_range_loop
  (iter : core.ops.range.Range Std.U32) (c : Std.U32) : Result Std.U32 := do
  loop
    (fun (iter1, c1) => count_range_loop.body iter1 c1)
    (iter, c)
-- [l30_for_range_count::count_range]:
@[reducible]
def count_range (n : Std.U32) : Result Std.U32 := do
  count_range_loop { start := 0#u32, «end» := n } 0#u32
end l30_for_range_count
```


# Appendix B — Other feature experiments

### A01_arith  —  **CLEAN**

```rust
pub fn add(a: u32, b: u32) -> u32 { a + b }
pub fn sub(a: i32, b: i32) -> i32 { a - b }
pub fn mul(a: u64, b: u64) -> u64 { a * b }
pub fn div(a: u8, b: u8) -> u8 { a / b }
pub fn rem(a: i16, b: i16) -> i16 { a % b }
pub fn neg(a: i32) -> i32 { -a }
pub fn big(a: u128, b: i128) -> u128 { a + (b as u128) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace a01_arith
-- [a01_arith::add]:
def add (a : Std.U32) (b : Std.U32) : Result Std.U32 := do
  a + b
-- [a01_arith::sub]:
def sub (a : Std.I32) (b : Std.I32) : Result Std.I32 := do
  a - b
-- [a01_arith::mul]:
def mul (a : Std.U64) (b : Std.U64) : Result Std.U64 := do
  a * b
-- [a01_arith::div]:
def div (a : Std.U8) (b : Std.U8) : Result Std.U8 := do
  a / b
-- [a01_arith::rem]:
def rem (a : Std.I16) (b : Std.I16) : Result Std.I16 := do
  a % b
-- [a01_arith::neg]:
def neg (a : Std.I32) : Result Std.I32 := do
  -. a
-- [a01_arith::big]:
def big (a : Std.U128) (b : Std.I128) : Result Std.U128 := do
  let i ← lift (IScalar.hcast .U128 b)
  a + i
end a01_arith
```


### A02_wrapping_checked  —  **CLEAN**

```rust
pub fn w(a: u32, b: u32) -> u32 { a.wrapping_add(b).wrapping_mul(3) }
pub fn c(a: u32, b: u32) -> Option<u32> { a.checked_add(b) }
pub fn o(a: u8, b: u8) -> (u8, bool) { a.overflowing_add(b) }
pub fn s(a: u8, b: u8) -> u8 { a.saturating_sub(b) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace a02_wrapping_checked
-- [a02_wrapping_checked::w]:
def w (a : Std.U32) (b : Std.U32) : Result Std.U32 := do
  let i ← lift (core.num.U32.wrapping_add a b)
  ok (core.num.U32.wrapping_mul i 3#u32)
-- [a02_wrapping_checked::c]:
def c (a : Std.U32) (b : Std.U32) : Result (Option Std.U32) := do
  ok (U32.checked_add a b)
-- [a02_wrapping_checked::o]:
def o (a : Std.U8) (b : Std.U8) : Result (Std.U8 × Bool) := do
  ok (core.num.U8.overflowing_add a b)
-- [a02_wrapping_checked::s]:
def s (a : Std.U8) (b : Std.U8) : Result Std.U8 := do
  ok (core.num.U8.saturating_sub a b)
end a02_wrapping_checked
```


### A03_casts  —  **FAIL**

```rust
pub fn c1(x: u32) -> u8 { x as u8 }
pub fn c2(x: i32) -> u64 { x as u64 }
pub fn c3(b: bool) -> u8 { b as u8 }
pub fn c4(x: u8) -> char { x as char }
pub fn c5(x: u16) -> u32 { u32::from(x) }
pub fn c6(x: u32) -> Result<u8, core::num::TryFromIntError> { u8::try_from(x) }
```

Exit codes: charon=0, aeneas=1, lean=1.

Aeneas messages:
```text
[Error] Unreachable
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Lean errors:
```text
../out/A03_casts/lean/A03Casts.lean:58:6: error: unexpected token '/--'; expected ')', '↑', '↥', '⇑' or term
```

Generated Lean:
```lean
namespace a03_casts
-- [core::convert::num::{impl core::convert::TryFrom<u32, core::num::error::TryFromIntError> for u8}::try_from]:
@[rust_fun
  "core::convert::num::{core::convert::TryFrom<u8, u32, core::num::error::TryFromIntError>}::try_from"]
axiom U8.Insts.CoreConvertTryFromU32TryFromIntError.try_from
  :
  Std.U32 → Result (core.result.Result Std.U8 core.num.error.TryFromIntError)
-- [a03_casts::c1]:
def c1 (x : Std.U32) : Result Std.U8 := do
  ok (UScalar.cast .U8 x)
-- [a03_casts::c2]:
def c2 (x : Std.I32) : Result Std.U64 := do
  ok (IScalar.hcast .U64 x)
-- [a03_casts::c3]:
def c3 (b : Bool) : Result Std.U8 := do
  ok (UScalar.cast_fromBool .U8 b)
-- [a03_casts::c4]:
def c4 (x : Std.U8) : Result Char := do
  ok (
    -- [a03_casts::c5]:
    def c5 (x : Std.U16) : Result Std.U32 := do
      ok (core.convert.num.FromU32U16.from x)
    -- [a03_casts::c6]:
    def c6
      (x : Std.U32) :
      Result (core.result.Result Std.U8 core.num.error.TryFromIntError)
      := do
      U8.Insts.CoreConvertTryFromU32TryFromIntError.try_from x
end a03_casts
```


### A04_bits  —  **CAVEATS**

```rust
pub fn b(a: u32, b: u32) -> u32 { (a & b) | (a ^ b) | !a }
pub fn sh(a: u32, k: u32) -> u32 { (a << k) >> 1 }
pub fn rot(a: u32) -> u32 { a.rotate_left(3) }
pub fn cnt(a: u64) -> u32 { a.count_ones() }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace a04_bits
-- [core::num::{u64}::count_ones]:
@[rust_fun "core::num::{u64}::count_ones"]
axiom core.num.U64.count_ones : Std.U64 → Result Std.U32
-- [a04_bits::b]:
def b (a : Std.U32) (b1 : Std.U32) : Result Std.U32 := do
  let i ← lift (a &&& b1)
  let i1 ← lift (a ^^^ b1)
  let i2 ← lift (i ||| i1)
  let i3 ← lift (~~~ a)
  ok (i2 ||| i3)
-- [a04_bits::sh]:
def sh (a : Std.U32) (k : Std.U32) : Result Std.U32 := do
  let i ← a <<< k
  i >>> 1#i32
-- [a04_bits::rot]:
def rot (a : Std.U32) : Result Std.U32 := do
  ok (core.num.U32.rotate_left a 3#u32)
-- [a04_bits::cnt]:
def cnt (a : Std.U64) : Result Std.U32 := do
  core.num.U64.count_ones a
end a04_bits
```


### A05_panics  —  **CLEAN**

```rust
pub fn p(x: u32) -> u32 { if x == 0 { panic!("zero") } x }
pub fn a(x: u32) -> u32 { assert!(x > 1); x }
pub fn ae(x: u32) { assert_eq!(x, 1) }
pub fn u(x: Option<u32>) -> u32 { x.unwrap() }
pub fn e(x: Option<u32>) -> u32 { x.expect("none") }
pub fn un() -> u32 { unreachable!() }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace a05_panics
-- [a05_panics::p]:
def p (x : Std.U32) : Result Std.U32 := do
  massert (¬ (x = 0#u32))
  ok x
-- [a05_panics::a]:
def a (x : Std.U32) : Result Std.U32 := do
  massert (x > 1#u32)
  ok x
-- [a05_panics::ae]:
def ae (x : Std.U32) : Result Unit := do
  massert (x = 1#u32)
-- [a05_panics::u]:
def u (x : Option Std.U32) : Result Std.U32 := do
  core.option.Option.unwrap x
-- [a05_panics::e]:
def e (x : Option Std.U32) : Result Std.U32 := do
  core.option.Option.expect x (toStr "none")
-- [a05_panics::un]:
def un : Result Std.U32 := do
  fail panic
end a05_panics
```


### A06_floats  —  **FAIL**

```rust
pub fn f(a: f64, b: f64) -> f64 { a * b + 1.0 }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Invalid inputs for binop
[Warn] Could not translate the body of function 'a06_floats::f because of previous error
```

Generated Lean:
```lean
namespace a06_floats
-- [a06_floats::f]:
def f (a : F64) (b : F64) : Result F64 := do
  sorry
end a06_floats
```


### B01_nested_borrows  —  **CLEAN**

```rust
pub fn incr_nested(x: &mut &mut u32) { **x += 1; }
pub fn use_nested() -> u32 { let mut a = 0; let mut r = &mut a; incr_nested(&mut r); a }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace b01_nested_borrows
-- [b01_nested_borrows::incr_nested]:
def incr_nested (x : Std.U32) : Result (Std.U32 × (Std.U32 → Std.U32)) := do
  let x1 ← x + 1#u32
  ok (x1, fun x2 => x2)
-- [b01_nested_borrows::use_nested]:
def use_nested : Result Std.U32 := do
  let (a, incr_nested_back) ← incr_nested 0#u32
  ok (incr_nested_back a)
end b01_nested_borrows
```


### B02_struct_with_lifetime  —  **CLEAN**

```rust
pub struct View<'a> { pub data: &'a mut [u32], pub pos: usize }
impl<'a> View<'a> {
    pub fn set(&mut self, x: u32) { self.data[self.pos] = x; }
}
pub fn use_view(v: &mut [u32]) { let mut w = View { data: v, pos: 0 }; w.set(7); }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace b02_struct_with_lifetime
-- [b02_struct_with_lifetime::View]
structure View where
  data : Slice Std.U32
  pos : Std.Usize
-- [b02_struct_with_lifetime::{b02_struct_with_lifetime::View<'a>}::set]:
def View.set
  (self : View) (x : Std.U32) : Result (View × (View → View)) := do
  let s ← Slice.update self.data self.pos x
  let back'a := fun self1 => { self with data := self1.data }
  ok ({ self with data := s }, back'a)
-- [b02_struct_with_lifetime::use_view]:
def use_view (v : Slice Std.U32) : Result (Slice Std.U32) := do
  let (w, set_back) ← View.set { data := v, pos := 0#usize } 7#u32
  ok (set_back w).data
end b02_struct_with_lifetime
```


### B03_return_borrow  —  **CLEAN**

```rust
pub fn choose<'a>(b: bool, x: &'a mut u32, y: &'a mut u32) -> &'a mut u32 { if b { x } else { y } }
pub fn use_choose() -> (u32, u32) {
    let mut x = 0; let mut y = 0;
    { let r = choose(true, &mut x, &mut y); *r = 1; }
    (x, y)
}
pub fn get_mut_field(p: &mut (u32, u32)) -> &mut u32 { &mut p.1 }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace b03_return_borrow
-- [b03_return_borrow::choose]:
def choose
  (b : Bool) (x : Std.U32) (y : Std.U32) :
  Result (Std.U32 × (Std.U32 → (Std.U32 × Std.U32)))
  := do
  if b
  then let back := fun x1 => (x1, y)
       ok (x, back)
  else let back := fun y1 => (x, y1)
       ok (y, back)
-- [b03_return_borrow::use_choose]:
def use_choose : Result (Std.U32 × Std.U32) := do
  let (_, choose_back) ← choose true 0#u32 0#u32
  ok (choose_back 1#u32)
-- [b03_return_borrow::get_mut_field]:
def get_mut_field
  (p : (Std.U32 × Std.U32)) :
  Result (Std.U32 × (Std.U32 → (Std.U32 × Std.U32)))
  := do
  let (i, i1) := p
  let back := fun i2 => (i, i2)
  ok (i1, back)
end b03_return_borrow
```


### B04_generic_with_mut_ref  —  **FAIL**

```rust
pub fn id<T>(x: T) -> T { x }
pub fn use_id() -> u32 { let mut x = 0; *id(&mut x) += 1; x }
```

Exit codes: charon=0, aeneas=0, lean=1.

Lean errors:
```text
../out/B04_generic_with_mut_ref/lean/B04GenericWithMutRef.lean:32:31: error: expected a product type, got
```

Generated Lean:
```lean
namespace b04_generic_with_mut_ref
-- [b04_generic_with_mut_ref::id]:
def id {T : Type _} (x : T) : Result T := do
  ok x
-- [b04_generic_with_mut_ref::use_id]:
def use_id : Result Std.U32 := do
  let (i, id_back) ← id 0#u32
  let i1 ← i + 1#u32
  ok (id_back i1)
end b04_generic_with_mut_ref
```


### B05_two_phase  —  **CLEAN**

```rust
pub fn push_len(v: &mut Vec<usize>) { v.push(v.len()); }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace b05_two_phase
-- [b05_two_phase::push_len]:
def push_len
  (v : alloc.vec.Vec Std.Usize) : Result (alloc.vec.Vec Std.Usize) := do
  let i := alloc.vec.Vec.len v
  alloc.vec.Vec.push v i
end b05_two_phase
```


### B06_option_mut_ref_in_struct  —  **CLEAN**

```rust
pub struct Node { pub v: u32, pub next: Option<Box<Node>> }
pub fn last_mut(n: &mut Node) -> &mut u32 {
    match &mut n.next { None => &mut n.v, Some(nx) => last_mut(nx) }
}
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace b06_option_mut_ref_in_struct
-- [b06_option_mut_ref_in_struct::Node]
inductive Node where
| mk : Std.U32 → Option Node → Node
def Node.v (x : Node) := match x with | Node.mk x1 _ => x1
def Node.next (x : Node) := match x with | Node.mk _ x1 => x1
@[simp]
theorem Node.v._simpLemma_ (v : Std.U32) (next : Option Node) :
  (Node.mk v next).v = v := by rfl
@[simp]
theorem Node.next._simpLemma_ (v : Std.U32) (next : Option Node) :
  (Node.mk v next).next = next := by rfl
-- [b06_option_mut_ref_in_struct::last_mut]:
def last_mut (n : Node) : Result (Std.U32 × (Std.U32 → Node)) := do
  match n.next with
  | none => let back := fun i => Node.mk i none
            ok (n.v, back)
  | some nx =>
    let (i, last_mut_back) ← last_mut nx
    let back := fun i1 => let nx1 := last_mut_back i1
                          Node.mk n.v (some nx1)
    ok (i, back)
partial_fixpoint
end b06_option_mut_ref_in_struct
```


### C01_closure_local  —  **CLEAN**

```rust
pub fn f(x: u32) -> u32 { let add1 = |y: u32| y + 1; add1(x) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace c01_closure_local
-- [c01_closure_local::f::{closure}]
@[reducible]
def f.closure := Unit
-- [c01_closure_local::f::{impl core::ops::function::Fn<(u32,), u32> for c01_closure_local::f::{closure}}::call]:
def f.closure.Insts.CoreOpsFunctionFnTupleU32U32.call
  (c : f.closure) (tupled_args : Std.U32) : Result Std.U32 := do
  tupled_args + 1#u32
-- [c01_closure_local::f]:
def f (x : Std.U32) : Result Std.U32 := do
  f.closure.Insts.CoreOpsFunctionFnTupleU32U32.call () x
end c01_closure_local
```


### C02_closure_generic_fn  —  **CLEAN**

```rust
pub fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 { f(x) }
pub fn use_apply(k: u32) -> u32 { apply(|y| y + k, 3) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace c02_closure_generic_fn
-- [c02_closure_generic_fn::apply]:
def apply
  {F : Type _} (coreopsfunctionFnFTupleU32U32Inst : core.ops.function.Fn F
  Std.U32 Std.U32) (f : F) (x : Std.U32) :
  Result Std.U32
  := do
  coreopsfunctionFnFTupleU32U32Inst.call f x
-- [c02_closure_generic_fn::use_apply::{closure}]
@[reducible]
def use_apply.closure := Std.U32
-- [c02_closure_generic_fn::use_apply::{impl core::ops::function::Fn<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}::call]:
def use_apply.closure.Insts.CoreOpsFunctionFnTupleU32U32.call
  (c : use_apply.closure) (tupled_args : Std.U32) : Result Std.U32 := do
  tupled_args + c
-- [c02_closure_generic_fn::use_apply::{impl core::ops::function::FnMut<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}::call_mut]:
def use_apply.closure.Insts.CoreOpsFunctionFnMutTupleU32U32.call_mut
  (state : use_apply.closure) (args : Std.U32) :
  Result (Std.U32 × use_apply.closure)
  := do
  let i ←
    use_apply.closure.Insts.CoreOpsFunctionFnTupleU32U32.call state args
  ok (i, state)
-- [c02_closure_generic_fn::use_apply::{impl core::ops::function::FnOnce<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}::call_once]:
def use_apply.closure.Insts.CoreOpsFunctionFnOnceTupleU32U32.call_once
  (c : use_apply.closure) (i : Std.U32) : Result Std.U32 := do
  let (i1, _) ←
    use_apply.closure.Insts.CoreOpsFunctionFnMutTupleU32U32.call_mut c i
  ok i1
-- Trait implementation: [c02_closure_generic_fn::use_apply::{impl core::ops::function::FnOnce<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}]
@[reducible]
def use_apply.closure.Insts.CoreOpsFunctionFnOnceTupleU32U32 :
  core.ops.function.FnOnce use_apply.closure Std.U32 Std.U32 := {
  call_once :=
    use_apply.closure.Insts.CoreOpsFunctionFnOnceTupleU32U32.call_once
}
-- Trait implementation: [c02_closure_generic_fn::use_apply::{impl core::ops::function::FnMut<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}]
@[reducible]
def use_apply.closure.Insts.CoreOpsFunctionFnMutTupleU32U32 :
  core.ops.function.FnMut use_apply.closure Std.U32 Std.U32 := {
  FnOnceInst := use_apply.closure.Insts.CoreOpsFunctionFnOnceTupleU32U32
  call_mut := use_apply.closure.Insts.CoreOpsFunctionFnMutTupleU32U32.call_mut
}
-- Trait implementation: [c02_closure_generic_fn::use_apply::{impl core::ops::function::Fn<(u32,), u32> for c02_closure_generic_fn::use_apply::{closure}<'_0>}]
@[reducible]
-- ... (9 more lines, see experiments/out/)
```


### C03_closure_fnmut  —  **FAIL**

```rust
pub fn call_twice<F: FnMut()>(mut f: F) { f(); f(); }
pub fn count() -> u32 { let mut c = 0; call_twice(|| c += 1); c }
```

Exit codes: charon=0, aeneas=0, lean=1.

Lean errors:
```text
../out/C03_closure_fnmut/lean/C03ClosureFnmut.lean:63:15: error: Type mismatch
../out/C03_closure_fnmut/lean/C03ClosureFnmut.lean:72:14: error: Type mismatch
../out/C03_closure_fnmut/lean/C03ClosureFnmut.lean:79:2: error: Type mismatch
```

Generated Lean:
```lean
namespace c03_closure_fnmut
-- [c03_closure_fnmut::call_twice]:
def call_twice
  {F : Type _} (coreopsfunctionFnMutFTupleTupleInst : core.ops.function.FnMut F
  Unit Unit) (f : F) :
  Result Unit
  := do
  let (_, f1) ← coreopsfunctionFnMutFTupleTupleInst.call_mut f ()
  let _ ← coreopsfunctionFnMutFTupleTupleInst.call_mut f1 ()
  ok ()
-- [c03_closure_fnmut::count::{closure}]
@[reducible]
def count.closure := Std.U32
-- [c03_closure_fnmut::count::{impl core::ops::function::FnMut<(), ()> for c03_closure_fnmut::count::{closure}<'_0>}::call_mut]:
def count.closure.Insts.CoreOpsFunctionFnMutTupleTuple.call_mut
  (c : count.closure) (_ : Unit) :
  Result (count.closure × (count.closure → count.closure))
  := do
  let i ← c + 1#u32
  let back := fun c1 => c1
  ok (i, back)
-- [c03_closure_fnmut::count::{impl core::ops::function::FnOnce<(), ()> for c03_closure_fnmut::count::{closure}<'_0>}::call_once]:
def count.closure.Insts.CoreOpsFunctionFnOnceTupleTuple.call_once
  (c : count.closure) (_ : Unit) : Result count.closure := do
  let (c1, call_mut_back) ←
    count.closure.Insts.CoreOpsFunctionFnMutTupleTuple.call_mut c ()
  ok (call_mut_back c1)
-- Trait implementation: [c03_closure_fnmut::count::{impl core::ops::function::FnOnce<(), ()> for c03_closure_fnmut::count::{closure}<'_0>}]
@[reducible]
def count.closure.Insts.CoreOpsFunctionFnOnceTupleTuple :
  core.ops.function.FnOnce count.closure Unit Unit := {
  call_once := count.closure.Insts.CoreOpsFunctionFnOnceTupleTuple.call_once
}
-- Trait implementation: [c03_closure_fnmut::count::{impl core::ops::function::FnMut<(), ()> for c03_closure_fnmut::count::{closure}<'_0>}]
@[reducible]
def count.closure.Insts.CoreOpsFunctionFnMutTupleTuple :
  core.ops.function.FnMut count.closure Unit Unit := {
  FnOnceInst := count.closure.Insts.CoreOpsFunctionFnOnceTupleTuple
  call_mut := count.closure.Insts.CoreOpsFunctionFnMutTupleTuple.call_mut
}
-- [c03_closure_fnmut::count]:
def count : Result Std.U32 := do
  call_twice count.closure.Insts.CoreOpsFunctionFnMutTupleTuple 0#u32
end c03_closure_fnmut
```


### C04_fn_pointer  —  **FAIL**

```rust
fn inc(x: u32) -> u32 { x + 1 }
pub fn apply_ptr(f: fn(u32) -> u32, x: u32) -> u32 { f(x) }
pub fn use_ptr() -> u32 { apply_ptr(inc, 1) }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Arrow types are not supported yet
[Warn] Could not translate the function signature of 'c04_fn_pointer::apply_ptr because of previous error
[Error] Invalid input for unop: fn_def_to_fn_ptr on c04_fn_pointer::inc
[Warn] Could not translate the body of function 'c04_fn_pointer::use_ptr because of previous error
[Error] Function pointers are not supported yet
[Warn] Could not translate the body of function 'c04_fn_pointer::apply_ptr because of previous error
[Error] Internal error: please file an issue
[Warn] Could not translate the function 'c04_fn_pointer::apply_ptr because of previous error.
```

Generated Lean:
```lean
namespace c04_fn_pointer
-- [c04_fn_pointer::inc]:
def inc (x : Std.U32) : Result Std.U32 := do
  x + 1#u32
-- [c04_fn_pointer::use_ptr]:
def use_ptr : Result Std.U32 := do
  sorry
end c04_fn_pointer
```


### C05_closure_fnonce_move  —  **CLEAN**

```rust
pub fn consume<F: FnOnce() -> Vec<u32>>(f: F) -> Vec<u32> { f() }
pub fn mk(v: Vec<u32>) -> Vec<u32> { consume(move || v) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace c05_closure_fnonce_move
-- [c05_closure_fnonce_move::consume]:
def consume
  {F : Type _} (coreopsfunctionFnOnceFTupleVecU32Inst :
  core.ops.function.FnOnce F Unit (alloc.vec.Vec Std.U32)) (f : F) :
  Result (alloc.vec.Vec Std.U32)
  := do
  coreopsfunctionFnOnceFTupleVecU32Inst.call_once f ()
-- [c05_closure_fnonce_move::mk::{closure}]
@[reducible]
def mk.closure := alloc.vec.Vec Std.U32
-- [c05_closure_fnonce_move::mk::{impl core::ops::function::FnOnce<(), alloc::vec::Vec<u32>> for c05_closure_fnonce_move::mk::{closure}}::call_once]:
def mk.closure.Insts.CoreOpsFunctionFnOnceTupleVecU32.call_once
  (c : mk.closure) (_ : Unit) : Result (alloc.vec.Vec Std.U32) := do
  ok c
-- Trait implementation: [c05_closure_fnonce_move::mk::{impl core::ops::function::FnOnce<(), alloc::vec::Vec<u32>> for c05_closure_fnonce_move::mk::{closure}}]
@[reducible]
def mk.closure.Insts.CoreOpsFunctionFnOnceTupleVecU32 :
  core.ops.function.FnOnce mk.closure Unit (alloc.vec.Vec Std.U32) := {
  call_once := mk.closure.Insts.CoreOpsFunctionFnOnceTupleVecU32.call_once
}
-- [c05_closure_fnonce_move::mk]:
def mk (v : alloc.vec.Vec Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  consume mk.closure.Insts.CoreOpsFunctionFnOnceTupleVecU32 v
end c05_closure_fnonce_move
```


### D01_arrays  —  **CLEAN**

```rust
pub fn arr() -> u32 { let mut a = [1u32, 2, 3, 4]; a[2] = 10; a[0] + a[2] + (a.len() as u32) }
pub fn swap01(a: &mut [u32; 2]) { let t = a[0]; a[0] = a[1]; a[1] = t; }
pub fn arr2d(m: [[u8; 2]; 2]) -> u8 { m[1][0] }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace d01_arrays
-- [d01_arrays::arr]:
def arr : Result Std.U32 := do
  let a ←
    Array.update (Array.make 4#usize [ 1#u32, 2#u32, 3#u32, 4#u32 ] : Array
      Std.U32 4#usize) 2#usize 10#u32
  let i ← Array.index_usize a 0#usize
  let i1 ← Array.index_usize a 2#usize
  let i2 ← i + i1
  let s ← lift (Array.to_slice a)
  let i3 := Slice.len s
  let i4 ← lift (UScalar.cast .U32 i3)
  i2 + i4
-- [d01_arrays::swap01]:
def swap01 (a : Array Std.U32 2#usize) : Result (Array Std.U32 2#usize) := do
  let t ← Array.index_usize a 0#usize
  let i ← Array.index_usize a 1#usize
  let a1 ← Array.update a 0#usize i
  Array.update a1 1#usize t
-- [d01_arrays::arr2d]:
def arr2d (m : Array (Array Std.U8 2#usize) 2#usize) : Result Std.U8 := do
  let a ← Array.index_usize m 1#usize
  Array.index_usize a 0#usize
end d01_arrays
```


### D02_slices  —  **CLEAN**

```rust
pub fn first(s: &[u32]) -> u32 { s[0] }
pub fn sub(s: &[u32]) -> &[u32] { &s[1..3] }
pub fn len(s: &[u32]) -> usize { s.len() }
pub fn split(s: &mut [u32]) -> u32 { let (a, b) = s.split_at_mut(1); a[0] + b[0] }
pub fn swap(s: &mut [u32]) { s.swap(0, 1) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace d02_slices
-- [d02_slices::first]:
def first (s : Slice Std.U32) : Result Std.U32 := do
  Slice.index_usize s 0#usize
-- [d02_slices::sub]:
def sub (s : Slice Std.U32) : Result (Slice Std.U32) := do
  core.slice.index.Slice.index (core.slice.index.SliceIndexRangeUsizeSlice
    Std.U32) s { start := 1#usize, «end» := 3#usize }
-- [d02_slices::len]:
def len (s : Slice Std.U32) : Result Std.Usize := do
  ok (Slice.len s)
-- [d02_slices::split]:
def split (s : Slice Std.U32) : Result (Std.U32 × (Slice Std.U32)) := do
  let ((a, b), split_at_mut_back) ← core.slice.Slice.split_at_mut s 1#usize
  let i ← Slice.index_usize a 0#usize
  let i1 ← Slice.index_usize b 0#usize
  let i2 ← i + i1
  let s1 := split_at_mut_back (a, b)
  ok (i2, s1)
-- [d02_slices::swap]:
def swap (s : Slice Std.U32) : Result (Slice Std.U32) := do
  core.slice.Slice.swap s 0#usize 1#usize
end d02_slices
```


### D03_vec  —  **CAVEATS**

```rust
pub fn build() -> Vec<u32> { let mut v = Vec::new(); v.push(1); v.push(2); v[0] = 5; v }
pub fn get(v: &Vec<u32>, i: usize) -> u32 { v[i] }
pub fn pop(v: &mut Vec<u32>) -> Option<u32> { v.pop() }
pub fn mac() -> Vec<u32> { vec![1, 2, 3] }
pub fn zeros(n: usize) -> Vec<u8> { vec![0; n] }
pub fn ins(v: &mut Vec<u32>) { v.insert(0, 9); v.remove(1); }
pub fn ext(v: &mut Vec<u32>, w: &[u32]) { v.extend_from_slice(w) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace d03_vec
-- [core::mem::maybe_uninit::MaybeUninit]
@[rust_type "core::mem::maybe_uninit::MaybeUninit"]
axiom core.mem.maybe_uninit.MaybeUninit (T : Type _) : Type
-- [alloc::vec::{alloc::vec::Vec<T>}::remove]:
@[rust_fun "alloc::vec::{alloc::vec::Vec<@T>}::remove"]
axiom alloc.vec.Vec.remove
  {T : Type _} (A : Type _) :
  alloc.vec.Vec T → Std.Usize → Result (T × (alloc.vec.Vec T))
-- [alloc::vec::{alloc::vec::Vec<T>}::pop]:
@[rust_fun "alloc::vec::{alloc::vec::Vec<@T>}::pop"]
axiom alloc.vec.Vec.pop
  {T : Type _} (A : Type _) :
  alloc.vec.Vec T → Result ((Option T) × (alloc.vec.Vec T))
-- [d03_vec::build]:
def build : Result (alloc.vec.Vec Std.U32) := do
  let v ← alloc.vec.Vec.push (alloc.vec.Vec.new Std.U32) 1#u32
  let v1 ← alloc.vec.Vec.push v 2#u32
  let (_, index_mut_back) ←
    alloc.vec.Vec.index_mut (core.slice.index.SliceIndexUsizeSlice Std.U32) v1
      0#usize
  ok (index_mut_back 5#u32)
-- [d03_vec::get]:
def get (v : alloc.vec.Vec Std.U32) (i : Std.Usize) : Result Std.U32 := do
  alloc.vec.Vec.index (core.slice.index.SliceIndexUsizeSlice Std.U32) v i
-- [d03_vec::pop]:
def pop
  (v : alloc.vec.Vec Std.U32) :
  Result ((Option Std.U32) × (alloc.vec.Vec Std.U32))
  := do
  alloc.vec.Vec.pop alloc.alloc.Global v
-- [d03_vec::mac]:
def mac : Result (alloc.vec.Vec Std.U32) := do
  let y ←
    lift (Std.Array.to_slice (Array.make 3#usize [ 1#u32, 2#u32, 3#u32 ] :
      Array Std.U32 3#usize))
  ok (alloc.slice.Slice.into_vec y)
-- [d03_vec::zeros]:
def zeros (n : Std.Usize) : Result (alloc.vec.Vec Std.U8) := do
  alloc.vec.from_elem core.clone.CloneU8 0#u8 n
-- [d03_vec::ins]:
def ins (v : alloc.vec.Vec Std.U32) : Result (alloc.vec.Vec Std.U32) := do
  let v1 ← alloc.vec.Vec.insert v 0#usize 9#u32
  let (_, v2) ← alloc.vec.Vec.remove alloc.alloc.Global v1 1#usize
  ok v2
-- ... (7 more lines, see experiments/out/)
```


### D04_strings  —  **CAVEATS**

```rust
pub fn s_len(s: &str) -> usize { s.len() }
pub fn mk() -> String { let mut s = String::new(); s.push_str("ab"); s }
pub fn eq(a: &str, b: &str) -> bool { a == b }
pub fn lit() -> &'static str { "hello" }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace d04_strings
-- [core::str::{str}::len]:
@[rust_fun "core::str::{str}::len"]
axiom core.str.Str.len : Str → Result Std.Usize
-- [core::str::traits::{impl core::cmp::PartialEq<str> for str}::eq]:
@[rust_fun "core::str::traits::{core::cmp::PartialEq<str, str>}::eq"]
axiom Str.Insts.CoreCmpPartialEqStr.eq : Str → Str → Result Bool
-- Trait implementation: [core::str::traits::{impl core::cmp::PartialEq<str> for str}]
@[reducible, rust_trait_impl "core::cmp::PartialEq<str, str>"]
def Str.Insts.CoreCmpPartialEqStr : core.cmp.PartialEq Str Str := {
  eq := Str.Insts.CoreCmpPartialEqStr.eq
}
-- [alloc::string::{alloc::string::String}::new]:
@[rust_fun "alloc::string::{alloc::string::String}::new"]
axiom alloc.string.String.new : Result String
-- [alloc::string::{alloc::string::String}::push_str]:
@[rust_fun "alloc::string::{alloc::string::String}::push_str"]
axiom alloc.string.String.push_str : String → Str → Result String
-- [d04_strings::s_len]:
def s_len (s : Str) : Result Std.Usize := do
  core.str.Str.len s
-- [d04_strings::mk]:
def mk : Result String := do
  let s ← alloc.string.String.new
  alloc.string.String.push_str s (toStr "ab")
-- [d04_strings::eq]:
def eq (a : Str) (b : Str) : Result Bool := do
  Str.Insts.CoreCmpPartialEqStr.eq a b
-- [d04_strings::lit]:
def lit : Result Str := do
  ok (toStr "hello")
end d04_strings
```


### D05_option_result  —  **CAVEATS**

```rust
pub fn div(a: u32, b: u32) -> Result<u32, ()> { if b == 0 { Err(()) } else { Ok(a / b) } }
pub fn chain(a: u32, b: u32, c: u32) -> Result<u32, ()> { let x = div(a, b)?; div(x, c) }
pub fn opt(x: Option<u32>) -> u32 { match x { Some(v) => v, None => 0 } }
pub fn unwrap_or(x: Option<u32>) -> u32 { x.unwrap_or(7) }
pub fn map_opt(x: Option<u32>) -> Option<u32> { x.map(|v| v + 1) }
pub fn q(x: Option<u32>) -> Option<u32> { let v = x?; Some(v) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace d05_option_result
-- [core::option::{impl core::ops::try_trait::Try for core::option::Option<T>}::branch]:
@[rust_fun
  "core::option::{core::ops::try_trait::Try<core::option::Option<@T>>}::branch"]
axiom core.option.Option.Insts.CoreOpsTry_traitTry.branch
  {T : Type _} :
  Option T → Result (core.ops.control_flow.ControlFlow (Option Never) T)
-- [core::option::{impl core::ops::try_trait::FromResidual<core::option::Option<!>> for core::option::Option<T>}::from_residual]:
@[rust_fun
  "core::option::{core::ops::try_trait::FromResidual<core::option::Option<@T>, core::option::Option<!>>}::from_residual"]
axiom
  core.option.Option.Insts.CoreOpsTry_traitFromResidualOptionNever.from_residual
  (T : Type _) : Option Never → Result (Option T)
-- [d05_option_result::div]:
def div
  (a : Std.U32) (b : Std.U32) : Result (core.result.Result Std.U32 Unit) := do
  if b = 0#u32
  then ok (core.result.Result.Err ())
  else let i ← a / b
       ok (core.result.Result.Ok i)
-- [d05_option_result::chain]:
def chain
  (a : Std.U32) (b : Std.U32) (c : Std.U32) :
  Result (core.result.Result Std.U32 Unit)
  := do
  let r ← div a b
  let cf ← core.result.Result.Insts.CoreOpsTry.branch r
  match cf with
  | core.ops.control_flow.ControlFlow.Continue val => div val c
  | core.ops.control_flow.ControlFlow.Break residual =>
    core.result.Result.Insts.CoreOpsTry_traitFromResidualResult.from_residual
      Std.U32 (core.convert.FromSame Unit) residual
-- [d05_option_result::opt]:
def opt (x : Option Std.U32) : Result Std.U32 := do
  match x with
  | none => ok 0#u32
  | some v => ok v
-- [d05_option_result::unwrap_or]:
def unwrap_or (x : Option Std.U32) : Result Std.U32 := do
  ok (core.option.Option.unwrap_or x 7#u32)
-- [d05_option_result::map_opt::{closure}]
@[reducible]
def map_opt.closure := Unit
-- [d05_option_result::map_opt::{impl core::ops::function::FnOnce<(u32,), u32> for d05_option_result::map_opt::{closure}}::call_once]:
def map_opt.closure.Insts.CoreOpsFunctionFnOnceTupleU32U32.call_once
-- ... (21 more lines, see experiments/out/)
```


### D06_box  —  **CLEAN**

```rust
pub fn b() -> u32 { let x = Box::new(5u32); *x + 1 }
pub fn bmut(x: &mut Box<u32>) { **x = 3; }
pub enum List { Cons(u32, Box<List>), Nil }
pub fn head(l: &List) -> u32 { match l { List::Cons(h, _) => *h, List::Nil => 0 } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace d06_box
-- [d06_box::b]:
def b : Result Std.U32 := do
  5#u32 + 1#u32
-- [d06_box::bmut]:
def bmut (x : Std.U32) : Result Std.U32 := do
  ok 3#u32
-- [d06_box::List]
@[discriminant isize]
inductive List where
| Cons : Std.U32 → List → List
| Nil : List
-- [d06_box::head]:
def head (l : List) : Result Std.U32 := do
  match l with
  | List.Cons h _ => ok h
  | List.Nil => ok 0#u32
end d06_box
```


### D07_struct_enum_match  —  **CLEAN**

```rust
#[derive(Clone, Copy, PartialEq)]
pub enum Color { R, G, B(u8) }
pub struct P { pub x: i32, pub y: i32 }
pub fn f(c: Color) -> u8 { match c { Color::R => 0, Color::G => 1, Color::B(v) if v > 10 => v, Color::B(_) => 2 } }
pub fn mv(p: &mut P) { p.x += 1; }
pub fn eqc(a: Color, b: Color) -> bool { a == b }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace d07_struct_enum_match
-- [d07_struct_enum_match::Color]
@[discriminant isize]
inductive Color where
| R : Color
| G : Color
| B : Std.U8 → Color
-- [d07_struct_enum_match::{impl core::clone::Clone for d07_struct_enum_match::Color}::clone]:
def Color.Insts.CoreCloneClone.clone (self : Color) : Result Color := do
  ok self
-- Trait implementation: [d07_struct_enum_match::{impl core::clone::Clone for d07_struct_enum_match::Color}]
@[reducible]
def Color.Insts.CoreCloneClone : core.clone.Clone Color := {
  clone := Color.Insts.CoreCloneClone.clone
}
-- Trait implementation: [d07_struct_enum_match::{impl core::marker::Copy for d07_struct_enum_match::Color}]
@[reducible]
def Color.Insts.CoreMarkerCopy : core.marker.Copy Color := {
  cloneInst := Color.Insts.CoreCloneClone
}
-- Trait implementation: [d07_struct_enum_match::{impl core::marker::StructuralPartialEq for d07_struct_enum_match::Color}]
@[reducible]
def Color.Insts.CoreMarkerStructuralPartialEq : core.marker.StructuralPartialEq
  Color := {
}
-- [d07_struct_enum_match::{impl core::cmp::PartialEq<d07_struct_enum_match::Color> for d07_struct_enum_match::Color}::eq]:
def Color.Insts.CoreCmpPartialEqColor.eq
  (self : Color) (other : Color) : Result Bool := do
  let self1 := read_discriminant self
  let other1 := read_discriminant other
  if self1 = other1
  then
    match self with
    | Color.R => ok true
    | Color.G => ok true
    | Color.B __self_0 =>
      match other with
      | Color.R => ok true
      | Color.G => ok true
      | Color.B __arg1_0 =>
        lift (core.cmp.impls.PartialEqU8.eq __self_0 __arg1_0)
  else ok false
-- Trait implementation: [d07_struct_enum_match::{impl core::cmp::PartialEq<d07_struct_enum_match::Color> for d07_struct_enum_match::Color}]
@[reducible]
def Color.Insts.CoreCmpPartialEqColor : core.cmp.PartialEq Color Color := {
-- ... (22 more lines, see experiments/out/)
```


### D08_tuple_struct_derive  —  **CAVEATS**

```rust
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct W(pub u32, pub u64);
pub fn c(w: &W) -> W { w.clone() }
pub fn d() -> W { W::default() }
pub fn e(a: &W, b: &W) -> bool { a == b }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace d08_tuple_struct_derive
-- [core::fmt::{core::fmt::Formatter<'a>}::debug_tuple_field2_finish]:
@[rust_fun "core::fmt::{core::fmt::Formatter<'a>}::debug_tuple_field2_finish"]
axiom core.fmt.Formatter.debug_tuple_field2_finish
  :
  core.fmt.Formatter → Str → Dyn (fun _dyn => core.fmt.Debug _dyn) → Dyn
    (fun _dyn => core.fmt.Debug _dyn) → Result ((core.result.Result Unit
    core.fmt.Error) × core.fmt.Formatter)
-- [d08_tuple_struct_derive::W]
def W := Std.U32 × Std.U64
-- [d08_tuple_struct_derive::{impl core::clone::Clone for d08_tuple_struct_derive::W}::clone]:
def W.Insts.CoreCloneClone.clone (self : W) : Result W := do
  let (i, i1) := self
  let i2 ← lift (core.clone.impls.CloneU32.clone i)
  let i3 ← lift (core.clone.impls.CloneU64.clone i1)
  ok (i2, i3)
-- Trait implementation: [d08_tuple_struct_derive::{impl core::clone::Clone for d08_tuple_struct_derive::W}]
@[reducible]
def W.Insts.CoreCloneClone : core.clone.Clone W := {
  clone := W.Insts.CoreCloneClone.clone
}
-- [d08_tuple_struct_derive::{impl core::fmt::Debug for d08_tuple_struct_derive::W}::fmt]:
def W.Insts.CoreFmtDebug.fmt
  (self : W) (f : core.fmt.Formatter) :
  Result ((core.result.Result Unit core.fmt.Error) × core.fmt.Formatter)
  := do
  let (i, i1) := self
  let dyn := Dyn.mk _ core.fmt.DebugU32 i
  let dyn1 := Dyn.mk _ (core.fmt.DebugShared core.fmt.DebugU64) i1
  core.fmt.Formatter.debug_tuple_field2_finish f (toStr "W") dyn dyn1
-- Trait implementation: [d08_tuple_struct_derive::{impl core::fmt::Debug for d08_tuple_struct_derive::W}]
@[reducible]
def W.Insts.CoreFmtDebug : core.fmt.Debug W := {
  fmt := W.Insts.CoreFmtDebug.fmt
}
-- Trait implementation: [d08_tuple_struct_derive::{impl core::marker::StructuralPartialEq for d08_tuple_struct_derive::W}]
@[reducible]
def W.Insts.CoreMarkerStructuralPartialEq : core.marker.StructuralPartialEq W
  := {
}
-- [d08_tuple_struct_derive::{impl core::cmp::PartialEq<d08_tuple_struct_derive::W> for d08_tuple_struct_derive::W}::eq]:
def W.Insts.CoreCmpPartialEqW.eq (self : W) (other : W) : Result Bool := do
  let (i, i1) := self
  let (i2, i3) := other
  if i = i2
-- ... (34 more lines, see experiments/out/)
```


### F01_extern_c  —  **CAVEATS**

```rust
extern "C" { fn ext_add(a: u32, b: u32) -> u32; }
pub fn call(a: u32) -> u32 { unsafe { ext_add(a, 1) } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace f01_extern_c
-- [f01_extern_c::ext_add]:
axiom ext_add : Std.U32 → Std.U32 → Result Std.U32
-- [f01_extern_c::call]:
def call (a : Std.U32) : Result Std.U32 := do
  ext_add a 1#u32
end f01_extern_c
```


### F02_opaque_module  —  **CAVEATS**

```rust
// This module is marked opaque via `--opaque crate::hidden` on the Charon command line.
pub mod hidden {
    pub fn secret(x: u32) -> u32 { x.wrapping_mul(31) }
}
pub fn use_hidden(x: u32) -> u32 { hidden::secret(x) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace f02_opaque_module
-- [f02_opaque_module::hidden::secret]:
axiom hidden.secret : Std.U32 → Result Std.U32
-- [f02_opaque_module::use_hidden]:
def use_hidden (x : Std.U32) : Result Std.U32 := do
  hidden.secret x
end f02_opaque_module
```


### F03_no_mangle_export  —  **CLEAN**

```rust
#[no_mangle]
pub extern "C" fn exported(x: u32) -> u32 { x.wrapping_add(1) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace f03_no_mangle_export
-- [f03_no_mangle_export::exported]:
def exported (x : Std.U32) : Result Std.U32 := do
  ok (core.num.U32.wrapping_add x 1#u32)
end f03_no_mangle_export
```


### I01_cell  —  **CAVEATS**

```rust
use std::cell::Cell;
pub fn c(x: &Cell<u32>) { x.set(x.get() + 1) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace i01_cell
-- [core::cell::Cell]
@[rust_type "core::cell::Cell"]
axiom core.cell.Cell (T : Type _) : Type
-- [core::cell::{core::cell::Cell<T>}::set]:
@[rust_fun "core::cell::{core::cell::Cell<@T>}::set"]
axiom core.cell.Cell.set {T : Type _} : core.cell.Cell T → T → Result Unit
-- [core::cell::{core::cell::Cell<T>}::get]:
@[rust_fun "core::cell::{core::cell::Cell<@T>}::get"]
axiom core.cell.Cell.get
  {T : Type _} (markerCopyInst : core.marker.Copy T) :
  core.cell.Cell T → Result T
-- [i01_cell::c]:
def c (x : core.cell.Cell Std.U32) : Result Unit := do
  let i ← core.cell.Cell.get core.marker.CopyU32 x
  let i1 ← i + 1#u32
  core.cell.Cell.set x i1
end i01_cell
```


### I02_refcell  —  **CAVEATS**

```rust
use std::cell::RefCell;
pub fn r(x: &RefCell<u32>) { *x.borrow_mut() += 1 }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace i02_refcell
-- [core::cell::RefCell]
@[rust_type "core::cell::RefCell"]
axiom core.cell.RefCell (T : Type _) : Type
-- [core::cell::RefMut]
@[rust_type "core::cell::RefMut" (mutRegions := #[0])]
axiom core.cell.RefMut (T : Type _) : Type
-- [core::cell::{core::cell::RefCell<T>}::borrow_mut]:
@[rust_fun "core::cell::{core::cell::RefCell<@T>}::borrow_mut"]
axiom core.cell.RefCell.borrow_mut
  {T : Type _} :
  core.cell.RefCell T → Result ((core.cell.RefMut T) × (core.cell.RefMut T
    → Unit))
-- [core::cell::{impl core::ops::deref::DerefMut<T> for core::cell::RefMut<'_0, T>}::deref_mut]:
@[rust_fun
  "core::cell::{core::ops::deref::DerefMut<core::cell::RefMut<'0, @T>, @T>}::deref_mut"]
axiom core.cell.RefMut.Insts.CoreOpsDerefDerefMut.deref_mut
  {T : Type _} :
  core.cell.RefMut T → Result (T × (T → core.cell.RefMut T) ×
    (core.cell.RefMut T → core.cell.RefMut T))
-- [i02_refcell::r]:
def r (x : core.cell.RefCell Std.U32) : Result Unit := do
  let (rm, _) ← core.cell.RefCell.borrow_mut x
  let (i, _, _) ← core.cell.RefMut.Insts.CoreOpsDerefDerefMut.deref_mut rm
  let _ ← i + 1#u32
  ok ()
end i02_refcell
```


### I03_rc  —  **CAVEATS**

```rust
use std::rc::Rc;
pub fn rc(x: u32) -> u32 { let a = Rc::new(x); let b = Rc::clone(&a); *a + *b }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace i03_rc
-- Trait declaration: [core::alloc::AllocatorClone]
@[rust_trait "core::alloc::AllocatorClone"
  (parentClauses := ["cloneCloneInst"])]
structure core.alloc.AllocatorClone (Self : Type _) where
  cloneCloneInst : core.clone.Clone Self
-- Trait implementation: [alloc::alloc::{impl core::alloc::AllocatorClone for alloc::alloc::Global}]
@[reducible, rust_trait_impl
  "core::alloc::AllocatorClone<alloc::alloc::Global>"]
def alloc.alloc.Global.Insts.CoreAllocAllocatorClone :
  core.alloc.AllocatorClone alloc.alloc.Global := {
  cloneCloneInst := core.clone.CloneGlobal
}
-- [alloc::rc::Rc]
@[rust_type "alloc::rc::Rc"]
axiom alloc.rc.Rc (T : Type _) : Type
-- [alloc::rc::{alloc::rc::Rc<T>}::new]:
@[rust_fun "alloc::rc::{alloc::rc::Rc<@T>}::new"]
axiom alloc.rc.Rc.new {T : Type _} : T → Result (alloc.rc.Rc T)
-- [alloc::rc::{impl core::ops::deref::Deref<T> for alloc::rc::Rc<T>}::deref]:
@[rust_fun
  "alloc::rc::{core::ops::deref::Deref<alloc::rc::Rc<@T>, @T>}::deref"]
axiom alloc.rc.Rc.Insts.CoreOpsDerefDeref.deref
  {T : Type _} (A : Type _) : alloc.rc.Rc T → Result T
-- [alloc::rc::{impl core::clone::Clone for alloc::rc::Rc<T>}::clone]:
@[rust_fun "alloc::rc::{core::clone::Clone<alloc::rc::Rc<@T>>}::clone"]
axiom alloc.rc.Rc.Insts.CoreCloneClone.clone
  {T : Type _} {A : Type _} (coreallocAllocatorCloneInst :
  core.alloc.AllocatorClone A) :
  alloc.rc.Rc T → Result (alloc.rc.Rc T)
-- [i03_rc::rc]:
def rc (x : Std.U32) : Result Std.U32 := do
  let a ← alloc.rc.Rc.new x
  let b ←
    alloc.rc.Rc.Insts.CoreCloneClone.clone
      alloc.alloc.Global.Insts.CoreAllocAllocatorClone a
  let i ← alloc.rc.Rc.Insts.CoreOpsDerefDeref.deref alloc.alloc.Global a
  let i1 ← alloc.rc.Rc.Insts.CoreOpsDerefDeref.deref alloc.alloc.Global b
  i + i1
end i03_rc
```


### I04_arc  —  **CAVEATS**

```rust
use std::sync::Arc;
pub fn arc(x: u32) -> u32 { let a = Arc::new(x); let b = a.clone(); *b }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace i04_arc
-- Trait declaration: [core::alloc::AllocatorClone]
@[rust_trait "core::alloc::AllocatorClone"
  (parentClauses := ["cloneCloneInst"])]
structure core.alloc.AllocatorClone (Self : Type _) where
  cloneCloneInst : core.clone.Clone Self
-- Trait implementation: [alloc::alloc::{impl core::alloc::AllocatorClone for alloc::alloc::Global}]
@[reducible, rust_trait_impl
  "core::alloc::AllocatorClone<alloc::alloc::Global>"]
def alloc.alloc.Global.Insts.CoreAllocAllocatorClone :
  core.alloc.AllocatorClone alloc.alloc.Global := {
  cloneCloneInst := core.clone.CloneGlobal
}
-- [alloc::sync::Arc]
@[rust_type "alloc::sync::Arc"]
axiom alloc.sync.Arc (T : Type _) : Type
-- [alloc::sync::{alloc::sync::Arc<T>}::new]:
@[rust_fun "alloc::sync::{alloc::sync::Arc<@T>}::new"]
axiom alloc.sync.Arc.new {T : Type _} : T → Result (alloc.sync.Arc T)
-- [alloc::sync::{impl core::clone::Clone for alloc::sync::Arc<T>}::clone]:
@[rust_fun "alloc::sync::{core::clone::Clone<alloc::sync::Arc<@T>>}::clone"]
axiom alloc.sync.Arc.Insts.CoreCloneClone.clone
  {T : Type _} {A : Type _} (coreallocAllocatorCloneInst :
  core.alloc.AllocatorClone A) :
  alloc.sync.Arc T → Result (alloc.sync.Arc T)
-- [alloc::sync::{impl core::ops::deref::Deref<T> for alloc::sync::Arc<T>}::deref]:
@[rust_fun
  "alloc::sync::{core::ops::deref::Deref<alloc::sync::Arc<@T>, @T>}::deref"]
axiom alloc.sync.Arc.Insts.CoreOpsDerefDeref.deref
  {T : Type _} (A : Type _) : alloc.sync.Arc T → Result T
-- [i04_arc::arc]:
def arc (x : Std.U32) : Result Std.U32 := do
  let a ← alloc.sync.Arc.new x
  let b ←
    alloc.sync.Arc.Insts.CoreCloneClone.clone
      alloc.alloc.Global.Insts.CoreAllocAllocatorClone a
  alloc.sync.Arc.Insts.CoreOpsDerefDeref.deref alloc.alloc.Global b
end i04_arc
```


### R01_recursion  —  **CLEAN**

```rust
pub fn fact(n: u64) -> u64 { if n == 0 { 1 } else { n * fact(n - 1) } }
pub enum L { Cons(u32, Box<L>), Nil }
pub fn len(l: &L) -> u32 { match l { L::Cons(_, t) => 1 + len(t), L::Nil => 0 } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace r01_recursion
-- [r01_recursion::fact]:
def fact (n : Std.U64) : Result Std.U64 := do
  if n = 0#u64
  then ok 1#u64
  else let i ← n - 1#u64
       let i1 ← fact i
       n * i1
partial_fixpoint
-- [r01_recursion::L]
@[discriminant isize]
inductive L where
| Cons : Std.U32 → L → L
| Nil : L
-- [r01_recursion::len]:
def len (l : L) : Result Std.U32 := do
  match l with
  | L.Cons _ t => let i ← len t
                  1#u32 + i
  | L.Nil => ok 0#u32
partial_fixpoint
end r01_recursion
```


### R02_mutual  —  **CLEAN**

```rust
pub fn even(n: u32) -> bool { if n == 0 { true } else { odd(n - 1) } }
pub fn odd(n: u32) -> bool { if n == 0 { false } else { even(n - 1) } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace r02_mutual
mutual
-- [r02_mutual::even]:
def even (n : Std.U32) : Result Bool := do
  if n = 0#u32
  then ok true
  else let i ← n - 1#u32
       odd i
partial_fixpoint
-- [r02_mutual::odd]:
def odd (n : Std.U32) : Result Bool := do
  if n = 0#u32
  then ok false
  else let i ← n - 1#u32
       even i
partial_fixpoint
end
end r02_mutual
```


### R03_mutual_inductive  —  **FAIL**

```rust
pub enum Tree { Node(u32, Forest) }
pub enum Forest { Nil, Cons(Box<Tree>, Box<Forest>) }
pub fn tsum(t: &Tree) -> u32 { match t { Tree::Node(v, f) => v + fsum(f) } }
pub fn fsum(f: &Forest) -> u32 { match f { Forest::Nil => 0, Forest::Cons(t, r) => tsum(t) + fsum(r) } }
```

Exit codes: charon=0, aeneas=0, lean=1.

Lean errors:
```text
../out/R03_mutual_inductive/lean/R03MutualInductive.lean:49:10: error: Could not prove 'r03_mutual_inductive.tsum' to be monotone in its recursive calls:
```

Generated Lean:
```lean
namespace r03_mutual_inductive
mutual
-- [r03_mutual_inductive::Tree]
@[discriminant isize]
inductive Tree where
| Node : Std.U32 → Forest → Tree
-- [r03_mutual_inductive::Forest]
@[discriminant isize]
inductive Forest where
| Nil : Forest
| Cons : Tree → Forest → Forest
end
mutual
-- [r03_mutual_inductive::tsum]:
def tsum (t : Tree) : Result Std.U32 := do
  let ⟨ v, f ⟩ := t
  let i ← fsum f
  SharedU32.Insts.CoreOpsArithAddU32U32.add v i
partial_fixpoint
-- [r03_mutual_inductive::fsum]:
def fsum (f : Forest) : Result Std.U32 := do
  match f with
  | Forest.Nil => ok 0#u32
  | Forest.Cons t r => let i ← tsum t
                       let i1 ← fsum r
                       i + i1
partial_fixpoint
end
end r03_mutual_inductive
```


### S01_hashmap  —  **CAVEATS**

```rust
use std::collections::HashMap;
pub fn h() -> Option<u32> { let mut m = HashMap::new(); m.insert(1u32, 2u32); m.get(&1).copied() }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace s01_hashmap
-- Trait declaration: [core::hash::BuildHasher]
@[rust_trait "core::hash::BuildHasher" (parentClauses := ["HasherInst"])]
structure core.hash.BuildHasher (Self : Type _) (Self_Hasher : Type _) where
  HasherInst : core.hash.Hasher Self_Hasher
  build_hasher : Self → Result Self_Hasher
-- [core::hash::impls::{impl core::hash::Hash for u32}::hash]:
@[rust_fun "core::hash::impls::{core::hash::Hash<u32>}::hash"]
axiom U32.Insts.CoreHashHash.hash
  {H : Type _} (HasherInst : core.hash.Hasher H) : Std.U32 → H → Result H
-- Trait implementation: [core::hash::impls::{impl core::hash::Hash for u32}]
@[reducible, rust_trait_impl "core::hash::Hash<u32>"]
def U32.Insts.CoreHashHash : core.hash.Hash Std.U32 := {
  hash := fun {H : Type} (HasherInst : core.hash.Hasher H) =>
    U32.Insts.CoreHashHash.hash HasherInst
}
-- [core::option::{core::option::Option<&'_0 T>}::copied]:
@[rust_fun "core::option::{core::option::Option<&'0 @T>}::copied"]
axiom core.option.OptionShared0T.copied
  {T : Type _} (markerCopyInst : core.marker.Copy T) :
  Option T → Result (Option T)
-- [std::collections::hash::map::HashMap]
@[rust_type "std::collections::hash::map::HashMap"]
axiom std.collections.hash.map.HashMap (K : Type _) (V : Type _) (S : Type _)
  (A : Type _) : Type
-- [std::hash::random::RandomState]
@[rust_type "std::hash::random::RandomState"]
axiom std.hash.random.RandomState : Type
-- [std::collections::hash::map::{std::collections::hash::map::HashMap<K, V, std::hash::random::RandomState, alloc::alloc::Global>}::new]:
@[rust_fun
  "std::collections::hash::map::{std::collections::hash::map::HashMap<@K, @V, std::hash::random::RandomState, alloc::alloc::Global>}::new"]
axiom std.collections.hash.map.HashMapKVRandomStateGlobal.new
  (K : Type _) (V : Type _) :
  Result (std.collections.hash.map.HashMap K V std.hash.random.RandomState
    alloc.alloc.Global)
-- [std::collections::hash::map::{std::collections::hash::map::HashMap<K, V, S, A>}::get]:
@[rust_fun
  "std::collections::hash::map::{std::collections::hash::map::HashMap<@K, @V, @S, @A>}::get"]
axiom std.collections.hash.map.HashMap.get
  {K : Type _} {V : Type _} {S : Type _} {A : Type _} {Q : Type _}
  {Clause2_Hasher : Type _} (corecmpEqInst : core.cmp.Eq K) (corehashHashInst :
  core.hash.Hash K) (corehashBuildHasherInst : core.hash.BuildHasher S
  Clause2_Hasher) (coreborrowBorrowInst : core.borrow.Borrow K Q)
  (corehashHashInst1 : core.hash.Hash Q) (corecmpEqInst1 : core.cmp.Eq Q) :
  std.collections.hash.map.HashMap K V S A → Q → Result (Option V)
-- ... (64 more lines, see experiments/out/)
```


### S02_cmp_minmax  —  **CLEAN**

```rust
use std::cmp::Ordering;
pub fn mx(a: u32, b: u32) -> u32 { std::cmp::max(a, b) }
pub fn mn(a: u32, b: u32) -> u32 { a.min(b) }
pub fn c(a: u32, b: u32) -> Ordering { a.cmp(&b) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace s02_cmp_minmax
-- [s02_cmp_minmax::mx]:
def mx (a : Std.U32) (b : Std.U32) : Result Std.U32 := do
  core.cmp.max core.cmp.OrdU32 a b
-- [s02_cmp_minmax::mn]:
def mn (a : Std.U32) (b : Std.U32) : Result Std.U32 := do
  ok (core.cmp.impls.OrdU32.min a b)
-- [s02_cmp_minmax::c]:
def c (a : Std.U32) (b : Std.U32) : Result Ordering := do
  ok (core.cmp.impls.OrdU32.cmp a b)
end s02_cmp_minmax
```


### S03_mem_swap_replace  —  **CAVEATS**

```rust
pub fn s(a: &mut u32, b: &mut u32) { core::mem::swap(a, b) }
pub fn r(a: &mut u32) -> u32 { core::mem::replace(a, 0) }
pub fn t(v: &mut Vec<u32>) -> Vec<u32> { core::mem::take(v) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace s03_mem_swap_replace
-- [core::mem::take]:
@[rust_fun "core::mem::take"]
axiom core.mem.take
  {T : Type _} (defaultDefaultInst : core.default.Default T) :
  T → Result (T × T)
-- [alloc::vec::{impl core::default::Default for alloc::vec::Vec<T>}::default]:
@[rust_fun
  "alloc::vec::{core::default::Default<alloc::vec::Vec<@T>>}::default"]
axiom alloc.vec.Vec.Insts.CoreDefaultDefault.default
  (T : Type _) : Result (alloc.vec.Vec T)
-- Trait implementation: [alloc::vec::{impl core::default::Default for alloc::vec::Vec<T>}]
@[reducible, rust_trait_impl "core::default::Default<alloc::vec::Vec<@T>>"]
def alloc.vec.Vec.Insts.CoreDefaultDefault (T : Type _) : core.default.Default
  (alloc.vec.Vec T) := {
  default := alloc.vec.Vec.Insts.CoreDefaultDefault.default T
}
-- [s03_mem_swap_replace::s]:
def s (a : Std.U32) (b : Std.U32) : Result (Std.U32 × Std.U32) := do
  ok (core.mem.swap a b)
-- [s03_mem_swap_replace::r]:
def r (a : Std.U32) : Result (Std.U32 × Std.U32) := do
  ok (core.mem.replace a 0#u32)
-- [s03_mem_swap_replace::t]:
def t
  (v : alloc.vec.Vec Std.U32) :
  Result ((alloc.vec.Vec Std.U32) × (alloc.vec.Vec Std.U32))
  := do
  core.mem.take (alloc.vec.Vec.Insts.CoreDefaultDefault Std.U32) v
end s03_mem_swap_replace
```


### S04_slice_methods  —  **CAVEATS**

```rust
pub fn c(s: &[u32]) -> bool { s.contains(&3) }
pub fn srt(s: &mut [u32]) { s.sort() }
pub fn cp(d: &mut [u32], s: &[u32]) { d.copy_from_slice(s) }
pub fn fl(d: &mut [u8]) { d.fill(0) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace s04_slice_methods
-- [alloc::slice::{[T]}::sort]:
@[rust_fun "alloc::slice::{[@T]}::sort"]
axiom alloc.slice.Slice.sort
  {T : Type _} (corecmpOrdInst : core.cmp.Ord T) : Slice T → Result (Slice T)
-- [s04_slice_methods::c]:
def c (s : Slice Std.U32) : Result Bool := do
  core.slice.Slice.contains core.cmp.PartialEqU32 s 3#u32
-- [s04_slice_methods::srt]:
def srt (s : Slice Std.U32) : Result (Slice Std.U32) := do
  alloc.slice.Slice.sort core.cmp.OrdU32 s
-- [s04_slice_methods::cp]:
def cp (d : Slice Std.U32) (s : Slice Std.U32) : Result (Slice Std.U32) := do
  core.slice.Slice.copy_from_slice core.marker.CopyU32 d s
-- [s04_slice_methods::fl]:
def fl (d : Slice Std.U8) : Result (Slice Std.U8) := do
  core.slice.Slice.fill core.clone.CloneU8 d 0#u8
end s04_slice_methods
```


### S05_fmt_print  —  **CAVEATS**

```rust
pub fn p(x: u32) { println!("{}", x) }
pub fn f(x: u32) -> String { format!("{}", x) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace s05_fmt_print
-- Trait implementation: [core::fmt::num::imp::{impl core::fmt::Display for u32}]
@[reducible, rust_trait_impl "core::fmt::Display<u32>"]
def U32.Insts.CoreFmtDisplay : core.fmt.Display Std.U32 := {
  fmt := core.fmt.num.imp.DisplayU32.fmt
}
-- [core::hint::must_use]:
@[rust_fun "core::hint::must_use"]
axiom core.hint.must_use {T : Type _} : T → Result T
-- [alloc::fmt::format]:
@[rust_fun "alloc::fmt::format"]
axiom alloc.fmt.format : core.fmt.Arguments → Result String
-- [s05_fmt_print::p]:
def p (x : Std.U32) : Result Unit := do
  let a ← core.fmt.rt.Argument.new_display U32.Insts.CoreFmtDisplay x
  let a1 ←
    core.fmt.Arguments.new (Array.make 4#usize [ 192#u8, 1#u8, 10#u8, 0#u8 ])
      (Array.make 1#usize [ a ])
  std.io.stdio._print a1
-- [s05_fmt_print::f]:
def f (x : Std.U32) : Result String := do
  let a ← core.fmt.rt.Argument.new_display U32.Insts.CoreFmtDisplay x
  let a1 ←
    core.fmt.Arguments.new (Array.make 2#usize [ 192#u8, 0#u8 ])
      (Array.make 1#usize [ a ])
  let s ← alloc.fmt.format a1
  core.hint.must_use s
end s05_fmt_print
```


### S06_vecdeque_btree  —  **CAVEATS**

```rust
use std::collections::{BTreeMap, VecDeque};
pub fn q() -> Option<u32> { let mut d = VecDeque::new(); d.push_back(1u32); d.pop_front() }
pub fn b() -> usize { let mut m = BTreeMap::new(); m.insert(1u32, 1u32); m.len() }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace s06_vecdeque_btree
-- Trait declaration: [core::alloc::AllocatorClone]
@[rust_trait "core::alloc::AllocatorClone"
  (parentClauses := ["cloneCloneInst"])]
structure core.alloc.AllocatorClone (Self : Type _) where
  cloneCloneInst : core.clone.Clone Self
-- Trait implementation: [alloc::alloc::{impl core::alloc::AllocatorClone for alloc::alloc::Global}]
@[reducible, rust_trait_impl
  "core::alloc::AllocatorClone<alloc::alloc::Global>"]
def alloc.alloc.Global.Insts.CoreAllocAllocatorClone :
  core.alloc.AllocatorClone alloc.alloc.Global := {
  cloneCloneInst := core.clone.CloneGlobal
}
-- [alloc::collections::btree::map::BTreeMap]
@[rust_type "alloc::collections::btree::map::BTreeMap"]
axiom alloc.collections.btree.map.BTreeMap (K : Type _) (V : Type _) (A :
  Type _) : Type
-- [alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<K, V, alloc::alloc::Global>}::new]:
@[rust_fun
  "alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<@K, @V, alloc::alloc::Global>}::new"]
axiom alloc.collections.btree.map.BTreeMapKVGlobal.new
  (K : Type _) (V : Type _) :
  Result (alloc.collections.btree.map.BTreeMap K V alloc.alloc.Global)
-- [alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<K, V, A>}::insert]:
@[rust_fun
  "alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<@K, @V, @A>}::insert"]
axiom alloc.collections.btree.map.BTreeMap.insert
  {K : Type _} {V : Type _} {A : Type _} (coreallocAllocatorCloneInst :
  core.alloc.AllocatorClone A) (corecmpOrdInst : core.cmp.Ord K) :
  alloc.collections.btree.map.BTreeMap K V A → K → V → Result ((Option V)
    × (alloc.collections.btree.map.BTreeMap K V A))
-- [alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<K, V, A>}::len]:
@[rust_fun
  "alloc::collections::btree::map::{alloc::collections::btree::map::BTreeMap<@K, @V, @A>}::len"]
axiom alloc.collections.btree.map.BTreeMap.len
  {K : Type _} {V : Type _} {A : Type _} (coreallocAllocatorCloneInst :
  core.alloc.AllocatorClone A) :
  alloc.collections.btree.map.BTreeMap K V A → Result Std.Usize
-- [alloc::collections::vec_deque::VecDeque]
@[rust_type "alloc::collections::vec_deque::VecDeque"]
axiom alloc.collections.vec_deque.VecDeque (T : Type _) (A : Type _) : Type
-- [alloc::collections::vec_deque::{alloc::collections::vec_deque::VecDeque<T, alloc::alloc::Global>}::new]:
@[rust_fun
  "alloc::collections::vec_deque::{alloc::collections::vec_deque::VecDeque<@T, alloc::alloc::Global>}::new"]
axiom alloc.collections.vec_deque.VecDequeTGlobal.new
-- ... (32 more lines, see experiments/out/)
```


### S07_from_into  —  **CLEAN**

```rust
pub struct M(pub u32);
impl From<u32> for M { fn from(x: u32) -> M { M(x) } }
pub fn conv(x: u32) -> M { x.into() }
pub fn b(x: u32) -> [u8; 4] { x.to_le_bytes() }
pub fn fb(b: [u8; 4]) -> u32 { u32::from_le_bytes(b) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace s07_from_into
-- [s07_from_into::M]
@[reducible]
def M := Std.U32
-- [s07_from_into::{impl core::convert::From<u32> for s07_from_into::M}::from]:
def M.Insts.CoreConvertFromU32.from (x : Std.U32) : Result M := do
  ok x
-- Trait implementation: [s07_from_into::{impl core::convert::From<u32> for s07_from_into::M}]
@[reducible]
def M.Insts.CoreConvertFromU32 : core.convert.From M Std.U32 := {
  «from» := M.Insts.CoreConvertFromU32.from
}
-- [s07_from_into::conv]:
def conv (x : Std.U32) : Result M := do
  core.convert.IntoFrom.into M.Insts.CoreConvertFromU32 x
-- [s07_from_into::b]:
def b (x : Std.U32) : Result (Array Std.U8 4#usize) := do
  ok (core.num.U32.to_le_bytes x)
-- [s07_from_into::fb]:
def fb (b1 : Array Std.U8 4#usize) : Result Std.U32 := do
  ok (core.num.U32.from_le_bytes b1)
end s07_from_into
```


### S08_drop_impl  —  **CLEAN**

```rust
pub struct G { pub x: u32 }
impl Drop for G { fn drop(&mut self) {} }
pub fn d() -> u32 { let g = G { x: 1 }; g.x }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace s08_drop_impl
-- [s08_drop_impl::G]
structure G where
  x : Std.U32
-- [s08_drop_impl::{impl core::ops::drop::Drop for s08_drop_impl::G}::drop]:
def G.Insts.CoreOpsDropDrop.drop (self : G) : Result G := do
  ok self
-- Trait implementation: [s08_drop_impl::{impl core::ops::drop::Drop for s08_drop_impl::G}]
@[reducible]
def G.Insts.CoreOpsDropDrop : core.ops.drop.Drop G := {
  drop := G.Insts.CoreOpsDropDrop.drop
}
-- [s08_drop_impl::d]:
def d : Result Std.U32 := do
  ok 1#u32
end s08_drop_impl
```


### T01_trait_generic  —  **CLEAN**

```rust
pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn total_area<T: Shape>(a: &T, b: &T) -> u32 { a.area() + b.area() }
pub fn use_it() -> u32 { total_area(&Sq { s: 2 }, &Sq { s: 3 }) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t01_trait_generic
-- Trait declaration: [t01_trait_generic::Shape]
structure Shape (Self : Type _) where
  area : Self → Result Std.U32
-- [t01_trait_generic::Sq]
structure Sq where
  s : Std.U32
-- [t01_trait_generic::{impl t01_trait_generic::Shape for t01_trait_generic::Sq}::area]:
def Sq.Insts.T01_trait_genericShape.area (self : Sq) : Result Std.U32 := do
  self.s * self.s
-- Trait implementation: [t01_trait_generic::{impl t01_trait_generic::Shape for t01_trait_generic::Sq}]
@[reducible]
def Sq.Insts.T01_trait_genericShape : Shape Sq := {
  area := Sq.Insts.T01_trait_genericShape.area
}
-- [t01_trait_generic::total_area]:
def total_area
  {T : Type _} (ShapeInst : Shape T) (a : T) (b : T) : Result Std.U32 := do
  let i ← ShapeInst.area a
  let i1 ← ShapeInst.area b
  i + i1
-- [t01_trait_generic::use_it]:
def use_it : Result Std.U32 := do
  total_area Sq.Insts.T01_trait_genericShape { s := 2#u32 } { s := 3#u32 }
end t01_trait_generic
```


### T02_assoc_types  —  **CLEAN**

```rust
pub trait Container { type Item; fn get(&self, i: usize) -> Option<&Self::Item>; }
pub struct Wrap { pub v: Vec<u32> }
impl Container for Wrap {
    type Item = u32;
    fn get(&self, i: usize) -> Option<&u32> { if i < self.v.len() { Some(&self.v[i]) } else { None } }
}
pub fn first<C: Container>(c: &C) -> Option<&C::Item> { c.get(0) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t02_assoc_types
-- Trait declaration: [t02_assoc_types::Container]
structure Container (Self : Type _) (Self_Item : Type _) where
  get : Self → Std.Usize → Result (Option Self_Item)
-- [t02_assoc_types::Wrap]
structure Wrap where
  v : alloc.vec.Vec Std.U32
-- [t02_assoc_types::{impl t02_assoc_types::Container<u32> for t02_assoc_types::Wrap}::get]:
def Wrap.Insts.T02_assoc_typesContainerU32.get
  (self : Wrap) (i : Std.Usize) : Result (Option Std.U32) := do
  let i1 := alloc.vec.Vec.len self.v
  if i < i1
  then
    let i2 ←
      alloc.vec.Vec.index (core.slice.index.SliceIndexUsizeSlice Std.U32)
        self.v i
    ok (some i2)
  else ok none
-- Trait implementation: [t02_assoc_types::{impl t02_assoc_types::Container<u32> for t02_assoc_types::Wrap}]
@[reducible]
def Wrap.Insts.T02_assoc_typesContainerU32 : Container Wrap Std.U32 := {
  get := Wrap.Insts.T02_assoc_typesContainerU32.get
}
-- [t02_assoc_types::first]:
def first
  {C : Type _} {Clause0_Item : Type _} (ContainerInst : Container C
  Clause0_Item) (c : C) :
  Result (Option Clause0_Item)
  := do
  ContainerInst.get c 0#usize
end t02_assoc_types
```


### T03_const_generics  —  **CLEAN**

```rust
pub struct Buf<const N: usize> { pub data: [u8; N] }
pub fn len_of<const N: usize>(b: &Buf<N>) -> usize { N }
pub fn get0<const N: usize>(b: &Buf<N>) -> u8 { b.data[0] }
pub fn mk() -> Buf<4> { Buf { data: [0; 4] } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t03_const_generics
-- [t03_const_generics::Buf]
structure Buf (N : Std.Usize) where
  data : Array Std.U8 N
-- [t03_const_generics::len_of]:
def len_of {N : Std.Usize} (b : Buf N) : Result Std.Usize := do
  ok N
-- [t03_const_generics::get0]:
def get0 {N : Std.Usize} (b : Buf N) : Result Std.U8 := do
  Array.index_usize b.data 0#usize
-- [t03_const_generics::mk]:
def mk : Result (Buf 4#usize) := do
  let a := Array.repeat 4#usize 0#u8
  ok { data := a }
end t03_const_generics
```


### T04_dyn_trait  —  **FAIL**

```rust
pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn area_dyn(x: &dyn Shape) -> u32 { x.area() }
pub fn call() -> u32 { let s = Sq { s: 3 }; area_dyn(&s) }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Function pointers are not supported yet
[Warn] Could not translate the body of function 't04_dyn_trait::area_dyn because of previous error
```

Generated Lean:
```lean
namespace t04_dyn_trait
-- Trait declaration: [t04_dyn_trait::Shape]
structure Shape (Self : Type _) where
  area : Self → Result Std.U32
-- [t04_dyn_trait::Sq]
structure Sq where
  s : Std.U32
-- [t04_dyn_trait::{impl t04_dyn_trait::Shape for t04_dyn_trait::Sq}::area]:
def Sq.Insts.T04_dyn_traitShape.area (self : Sq) : Result Std.U32 := do
  self.s * self.s
-- Trait implementation: [t04_dyn_trait::{impl t04_dyn_trait::Shape for t04_dyn_trait::Sq}]
@[reducible]
def Sq.Insts.T04_dyn_traitShape : Shape Sq := {
  area := Sq.Insts.T04_dyn_traitShape.area
}
-- [t04_dyn_trait::area_dyn]:
def area_dyn (x : Dyn (fun _dyn => Shape _dyn)) : Result Std.U32 := do
  sorry
-- [t04_dyn_trait::call]:
def call : Result Std.U32 := do
  let dyn := Dyn.mk _ Sq.Insts.T04_dyn_traitShape ({ s := 3#u32 } : Sq)
  area_dyn dyn
end t04_dyn_trait
```


### T04b_box_dyn  —  **FAIL**

```rust
pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn mk() -> Box<dyn Shape> { Box::new(Sq { s: 3 }) }
pub fn use_box(b: Box<dyn Shape>) -> u32 { b.area() }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Function pointers are not supported yet
[Warn] Could not translate the body of function 't04b_box_dyn::use_box because of previous error
```

Generated Lean:
```lean
namespace t04b_box_dyn
-- Trait declaration: [t04b_box_dyn::Shape]
structure Shape (Self : Type _) where
  area : Self → Result Std.U32
-- [t04b_box_dyn::Sq]
structure Sq where
  s : Std.U32
-- [t04b_box_dyn::{impl t04b_box_dyn::Shape for t04b_box_dyn::Sq}::area]:
def Sq.Insts.T04b_box_dynShape.area (self : Sq) : Result Std.U32 := do
  self.s * self.s
-- Trait implementation: [t04b_box_dyn::{impl t04b_box_dyn::Shape for t04b_box_dyn::Sq}]
@[reducible]
def Sq.Insts.T04b_box_dynShape : Shape Sq := {
  area := Sq.Insts.T04b_box_dynShape.area
}
-- [t04b_box_dyn::mk]:
def mk : Result (Dyn (fun _dyn => Shape _dyn)) := do
  ok (Dyn.mk _ Sq.Insts.T04b_box_dynShape ({ s := 3#u32 } : Sq))
-- [t04b_box_dyn::use_box]:
def use_box (b : Dyn (fun _dyn => Shape _dyn)) : Result Std.U32 := do
  sorry
end t04b_box_dyn
```


### T05_default_supertrait  —  **CLEAN**

```rust
pub trait Named { fn id(&self) -> u32; }
pub trait Greet: Named {
    fn greet(&self) -> u32 { self.id() + 1 }
}
pub struct P;
impl Named for P { fn id(&self) -> u32 { 41 } }
impl Greet for P {}
pub fn g<T: Greet>(x: &T) -> u32 { x.greet() }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t05_default_supertrait
-- Trait declaration: [t05_default_supertrait::Named]
structure Named (Self : Type _) where
  id : Self → Result Std.U32
-- Trait declaration: [t05_default_supertrait::Greet]
structure Greet (Self : Type _) where
  NamedInst : Named Self
  greet : Self → Result Std.U32
-- [t05_default_supertrait::Greet::greet]:
@[trait_default]
def Greet.greet.default
  {Self : Type _} (GreetInst : Greet Self) (self : Self) : Result Std.U32 := do
  let i ← GreetInst.NamedInst.id self
  i + 1#u32
-- [t05_default_supertrait::P]
@[reducible]
def P := Unit
-- [t05_default_supertrait::{impl t05_default_supertrait::Named for t05_default_supertrait::P}::id]:
def P.Insts.T05_default_supertraitNamed.id (self : P) : Result Std.U32 := do
  ok 41#u32
-- Trait implementation: [t05_default_supertrait::{impl t05_default_supertrait::Named for t05_default_supertrait::P}]
@[reducible]
def P.Insts.T05_default_supertraitNamed : Named P := {
  id := P.Insts.T05_default_supertraitNamed.id
}
-- Trait implementation: [t05_default_supertrait::{impl t05_default_supertrait::Greet for t05_default_supertrait::P}]
@[reducible]
impl_def P.Insts.T05_default_supertraitGreet : Greet P := {
  NamedInst := P.Insts.T05_default_supertraitNamed
  greet := Greet.greet.default P.Insts.T05_default_supertraitGreet
}
-- [t05_default_supertrait::g]:
def g {T : Type _} (GreetInst : Greet T) (x : T) : Result Std.U32 := do
  GreetInst.greet x
end t05_default_supertrait
```


### T06_generic_struct_impl  —  **CLEAN**

```rust
pub struct Pair<A, B> { pub a: A, pub b: B }
impl<A: Copy, B: Copy> Pair<A, B> {
    pub fn swap(&self) -> Pair<B, A> { Pair { a: self.b, b: self.a } }
}
pub fn use_pair() -> u8 { Pair { a: 1u8, b: true }.swap().b }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t06_generic_struct_impl
-- [t06_generic_struct_impl::Pair]
structure Pair (A : Type _) (B : Type _) where
  a : A
  b : B
-- [t06_generic_struct_impl::{t06_generic_struct_impl::Pair<A, B>}::swap]:
def Pair.swap
  {A : Type _} {B : Type _} (coremarkerCopyInst : core.marker.Copy A)
  (coremarkerCopyInst1 : core.marker.Copy B) (self : Pair A B) :
  Result (Pair B A)
  := do
  ok { a := self.b, b := self.a }
-- [t06_generic_struct_impl::use_pair]:
def use_pair : Result Std.U8 := do
  let p ←
    Pair.swap core.marker.CopyU8 core.marker.CopyBool { a := 1#u8, b := true }
  ok p.b
end t06_generic_struct_impl
```


### T07_impl_trait_arg_ret  —  **CLEAN**

```rust
pub fn evens(n: u32) -> impl Fn(u32) -> bool { move |x| x % 2 == 0 && x < n }
pub fn apply(f: impl Fn(u32) -> u32, x: u32) -> u32 { f(x) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t07_impl_trait_arg_ret
-- [t07_impl_trait_arg_ret::evens::{closure}]
@[reducible]
def evens.closure := Std.U32
-- [t07_impl_trait_arg_ret::evens]:
def evens (n : Std.U32) : Result evens.closure := do
  ok n
-- [t07_impl_trait_arg_ret::evens::{impl core::ops::function::Fn<(u32,), bool> for t07_impl_trait_arg_ret::evens::{closure}}::call]:
def evens.closure.Insts.CoreOpsFunctionFnTupleU32Bool.call
  (c : evens.closure) (tupled_args : Std.U32) : Result Bool := do
  let i ← tupled_args % 2#u32
  if i = 0#u32
  then ok (tupled_args < c)
  else ok false
-- [t07_impl_trait_arg_ret::apply]:
def apply
  {T0 : Type _} (coreopsfunctionFnT0TupleU32U32Inst : core.ops.function.Fn T0
  Std.U32 Std.U32) (f : T0) (x : Std.U32) :
  Result Std.U32
  := do
  coreopsfunctionFnT0TupleU32U32Inst.call f x
end t07_impl_trait_arg_ret
```


### T08_generic_trait_static  —  **CLEAN**

```rust
pub trait Zero { fn zero() -> Self; }
impl Zero for u32 { fn zero() -> u32 { 0 } }
impl Zero for u64 { fn zero() -> u64 { 0 } }
pub fn z<T: Zero>() -> T { T::zero() }
pub fn both() -> (u32, u64) { (z(), z()) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Generated Lean:
```lean
namespace t08_generic_trait_static
-- Trait declaration: [t08_generic_trait_static::Zero]
structure Zero (Self : Type _) where
  zero : Result Self
-- [t08_generic_trait_static::{impl t08_generic_trait_static::Zero for u32}::zero]:
def U32.Insts.T08_generic_trait_staticZero.zero : Result Std.U32 := do
  ok 0#u32
-- Trait implementation: [t08_generic_trait_static::{impl t08_generic_trait_static::Zero for u32}]
@[reducible]
def U32.Insts.T08_generic_trait_staticZero : Zero Std.U32 := {
  zero := U32.Insts.T08_generic_trait_staticZero.zero
}
-- [t08_generic_trait_static::{impl t08_generic_trait_static::Zero for u64}::zero]:
def U64.Insts.T08_generic_trait_staticZero.zero : Result Std.U64 := do
  ok 0#u64
-- Trait implementation: [t08_generic_trait_static::{impl t08_generic_trait_static::Zero for u64}]
@[reducible]
def U64.Insts.T08_generic_trait_staticZero : Zero Std.U64 := {
  zero := U64.Insts.T08_generic_trait_staticZero.zero
}
-- [t08_generic_trait_static::z]:
def z {T : Type _} (ZeroInst : Zero T) : Result T := do
  ZeroInst.zero
-- [t08_generic_trait_static::both]:
def both : Result (Std.U32 × Std.U64) := do
  let i ← z U32.Insts.T08_generic_trait_staticZero
  let i1 ← z U64.Insts.T08_generic_trait_staticZero
  ok (i, i1)
end t08_generic_trait_static
```


### U01_raw_ptr  —  **FAIL**

```rust
pub fn rp(x: &mut u32) -> u32 { let p = x as *mut u32; unsafe { *p = 3; *p } }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Unsupported operation: &raw mut (*x)
[Warn] Could not translate the body of function 'u01_raw_ptr::rp because of previous error
```

Generated Lean:
```lean
namespace u01_raw_ptr
-- [u01_raw_ptr::rp]:
def rp (x : Std.U32) : Result (Std.U32 × Std.U32) := do
  sorry
end u01_raw_ptr
```


### U02_ptr_cast  —  **FAIL**

```rust
pub fn pc(x: &u32) -> u8 { let p = x as *const u32 as *const u8; unsafe { *p } }
pub fn addr(x: &u32) -> usize { x as *const u32 as usize }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Unsupported operation: &raw const (*x)
[Warn] Could not translate the body of function 'u02_ptr_cast::pc because of previous error
[Warn] Could not translate the body of function 'u02_ptr_cast::addr because of previous error
```

Generated Lean:
```lean
namespace u02_ptr_cast
-- [u02_ptr_cast::pc]:
def pc (x : Std.U32) : Result Std.U8 := do
  sorry
-- [u02_ptr_cast::addr]:
def addr (x : Std.U32) : Result Std.Usize := do
  sorry
end u02_ptr_cast
```


### U03_union  —  **FAIL**

```rust
pub union U { pub a: u32, pub b: f32 }
pub fn read(u: U) -> u32 { unsafe { u.a } }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] unions are not supported
[Error] Encountered an error when analyzing the following type declarations group:
[Error] Internal error: please file an issue
[Warn] Could not translate type decl 'u03_union::U because of previous error
[Warn] Could not translate the function signature of 'u03_union::read because of previous error
[Warn] Could not translate the function 'u03_union::read because of previous error.
```

Generated Lean:
```lean
namespace u03_union
end u03_union
```


### U04_transmute  —  **CAVEATS**

```rust
pub fn t(x: u32) -> [u8; 4] { unsafe { core::mem::transmute(x) } }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace u04_transmute
-- [core::intrinsics::transmute]:
@[rust_fun "core::intrinsics::transmute"]
axiom core.intrinsics.transmute
  {Src : Type _} (Dst : Type _) : Src → Result Dst
-- [u04_transmute::t]:
def t (x : Std.U32) : Result (Array Std.U8 4#usize) := do
  core.intrinsics.transmute (Array Std.U8 4#usize) x
end u04_transmute
```


### U05_unsafe_fn_ptr_read  —  **FAIL**

```rust
pub unsafe fn rd(p: *const u32) -> u32 { core::ptr::read(p) }
pub fn safe_call(x: &u32) -> u32 { unsafe { rd(x) } }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Unsupported operation: &raw const (*x)
[Warn] Could not translate the body of function 'u05_unsafe_fn_ptr_read::safe_call because of previous error
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace u05_unsafe_fn_ptr_read
-- [core::ptr::read]:
@[rust_fun "core::ptr::read"]
axiom core.ptr.read {T : Type _} : ConstRawPtr T → Result T
-- [u05_unsafe_fn_ptr_read::rd]:
def rd (p : ConstRawPtr Std.U32) : Result Std.U32 := do
  core.ptr.read p
-- [u05_unsafe_fn_ptr_read::safe_call]:
def safe_call (x : Std.U32) : Result Std.U32 := do
  sorry
end u05_unsafe_fn_ptr_read
```


### U06_static_mut  —  **FAIL**

```rust
pub static K: u32 = 7;
pub const C: u32 = 3;
pub static mut G: u32 = 0;
pub fn k() -> u32 { K + C }
pub fn g() -> u32 { unsafe { G += 1; G } }
```

Exit codes: charon=0, aeneas=1, lean=0.

Aeneas messages:
```text
[Error] Unexpected access to a global
[Warn] Could not translate the body of function 'u06_static_mut::g because of previous error
```

Generated Lean:
```lean
namespace u06_static_mut
-- [u06_static_mut::K]
@[global_simps, irreducible] def K : Std.U32 := 7#u32
-- [u06_static_mut::C]
@[global_simps, irreducible] def C : Std.U32 := 3#u32
-- [u06_static_mut::G]
@[global_simps, irreducible] def G : Std.U32 := 0#u32
-- [u06_static_mut::k]:
def k : Result Std.U32 := do
  K + C
-- [u06_static_mut::g]:
def g : Result Std.U32 := do
  sorry
end u06_static_mut
```


### X01_thread  —  **FAIL**

```rust
pub fn spawn() -> u32 { std::thread::spawn(|| 1u32).join().unwrap() }
```

Exit codes: charon=0, aeneas=0, lean=1.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Lean errors:
```text
../out/X01_thread/lean/X01Thread.lean:126:0: error: declaration `x01_thread.spawn` contains universe level metavariables at the expression
```

Generated Lean:
```lean
namespace x01_thread
-- [core::any::TypeId]
@[rust_type "core::any::TypeId"]
axiom core.any.TypeId : Type
-- Trait declaration: [core::any::Any]
@[rust_trait "core::any::Any"]
structure core.any.Any (Self : Type _) where
  type_id : Self → Result core.any.TypeId
-- [core::any::{impl core::fmt::Debug for (dyn core::any::Any + 'static)}::fmt]:
@[rust_fun "core::any::{core::fmt::Debug<@>}::fmt"]
axiom P.Insts.CoreFmtDebug.fmt
  :
  Dyn (fun _dyn => core.any.Any _dyn) → core.fmt.Formatter → Result
    ((core.result.Result Unit core.fmt.Error) × core.fmt.Formatter)
-- Trait implementation: [core::any::{impl core::fmt::Debug for (dyn core::any::Any + 'static)}]
@[reducible, rust_trait_impl "core::fmt::Debug<@>"]
def P.Insts.CoreFmtDebug : core.fmt.Debug (Dyn (fun _dyn => core.any.Any _dyn))
  := {
  fmt := P.Insts.CoreFmtDebug.fmt
}
-- [std::thread::join_handle::JoinHandle]
@[rust_type "std::thread::join_handle::JoinHandle"]
axiom std.thread.join_handle.JoinHandle (T : Type _) : Type
-- [std::thread::functions::spawn]:
@[rust_fun "std::thread::functions::spawn"]
axiom std.thread.functions.spawn
  {F : Type _} {T : Type _} (coreopsfunctionFnOnceFTupleTInst :
  core.ops.function.FnOnce F Unit T) :
  F → Result (std.thread.join_handle.JoinHandle T)
-- [std::thread::join_handle::{std::thread::join_handle::JoinHandle<T>}::join]:
@[rust_fun
  "std::thread::join_handle::{std::thread::join_handle::JoinHandle<@T>}::join"]
axiom std.thread.join_handle.JoinHandle.join
  {T : Type _} :
  std.thread.join_handle.JoinHandle T → Result (core.result.Result T (Dyn
    (fun _dyn => core.any.Any _dyn)))
-- [alloc::boxed::{impl core::fmt::Debug for alloc::boxed::Box<T>}::fmt]:
@[rust_fun "alloc::boxed::{core::fmt::Debug<Box<@T>>}::fmt"]
axiom Box.Insts.CoreFmtDebug.fmt
  {T : Type _} (A : Type _) (corefmtDebugInst : core.fmt.Debug T) :
  T → core.fmt.Formatter → Result ((core.result.Result Unit core.fmt.Error)
    × core.fmt.Formatter)
-- Trait implementation: [alloc::boxed::{impl core::fmt::Debug for alloc::boxed::Box<T>}]
@[reducible, rust_trait_impl "core::fmt::Debug<Box<@T>>"]
def Box.Insts.CoreFmtDebug {T : Type _} (A : Type _) (corefmtDebugInst :
-- ... (25 more lines, see experiments/out/)
```


### X02_mutex  —  **CAVEATS**

```rust
use std::sync::Mutex;
pub fn m(x: &Mutex<u32>) { *x.lock().unwrap() += 1 }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace x02_mutex
-- [std::sync::poison::mutex::Mutex]
@[rust_type "std::sync::poison::mutex::Mutex"]
axiom std.sync.poison.mutex.Mutex (T : Type _) : Type
-- [std::sync::poison::mutex::MutexGuard]
@[rust_type "std::sync::poison::mutex::MutexGuard"]
axiom std.sync.poison.mutex.MutexGuard (T : Type _) : Type
-- [std::sync::poison::PoisonError]
@[rust_type "std::sync::poison::PoisonError"]
axiom std.sync.poison.PoisonError (T : Type _) : Type
-- [std::sync::poison::mutex::{std::sync::poison::mutex::Mutex<T>}::lock]:
@[rust_fun
  "std::sync::poison::mutex::{std::sync::poison::mutex::Mutex<@T>}::lock"]
axiom std.sync.poison.mutex.Mutex.lock
  {T : Type _} :
  std.sync.poison.mutex.Mutex T → Result (core.result.Result
    (std.sync.poison.mutex.MutexGuard T) (std.sync.poison.PoisonError
    (std.sync.poison.mutex.MutexGuard T)))
-- [std::sync::poison::mutex::{impl core::ops::deref::DerefMut<T> for std::sync::poison::mutex::MutexGuard<'_0, T>}::deref_mut]:
@[rust_fun
  "std::sync::poison::mutex::{core::ops::deref::DerefMut<std::sync::poison::mutex::MutexGuard<'0, @T>, @T>}::deref_mut"]
axiom std.sync.poison.mutex.MutexGuard.Insts.CoreOpsDerefDerefMut.deref_mut
  {T : Type _} :
  std.sync.poison.mutex.MutexGuard T → Result (T × (T →
    std.sync.poison.mutex.MutexGuard T))
-- [std::sync::poison::{impl core::fmt::Debug for std::sync::poison::PoisonError<T>}::fmt]:
@[rust_fun
  "std::sync::poison::{core::fmt::Debug<std::sync::poison::PoisonError<@T>>}::fmt"]
axiom std.sync.poison.PoisonError.Insts.CoreFmtDebug.fmt
  {T : Type _} :
  std.sync.poison.PoisonError T → core.fmt.Formatter → Result
    ((core.result.Result Unit core.fmt.Error) × core.fmt.Formatter)
-- Trait implementation: [std::sync::poison::{impl core::fmt::Debug for std::sync::poison::PoisonError<T>}]
@[reducible, rust_trait_impl
  "core::fmt::Debug<std::sync::poison::PoisonError<@T>>"]
def std.sync.poison.PoisonError.Insts.CoreFmtDebug (T : Type _) :
  core.fmt.Debug (std.sync.poison.PoisonError T) := {
  fmt := std.sync.poison.PoisonError.Insts.CoreFmtDebug.fmt
}
-- [x02_mutex::m]:
def m (x : std.sync.poison.mutex.Mutex Std.U32) : Result Unit := do
  let r ← std.sync.poison.mutex.Mutex.lock x
  let mg ←
    core.result.Result.unwrap (std.sync.poison.PoisonError.Insts.CoreFmtDebug
      (std.sync.poison.mutex.MutexGuard Std.U32)) r
-- ... (5 more lines, see experiments/out/)
```


### X03_atomic  —  **CAVEATS**

```rust
use std::sync::atomic::{AtomicU32, Ordering};
pub fn a(x: &AtomicU32) -> u32 { x.fetch_add(1, Ordering::SeqCst) }
```

Exit codes: charon=0, aeneas=0, lean=0.

Aeneas messages:
```text
[Warn] The crate contains extracted external, unknown definitions: we advise using the option -split-files to allow manually providing these definitions in separate files.
```

Generated Lean:
```lean
namespace x03_atomic
-- [core::sync::atomic::private::Align4]
@[rust_type "core::sync::atomic::private::Align4"]
axiom core.sync.atomic.private.Align4 (T : Type _) : Type
-- [core::sync::atomic::Atomic]
@[rust_type "core::sync::atomic::Atomic"]
axiom core.sync.atomic.Atomic (T : Type _) (Clause0_Storage : Type _) : Type
-- [core::sync::atomic::Ordering]
@[discriminant isize, rust_type "core::sync::atomic::Ordering"]
inductive core.sync.atomic.Ordering where
| Relaxed : core.sync.atomic.Ordering
| Release : core.sync.atomic.Ordering
| Acquire : core.sync.atomic.Ordering
| AcqRel : core.sync.atomic.Ordering
| SeqCst : core.sync.atomic.Ordering
-- [core::sync::atomic::{core::sync::atomic::Atomic<u32, core::sync::atomic::private::Align4<u32>>}::fetch_add]:
@[rust_fun
  "core::sync::atomic::{core::sync::atomic::Atomic<u32, core::sync::atomic::private::Align4<u32>>}::fetch_add"]
axiom core.sync.atomic.AtomicU32Align4U32.fetch_add
  :
  core.sync.atomic.Atomic Std.U32 (core.sync.atomic.private.Align4 Std.U32) →
    Std.U32 → core.sync.atomic.Ordering → Result Std.U32
-- [x03_atomic::a]:
def a
  (x : core.sync.atomic.Atomic Std.U32 (core.sync.atomic.private.Align4
  Std.U32)) :
  Result Std.U32
  := do
  core.sync.atomic.AtomicU32Align4U32.fetch_add x 1#u32
    core.sync.atomic.Ordering.SeqCst
end x03_atomic
```


### X04_async  —  **FAIL**

```rust
pub async fn f(x: u32) -> u32 { x + 1 }
pub async fn g() -> u32 { f(1).await }
```

Exit codes: charon=0, aeneas=1, lean=0.

Charon:
```text
warning: Coroutine types are not supported yet
warning: Coroutines are not supported
warning: The extraction generated 4 warnings
```

Aeneas messages:
```text
[Error] Found type error in the output of charon
[Warn] Could not translate the function signature of 'x04_async::f because of previous error
[Warn] Could not translate the function signature of 'x04_async::g because of previous error
[Error] Internal error: please file an issue
[Warn] Could not translate the function 'x04_async::g because of previous error.
[Warn] Could not translate the function 'x04_async::f because of previous error.
```

Generated Lean:
```lean
namespace x04_async
end x04_async
```


