---
title: "What Aeneas supports today: an evidence-based report"
subtitle: "Aeneas (Rust → Lean) and its front-end Charon, tested on 2026-11-10"
date: "2026-11-10"
---

# How to read this report

**Audience.** Engineers who know formal verification (e.g. the K framework) but have not used Aeneas.

**The question.** An LLM claimed that "Aeneas does not support loops and iterators". This report checks that claim, and Aeneas's other limits, against evidence.

**Short answer.** The claim is **false**, though it has a real core.

- **What works.** `while`, `loop`/`break`, `continue`, early `return`, nested loops, and `for` over ranges, slices, `Vec` and iterators all translate to Lean that type-checks. Loops that mutate through `&mut` and loops that return borrows also work.
- **Proofs work too.** I proved termination and functional correctness for two of the translated loops in Lean (§2.5).
- **What is missing:**
  - **Loop control flow:**
    - `break`/`continue` to an *outer* loop.
    - `return` inside a *nested* loop.
    - Some sequences of loops that each contain an early `return`.
    - Loops with no exit at all.
  - **Iterator adaptors.** Several common adaptors (`map`, `filter`, `fold`, `sum`, `any`, `count`) have no Lean model. Aeneas then emits `axiom`s, or Lean that does not type-check.

**Evidence labels.** Every claim carries one or more of these:

| Label | Meaning |
|---|---|
| **[doc]** | Stated in documentation in the repository at the commit tested. |
| **[test]** | Observed in an experiment run for this report (`experiments/` in this repository). |
| **[src]** | Read from source code at the commit tested (`file:line`). |
| **[issue]** | Taken from a GitHub issue or PR (number, state and dates as of 2026-11-10). |

**Path conventions.**

- `aeneas/…` paths refer to AeneasVerif/aeneas at commit `0d21a9e`.
- `charon/…` paths refer to AeneasVerif/charon at commit `454586e` (the commit Aeneas pins; see §1.1).
- Issue numbers without a prefix are Aeneas issues: `https://github.com/AeneasVerif/aeneas/issues/N` (PRs live at the same number under `/pull/N`). Charon issues are written `charon#N`.

**Rule followed.** I used no prior knowledge of Aeneas as a source. Where I could not check something, I say so.

\newpage

# 1. One-page summary

## 1.1 Versions tested

| Component | Version / commit | Notes |
|---|---|---|
| Aeneas (upstream `main`) | `0d21a9ec7bbbb110110cacc14464a47fd71c8705`, 2026-11-09 | "Update to leanprover/lean4 v4.36.0 rc2 (#1547)". Built from source. `aeneas -version` prints `aeneas 0d21a9ec`. |
| Charon used by Aeneas | `454586e9b770231e66d2062993e632dfc06463e2`, 2026-11-08 | Pinned in `aeneas/charon-pin`. `charon version` prints `0.1.315`. Built from source with Rust `nightly-2026-09-17`. |
| Charon upstream `main` | `b9da4e3a108a5c2092fe80162bbd20ca1986d616`, 2026-11-09 | Cloned and recorded but **not used**: Aeneas requires the exact pinned Charon (`aeneas/README.md:53-63`, `aeneas/charon-pin`). |
| Lean | 4.36.0-rc2 | From `aeneas/backends/lean/lean-toolchain`. Mathlib `9d581df` (lake manifest). |
| OCaml | 5.3.0 | Isolated opam root inside the work dir. `visitors 20260520` and `ppx_deriving_hash 0.1.5` were pinned, as `charon/flake.nix:203-212` and `charon/nix/charon-ml.nix:13-26` require. |

**Local install found on this machine (not used).**

- A pre-existing Aeneas checkout is at `3a8586f` (2026-07-25). `aeneas -version` prints `3a8586f-dirty` because a local file is modified. That checkout pins Charon `527ea8e`.
- Its `bin/charon` is a dangling symlink to `/tmp/charon-pinned`.
- A pre-existing Charon checkout is at `909ff09` (2026-07-06) and its binary reports `0.1.220`.
- The local install is about 3.5 months behind upstream and its Charon link is broken. Everything was therefore tested against **upstream main**, built inside a dedicated work directory (the root of this repository). No existing repo on the machine was modified.

## 1.2 Bottom line on loops and iterators

The status labels used in the tables below:

- **Clean:** translates with no axioms and the Lean type-checks.
- **Caveats:** the Lean type-checks, but contains `axiom`s for missing library models, or `sorry`.
- **Fails:** Aeneas reports an error, or the generated Lean does not type-check.

38 loop/iterator programs were run **[test]**:

| Status | Count |
|---|---|
| Clean | 24 |
| Caveats | 7 |
| Fails | 7 |

Running them again with `-loops-to-rec` gave the same statuses.

## 1.3 Feature status table

| Feature | Status | Evidence |
|---|---|---|
| `while`, `loop`+`break`, `continue` | **Supported** | [test] L01–L03; [doc] `aeneas/documentation/aeneas-overview.md:135-191` |
| Early `return` inside a (single) loop | **Supported** (some shapes fail) | [test] L04, L27, L28 clean; L29 fails; [issue] #822 open, #1501 open |
| Nested loops (no cross-level jumps) | **Supported** | [test] L05 |
| `break`/`continue` to an outer loop, `return` in a nested loop | **Not supported** | [doc] `aeneas/README.md:128-131`; [src] `aeneas/src/PrePasses.ml:636-648`; [test] L05b/c/d |
| Loop with no exit (`loop {}`) | **Not supported** | [src] `aeneas/src/interp/InterpLoops.ml:409-412`; [test] L26 |
| `for` over a range (`..`, `..=`, `step_by`, `rev`) | **Supported** | [test] L06, L06b, L17, L30 |
| `for` over a slice / `&Vec` / `Vec` / `v.iter()` / `iter_mut()` / `into_iter()` | **Supported**; `for x in &vec` gives one axiom | [test] L07, L08, L08b, L08c, L10, L11, L12 |
| `for` over an array (`[T; N]`, `&[T; N]`) | **Caveats** (array `IntoIter` is an axiom) | [test] L09 |
| `enumerate`, `zip`, `rev`, `step_by`, `collect` (from a range) | **Supported** | [test] L15, L16, L17, L06b, L20 |
| `map`, `filter`, `fold`, `sum`, `any`, `count` | **Not modelled**: axioms; `map`+`collect` gives ill-typed Lean | [test] L13, L14, L18, L19, L21; [issue] #1043, #1510, #1542, #1544, #1552 (open) |
| User-defined `Iterator` impl in a `for` loop | **Supported** | [test] L25 |
| Loop mutating through `&mut`; loop returning a borrow | **Supported** | [test] L11, L11b, L22, L23 |
| Termination / correctness proofs of loops | **Supported**, extrinsic, by the user | [test] proofs in §2.5 (Lean-checked); [src] `aeneas/backends/lean/Aeneas/Std/WP.lean:757-797` |
| Traits, generics, associated types, const generics, `impl Trait`, supertraits | **Supported** | [test] T01–T03, T05–T08 |
| `dyn Trait` method calls | **Not supported** (body becomes `sorry`) | [test] T04, T04b; [issue] #1500 open |
| GATs | **Not supported** | [src] `aeneas/src/symbolic/SymbolicToPureTypes.ml:131`; `charon/…/expand_associated_types.rs:1145-1150`; [issue] #1112 open |
| Closures (`Fn`, `FnOnce`, captured env, returned `impl Fn`) | **Supported** | [test] C01, C02, C05, T07 |
| `FnMut` closures | **Fails** (ill-typed Lean) | [test] C03; [issue] #960 open |
| Function pointers | **Not supported** | [test] C04; [src] `aeneas/src/symbolic/SymbolicToPureTypes.ml:190,422` |
| `&mut &mut T`, structs holding `&mut`, returned borrows, two-phase borrows | **Supported** | [test] B01–B03, B05, B06 |
| Generic type instantiated with `&mut` (`id(&mut x)`) | **Fails** (no Aeneas error, ill-typed Lean) | [doc] `aeneas/README.md:132-135`; [test] B04 |
| Arrays, slices, `Box`, `Option`/`Result`, `?`, structs/enums/match | **Supported** (some std methods are axioms) | [test] D01, D02, D05–D07 |
| `Vec` | **Supported**; `pop`, `remove`, `vec![..]` partly axioms | [test] D03 |
| `String`/`&str` | **Caveats**: every operation tested is an axiom | [test] D04 |
| Integer arithmetic, overflow checks, wrapping/checked/overflowing/saturating, bit ops, `as` between integers, `bool as u8` | **Supported** | [test] A01, A02, A04 (`count_ones` is an axiom) |
| `u8 as char` | **Fails** (internal "Unreachable", broken Lean file) | [test] A03 |
| Floats | **Not supported** | [test] A06; [issue] #828 open |
| Panics, `assert!`, `unwrap`, `expect`, `unreachable!` | **Supported** (modelled as `fail`) | [test] A05 |
| Recursion, mutual recursion (`even`/`odd`) | **Supported** (`partial_fixpoint`) | [test] R01, R02 |
| Mutual recursion over a mutually inductive type | **Fails** in Lean (monotonicity) | [test] R03 |
| Raw-pointer creation or dereference, pointer→int casts | **Not supported** | [test] U01, U02, U05; [src] `aeneas/src/interp/InterpPaths.ml:191-197` |
| Unions | **Not supported** | [test] U03; [src] `aeneas/src/llbc/TypesAnalysis.ml:544` |
| `transmute` | Becomes an **axiom** (no error) | [test] U04 |
| `static mut` | **Not supported** (`static`/`const` are fine) | [test] U06 |
| `Cell`, `RefCell`, `Rc`, `Arc` | **Opaque axioms only**: types and methods are `axiom`s | [test] I01–I04; [issue] #1525 open |
| Threads, `Mutex`, atomics | **Opaque axioms only**; `thread::spawn` gives ill-typed Lean | [test] X01–X03; [doc] `aeneas/README.md:137-139` |
| `async` | **Not supported** | [test] X04; [src] `charon/src/bin/charon-driver/translate/translate_bodies.rs:1487` |
| `extern "C"` functions, `--opaque` modules | **Supported as axioms** (by design) | [test] F01, F02 |
| `HashMap`, `BTreeMap`, `VecDeque`, `format!`, `sort` | **Axioms only** (no models) | [test] S01, S04–S06 |
| `Drop` impls | **Silently ignored** (never called) | [test] S08; [src] `aeneas/src/Config.ml:625-627`; [issue] #1410 open |

**Important behaviour for tool users [test][src].** By default, an item that Aeneas cannot translate does not stop the run. Instead:

- An untranslatable body is emitted as a Lean `axiom` (e.g. L05b) or as `sorry` (e.g. T04).
- Aeneas exits with status 1, but the generated Lean may still type-check.
- A green `lake build` therefore does **not** mean every function was translated.

To make the first error fatal, pass `-abort-on-error`: `aeneas/src/Main.ml:148`, and the error helpers in `aeneas/src/Errors.ml:121-201`.

\newpage

# 2. Loops and iterators in detail

## 2.1 What the documentation says

**README [doc].** `aeneas/README.md:126-131` lists one loop limitation:

> "**loops**: `return` inside *nested* loops, or `break`/`continue` to *outer* loops are not supported yet (e.g., `'a : loop { loop { break 'a; } } `). This is a technical limitation, not a fundamental issue…"

**Two translation modes [doc].** These are described in `aeneas/documentation/aeneas-overview.md:135-191` and `aeneas/documentation/tips-and-tricks.md:13-26`.

- **Default:** a `loop` fixed-point combinator over `ControlFlow`. Proofs use `loop.spec_decr_nat` or `loop.spec`.
- **`-loops-to-rec`:** a directly recursive `_loop` function marked `partial_fixpoint`. Proofs add `termination_by` and `decreasing_by`.
- The docs currently recommend `-loops-to-rec` for proof work. Quoting `tips-and-tricks.md:23-26`: "the proof infrastructure for it [the combinator] is not yet fully developed. Until it matures, **use `-loops-to-rec`**".

**Outdated user docs [doc].** `aeneas/docs/user/src/README.md:11-12` still says "loops: no nested loops for now" and "no functions pointers/closures". The experiments below contradict both statements for nested loops and for closures (function pointers do fail). That file is outdated.

**Charon's control-flow model [doc].** `charon/docs/limitations.md:31-33` says Charon does not distinguish `loop`, `while` and `for`; "only semantic information is kept".

- A `for` loop therefore reaches Aeneas as a `loop` that calls `Iterator::next` and matches on the result. This is visible in every `for` example below **[test]**.

## 2.2 How a loop is translated (pipeline)

Read from source **[src]**. The source-survey agent mapped these line references; I re-checked the ones marked ✓.

1. **Charon** turns the Rust MIR control-flow graph back into structured `loop`/`break`/`continue` statements.
   - Irreducible control flow is reported as an error (`charon/src/transform/control_flow/ullbc_to_llbc.rs:1362-1370`).
   - **Charon has no restriction on labelled breaks**: they are represented with a depth index.
2. **Aeneas pre-pass `update_loops`** (`aeneas/src/PrePasses.ml:392-680`). It rewrites a `return` inside a loop into a `break` whose exit path carries the return. It rejects the following:
   - ✓ `break` to an outer loop: "Breaks to outer loops are not supported yet" (`:637`).
   - ✓ `continue` to an outer loop: "Continue to outer loops are not supported yet" (`:641`).
   - ✓ `return` at loop depth > 1: "Returns inside of nested loops are not supported yet" (`:644-648`).
   - ✓ Some early-return shapes: "Early returns inside of loops are not supported yet" (`:571-576`).
3. **Symbolic execution with a fixed point** (`aeneas/src/interp/InterpLoopsFixedPoint.ml`, `InterpLoops.ml`).
   - The loop body is executed symbolically. The abstract states reaching `continue` are joined with the entry state until the state stops changing. ✓ This is capped at `loop_fixed_point_max_num_iters = 2` (`aeneas/src/Config.ml:182`).
   - Borrows that flow through the loop become "region abstractions". This is what lets loops that mutate through `&mut`, or return borrows, become pure functions that also return a "backward" function (see L11 and L23 below).
   - ✓ A loop with no `break` is rejected: "(Infinite) loops which do not contain breaks are not supported yet" (`aeneas/src/interp/InterpLoops.ml:409-412`).
4. **Pure-code passes** (`aeneas/src/pure/PureMicroPassesLoops.ml`).
   - By default the loop becomes a `loop` combinator application, with the body lifted into `<fn>_loop.body`.
   - The loop instead becomes a directly recursive function when `-loops-to-rec` is given, or when a heuristic holds. ✓ `loop_satisfies_recursive_criteria_aux` (`:2475`) uses `loop_rel_has_termination_measure` (`:2436`); per [issue] PR #779, this applies to loops over recursive data structures.
   - Observed **[test]**: L23 (a `while let` walking a linked list) was emitted as a recursive `partial_fixpoint` function even in default mode. All other loops used the combinator.
5. **Lean extraction.** `loop` is the library combinator. ✓ `aeneas/backends/lean/Aeneas/Std/Primitives.lean:452-457`:

```lean
def loop {α : Type u} {β : Type v} (body : α → Result (ControlFlow α β)) (x : α) : Result β := do
  bind (body x) fun r =>
  match r with
  | ControlFlow.cont x => loop body x
  | ControlFlow.done x => ok x
partial_fixpoint
```

The `Result` monad and its effects:

- ✓ `Result α` is an interaction tree with three outcomes: `ok`, `fail e` (panic, overflow, out-of-bounds) and `div` (`Primitives.lean:68-84`).
- Because `loop` and recursive functions are defined by `partial_fixpoint`, **Lean accepts the definition without a termination proof**.
- Non-termination is a value (`div`). Termination is a theorem the user proves afterwards (§2.5).
- Aeneas does not use a `divergent` keyword. Lean is not supported by the fuel-based alternative `-use-fuel` (`aeneas/src/Main.ml:532-535`) [src].

## 2.3 Results for all loop and iterator experiments

**Columns.** `A`/`L` are the Aeneas and Lean exit codes. "Axioms" counts `axiom` declarations in the generated file; these are missing std-library models, or the user's own function when it could not be translated. Every program is listed in full, with its Lean output, in **Appendix A**.

| Case | Rust feature | A | L | Axioms | Status | Notes |
|---|---|---|---|---|---|---|
| L01 | `while` | 0 | 0 | 0 | Clean | `loop` combinator |
| L02 | `loop` + `break` | 0 | 0 | 0 | Clean | |
| L03 | `continue` | 0 | 0 | 0 | Clean | |
| L04 | early `return` in `while` | 0 | 0 | 0 | Clean | `done (some i)` |
| L05 | nested `while` | 0 | 0 | 0 | Clean | two `_loop`s |
| L05b | `break 'outer` | 1 | 0 | 1 | **Fails** | "Breaks to outer loops are not supported yet"; fn becomes `axiom` |
| L05c | `return` in inner loop | 1 | 0 | 1 | **Fails** | "Returns inside of nested loops are not supported yet" |
| L05d | `continue 'outer` | 1 | 0 | 1 | **Fails** | "Continue to outer loops are not supported yet" |
| L06 | `for i in 0..n` | 0 | 0 | 0 | Clean | `IteratorRange.next` |
| L06b | `0..=n`, `(0..n).step_by(2)` | 0 | 0 | 0 | Clean | |
| L07 | `for x in slice` | 0 | 0 | 0 | Clean | `slice::Iter` |
| L07b | index loops over slice | 0 | 0 | 0 | Clean | |
| L08 | `for x in &vec` | 0 | 0 | 1 | Caveats | `IntoIterator for &Vec` is an axiom |
| L08b | `for x in vec` (owned) | 0 | 0 | 0 | Clean | `vec::IntoIter` modelled |
| L08c | `vec.iter()`, `vec.iter_mut()` | 0 | 0 | 0 | Clean | via `Vec.deref` |
| L09 | `for x in [T;4]` / `&[T;4]` | 0 | 0 | 3 | Caveats | `array::IntoIter` and its `next` are axioms |
| L10 | `slice.iter()` | 0 | 0 | 0 | Clean | |
| L11 | `slice.iter_mut()` | 0 | 0 | 0 | Clean | backward function threaded through loop |
| L11b | index loop mutating slice | 0 | 0 | 0 | Clean | |
| L12 | `vec.into_iter()` | 0 | 0 | 0 | Clean | |
| L13 | `.iter().map(..).collect()` | 0 | **1** | 3 | **Fails** | Lean: "`map` is not a field of structure `Iterator`" |
| L14 | `.iter().filter(..).count()` | 0 | 0 | 5 | Caveats | `Filter`, `filter`, `count` are axioms |
| L15 | `.enumerate()` | 0 | 0 | 0 | Clean | |
| L16 | `.zip()` | 0 | 0 | 1 | Caveats | zip modelled; `&u32 * &u32` is an axiom |
| L17 | `(0..n).rev()` | 0 | 0 | 0 | Clean | |
| L18 | `.fold(0, \|a,x\| a+x)` | 0 | 0 | 2 | Caveats | `Iter::fold` is an axiom |
| L19 | `.sum()` | 0 | 0 | 2 | Caveats | `sum` is an axiom |
| L20 | `(0..n).collect::<Vec<_>>()` | 0 | 0 | 0 | Clean | `FromIterator for Vec` modelled |
| L21 | closures capturing env in `map`/`any` | 0 | **1** | 4 | **Fails** | same error as L13 |
| L22 | loop mutating `&mut Vec` / `&mut struct` | 0 | 0 | 0 | Clean | |
| L23 | loop returning `&mut` into a list; returning `&` into a slice | 0 | 0 | 0 | Clean | list loop becomes recursive fn with backward closure |
| L24 | `while let Some(x) = v.pop()` | 0 | 0 | 1 | Caveats | `Vec::pop` is an axiom |
| L25 | user `impl Iterator` in `for` | 0 | 0 | 0 | Clean | |
| L26 | `loop {}` (no exit) | 1 | 0 | 0 | **Fails** | "(Infinite) loops which do not contain breaks are not supported yet" |
| L27 | `loop` exited only by `return` | 0 | 0 | 0 | Clean | |
| L28 | `return` and `break` in one loop, code after | 0 | 0 | 0 | Clean | |
| L29 | two sequential loops, each with `return` | 1 | 0 | 1 | **Fails** | "Breaks to outer loops are not supported yet" (reproduces [issue] #1501) |
| L30 | `for _ in 0..n { c += 1 }` | 0 | 0 | 0 | Clean | used for the proof in §2.5 |

**`-loops-to-rec` [test].** Re-running all L-cases with `-loops-to-rec` (`experiments/out-loops-to-rec/`, `experiments/results-loops-to-rec.md`) produced exactly the same exit codes. Only the shape of the Lean changed: direct recursion plus `partial_fixpoint` instead of `loop`.

**Warnings in clean range cases [test].** Every range case logs two Aeneas warnings, for example:

> "When retrieving the builtin information for trait decl 'core::iter::range::Step', could not find the information for item 'forward_overflowing'. The model defined in the Lean library seems to be missing the corresponding field."

This matches open [issue] #1514. It did not stop the output from type-checking.

## 2.4 Selected cases: Rust input, Lean output, explanation

### (a) `while` loop → `loop` combinator (L01, default mode) [test]

```rust
pub fn sum_to(n: u32) -> u32 {
    let mut i = 0; let mut s = 0;
    while i < n { s += i; i += 1; }
    s
}
```

```lean
def sum_to_loop.body (n : Std.U32) (i : Std.U32) (s : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32) := do
  if i < n
  then let s1 ← s + i          -- `+` is checked: overflow ⇒ `fail`
       let i1 ← i + 1#u32
       ok (cont (i1, s1))      -- next iteration with the new loop state
  else ok (done s)             -- exit with the loop's result

def sum_to_loop (n : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  loop (fun (i1, s1) => sum_to_loop.body n i1 s1) (i, s)

def sum_to (n : Std.U32) : Result Std.U32 := do
  sum_to_loop n 0#u32 0#u32
```

**How the translation works.**

- The mutable variables live across iterations (`i`, `s`), so they become the loop state (a tuple).
- Each iteration returns `cont state'` or `done result`.
- Arithmetic is checked: `s + i` returns `fail` on overflow. Proving absence of overflow is part of proving the function correct.

### (b) Same loop with `-loops-to-rec` (L01) [test]

```lean
def sum_to_loop (n : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  if i < n
  then let s1 ← s + i
       let i1 ← i + 1#u32
       sum_to_loop n i1 s1
  else ok s
partial_fixpoint
```

### (c) `for` over a range (L06) [test]

The `for` loop is desugared by rustc and Charon into calls to `Iterator::next`. The iterator becomes part of the loop state.

```rust
pub fn sum_range(n: u32) -> u32 { let mut s = 0; for i in 0..n { s += i; } s }
```

```lean
def sum_range_loop.body (iter : core.ops.range.Range Std.U32) (s : Std.U32) :
  Result (ControlFlow ((core.ops.range.Range Std.U32) × Std.U32) Std.U32) := do
  let (o, iter1) ← core.iter.range.IteratorRange.next core.iter.range.StepU32 iter
  match o with
  | none => ok (done s)
  | some i => let s1 ← s + i
              ok (cont (iter1, s1))

def sum_range (n : Std.U32) : Result Std.U32 := do
  sum_range_loop { start := 0#u32, «end» := n } 0#u32
```

`IteratorRange.next` is a **defined** (not axiomatised) model in the Lean library: `aeneas/backends/lean/Aeneas/Std/Core/Iter.lean:604` [src]. It has `@[step]` specification lemmas (`aeneas/backends/lean/Aeneas/Std/RangeIter.lean:263-276`), added by PR #1172, merged 2026-06-30 [src][issue].

**Docs and issues vs experiment.**

- Open [issue] #1132 (opened 2026-06-17) still says "IteratorRange::next has no reasoning lemma, range-for loops can't be proved".
- The lemma does exist at the tested commit, and I used it in the proof of L30 (§2.5) **[test]**. The issue appears to be out of date.

### (d) Mutation through an iterator over `&mut [T]` (L11) [test]

```rust
pub fn incr_all(s: &mut [u32]) { for x in s.iter_mut() { *x = x.wrapping_add(1); } }
```

```lean
def incr_all_loop.body (iter : core.slice.iter.IterMut Std.U32)
  (back : core.slice.iter.IterMut Std.U32 → core.slice.iter.IterMut Std.U32) : ... := do
  let (o, iter1, next_back) ← core.slice.iter.IteratorIterMut.next iter
  match o with
  | none => ok (done (let im := next_back iter1 none
                      back im))
  | some x =>
    let x1 ← lift (core.num.U32.wrapping_add x 1#u32)
    ok (cont (iter1, fun im => let im1 := next_back im (some x1)
                               back im1))

def incr_all (s : Slice Std.U32) : Result (Slice Std.U32) := do
  let (iter, iter_mut_back) ← core.slice.Slice.iter_mut s
  let back ← incr_all_loop iter (fun im => im)
  ok (iter_mut_back back)
```

**How the translation works.**

- `&mut` borrows are not modelled with a heap. Each mutable borrow becomes a value plus a **backward function** that puts the updated value back into its owner.
- Inside the loop, the backward functions are composed into the loop state (`back`).
- After the loop, the composed function is applied to rebuild the updated slice.
- The user reasons about this purely functionally. No separation logic is needed.

### (e) Loop that returns a mutable borrow (L23) [test]

```rust
pub fn nth_mut<T>(mut l: &mut List<T>, mut i: u32) -> Option<&mut T> {
    while let List::Cons(x, tl) = l {
        if i == 0 { return Some(x); }
        i -= 1; l = tl;
    }
    None
}
```

```lean
def nth_mut_loop {T : Type _} (l : List T) (i : Std.U32) :
  Result ((Option T) × (Option T → List T)) := do
  match l with
  | List.Cons x tl =>
    if i = 0#u32
    then ok (some x, fun o => let t := match o with | some t1 => t1 | _ => x
                              List.Cons t tl)
    else
      let i1 ← i - 1#u32
      let (o, back) ← nth_mut_loop tl i1
      ok (o, fun o1 => let l1 := back o1
                        List.Cons x l1)
  | List.Nil => ok (none, fun o => List.Nil)
partial_fixpoint
```

**How the translation works.**

- The returned `&mut T` becomes a pair: the current value, and a function that writes a new value back into the list.
- This loop walks a recursive structure, so Aeneas emitted it as a recursive function even in default mode (see the heuristic in §2.2, step 4).

### (f) Iterator adaptor without a model (L13) [test]

```rust
pub fn doubled(s: &[u32]) -> Vec<u32> { s.iter().map(|x| x * 2).collect() }
```

Aeneas succeeds (exit 0) and emits axioms such as `core.iter.traits.iterator.Iterator.map.default` and `core.iter.adapters.map.Map.Insts.CoreIterTraitsIteratorIterator.next`. Lean then rejects the file:

```text
L13Map.lean:65:2: error: `map` is not a field of structure `core.iter.traits.iterator.Iterator`
L13Map.lean:71:2: error: `collect` is not a field of structure `core.iter.traits.iterator.Iterator`
```

**Cause [src].** The Lean model of the `Iterator` trait has only the fields `next`, `step_by`, `enumerate` and `take`. The file carries the comments "TODO: adding more fields like rev leads to a circularity" and "TODO: collect" (`aeneas/backends/lean/Aeneas/Std/Core/Iter.lean:92-105`). The generated `Map` instance tries to fill `map :=` and `collect :=` fields that do not exist.

**Tracking [issue].**

- This is open issue #1043, opened 2026-05-20.
- Open PRs that add models: #1510 (`map`, `chain`), #1542 (`fold`), #1544 (`any`), #1552 (`count`).

**Workaround.** Rewrite the adaptor as an explicit `for`/`while` loop. Every loop form in L01–L12 is supported.

### (g) What fails: cross-level control flow (L05b, L29) [test]

```rust
'outer: while i < n { ... while j < n { if i + j == t { r = i; break 'outer; } j += 1; } i += 1; }
```

```text
[Error] Breaks to outer loops are not supported yet
Source: 'cases/L05b_nested_break_outer.rs', lines 1:0-16:1
[Error] Ignoring the body of 'l05b_nested_break_outer::find_pair' because of previous error
```

The function is emitted as `axiom find_pair : Std.U32 → Std.U32 → Result Std.U32`, and the Lean file still type-checks.

L29 has **no** labelled break in the source. It has two loops in sequence, each with an early `return`, and it fails with the same message. This reproduces open [issue] #1501, opened 2026-10-30. The issue explains that Charon places the second loop inside the first loop's exit branch.

## 2.5 What the user has to prove, and with which mechanism

**Semantics [src].** Specifications use Hoare-style triples over `Result`, defined in `aeneas/backends/lean/Aeneas/Std/WP.lean`:

- `f x ⦃ r => P r ⦄` is **total correctness**: `f x` returns `ok r` with `P r`; it neither fails nor diverges. Divergence makes it false: `spec_div : spec div p ↔ False`, `WP.lean:106`; notation at `WP.lean:191-192`.
- `f x ⦃ r => P r ⦄div` is **partial correctness**: divergence is allowed. `dspec_div … ↔ True`, `WP.lean:164`; notation at `WP.lean:195-196`.

So a total-correctness proof of a loop simultaneously proves:

1. No panic: no overflow, no out-of-bounds indexing, no failed `unwrap`, etc.
2. Termination.
3. The functional postcondition.

**Mechanisms [src][doc].**

| Translation mode | What Lean sees | What the user proves | Library support |
|---|---|---|---|
| Default (`loop` combinator) | `loop body init`, where `loop` is `partial_fixpoint` | A **measure** that decreases, a loop **invariant** and a **postcondition**, via `loop.spec_decr_nat` (Nat measure) or `loop.spec` (any well-founded relation) | `WP.lean:757-797` |
| `-loops-to-rec` | Recursive `f_loop … partial_fixpoint` | A spec theorem about `f_loop` proved by recursion on itself. Termination is shown by `termination_by <measure>` and `decreasing_by scalar_decr_tac`; the invariant is the theorem's precondition | `aeneas-overview.md:143-175`; tutorial `aeneas/tests/lean/Tutorial/Solutions.lean:229-255` |

**Automation.** The `step` tactic applies `@[step]` lemmas for library functions and for the user's own loop lemmas. `scalar_tac` discharges bounded-integer arithmetic goals.

**Experiment [test].** I wrote the following proofs against the generated code. All three type-check with Lean 4.36.0-rc2 and the Aeneas library (exit 0, no `sorry`). Full text is in Appendix C; files are in `experiments/proofs/`.

1. **`find` (L04, early return), default mode**, using `loop.spec_decr_nat`:
   - measure `s.length - i`;
   - invariant "`i ≤ len` and no earlier element equals `x`";
   - postcondition "`some j` ⇒ `s[j] = x`; `none` ⇒ `x` is absent".

```lean
theorem find_loop_spec (s : Slice Std.U32) (x : Std.U32) (i : Std.Usize)
    (hi : i.val ≤ s.length) (hprev : ∀ k, k < i.val → s.val[k]! ≠ x) :
    find_loop s x i ⦃ r => FindPost s x r ⦄ := by
  unfold find_loop
  apply loop.spec_decr_nat
    (measure := fun (i : Std.Usize) => s.length - i.val)
    (inv := fun (i : Std.Usize) => i.val ≤ s.length ∧ ∀ k, k < i.val → s.val[k]! ≠ x)
  · rintro i' ⟨hi', hprev'⟩
    unfold find_loop.body
    ...   -- 15 lines: `step` through indexing and `+ 1`, `scalar_tac` for bounds
  · exact ⟨hi, hprev⟩
```

2. **The same `find`, `-loops-to-rec` mode.** The recursive spec ends with:

```lean
termination_by s.length - i.val
decreasing_by scalar_decr_tac
```

3. **`for _ in 0..n { c += 1 }` (L30), default mode.**
   - The loop state contains the `Range` iterator.
   - The invariant relates `it.start` to the counter: `it.end = n ∧ it.start ≤ n ∧ c = it.start`.
   - The measure is `n - it.start`.
   - The library's `@[step]` lemma for `IteratorRange.next` is applied automatically by `step`.
   - Proving absence of overflow of `c + 1` falls out of the invariant.

**Effort.** Each loop proof is 20–35 lines. Each took a few rounds of fixing against Lean's error messages; only the final, successful check logs are kept, in `experiments/proofs/*.log`. The `step` and `scalar_tac` tactics discharged all the bounded-integer arithmetic, including absence of overflow and index bounds.

**Caveats from the sources.**

- **[doc]** The combinator mode's proof infrastructure "is less mature — fewer lemmas, less automation" (`tips-and-tricks.md:21-22`).
- **[doc]** `partial_fixpoint_induct` should not be used, because it needs an admissibility proof that is `sorry`'d (`aeneas/documentation/skills/aeneas-lean-core.instructions.md:262-265`; per the docs survey, not re-checked).
- **[issue]** #1367 (open): recursion *through* a loop body does not pass Lean's `partial_fixpoint` monotonicity check.

## 2.6 Iterator model coverage in the Lean library

Read from source **[src]**: `aeneas/backends/lean/Aeneas/Std/Core/Iter.lean` and related files. Rows marked [test] were also confirmed by the experiments.

| Item | Modelled? |
|---|---|
| `Iterator` trait | Fields: `next`, `step_by`, `enumerate`, `take` only (`Iter.lean:92-105`) |
| `next` for `Range`, `RangeInclusive`, `StepBy`, `Enumerate`, `Take`, `TakeWhile`, `Zip`, `Rev`, slice `Iter`/`IterMut`/`ChunksExact`, `vec::IntoIter` | Yes (docs survey; [test] for Range, StepBy, Enumerate, Zip, Rev, Iter, IterMut, vec IntoIter) |
| `Map` | Type only, no `next`; axioms [test] |
| `Filter`, `fold`, `sum`, `any`, `count`, `chain`, `skip`, `windows`, `copied`/`cloned`, `peekable` | No; axioms [test for filter/fold/sum/any/count] |
| `array::IntoIter` (by value) | No; axioms [test] |
| `IntoIterator for &Vec` | No; axiom [test] |
| `FromIterator for Vec` (`collect`) | Yes [test L20] |
| `String`/`&str` | `Str` is a slice of bytes; `String` is opaque; `Chars::next` is opaque (`StringIter.lean:9-22`) |

**The axiom risk.** An `axiom` declared for a std function (e.g. `Iter::fold`) has no defining equations, so nothing can be proved about code that calls it. Users must either:

- write a model and its spec themselves (`-split-files` generates a `FunsExternal_Template.lean`: `aeneas/README.md:153-174` [doc]), or
- rewrite the code as an explicit loop.

\newpage

# 3. Other features

## 3.1 Traits and generics

**Supported [test]:**

- traits with bounds (T01);
- associated types (T02);
- const generics (T03);
- default methods and supertraits (T05);
- generic structs with `impl` blocks (T06);
- `impl Trait` in argument and return position (T07);
- static trait methods `T::zero()` (T08).

All are clean.

**`dyn Trait` [test].** In T04, `fn area_dyn(x: &dyn Shape) -> u32 { x.area() }`:

- the *type* is translated as `Dyn (fun _dyn => Shape _dyn)`;
- the unsizing coercion at the call site works;
- the method call fails with "Function pointers are not supported yet", and the body becomes `sorry`.

`Box<dyn Trait>` (T04b) fails the same way.

- [src] Charon lowers dyn method calls to vtable function-pointer calls, and Aeneas rejects calls through function pointers (`aeneas/src/interp/InterpStatements.ml:1230,1249,1270`).
- [issue] #1500 (open, 2026-10-30) and PR #729 (preliminary `dyn` support, merged 2026-02-05).
- `dyn Trait` in struct fields is also rejected: "Dynamic trait types are not supported yet" (`aeneas/src/symbolic/SymbolicToPureTypes.ml:132,192,542,677` [src]; [issue] #1343).

**Generic associated types: not supported [src][issue].**

- "Generic Associated Types are not supported yet" (`SymbolicToPureTypes.ml:131`).
- Charon's `--preset=aeneas` cannot lift GATs: "GATs cannot work with the `--lift-associated-types` option (implied by `--preset=aeneas`)" (`charon/src/transform/normalize/expand_associated_types.rs:1145-1150`).
- Tracked in [issue] #1112 (open).

**Mutually recursive traits [test-suite].** These are a declared known failure: `aeneas/tests/src/mutually-recursive-traits.rs:1` (`//@ known-failure`); [issue] #176 (open).

**Higher-ranked bounds [test-suite].**

- Preliminary support landed in PR #1158 (merged 2026-07-02).
- Three known-failure tests remain: `aeneas/tests/src/higher_ranked_implied_bounds_{borrow,regions,types}.rs`. Their message: "Unimplemented … lifetime constraint relating a higher-ranked lifetime to a free lifetime".

## 3.2 Closures and function pointers

**Closures [test].** Local closures (C01), closures passed to generic `F: Fn` (C02), `FnOnce` with a moved capture (C05), and returning `impl Fn` (T07) are all clean.

- A closure becomes a Lean structure holding its captured state, plus `FnOnce`/`FnMut`/`Fn` instances (visible in L18, Appendix A).

**`FnMut` that mutates a capture (C03): fails in Lean [test].**

```text
error: Type mismatch … has type closure → Unit → Result closure
       but is expected to have type closure → Unit → Result Unit
```

This matches open [issue] #960 ("Wrong binder for call to `FnMut` closure returning unit") and the commented-out FnMut tests in `aeneas/tests/src/closures.rs:13-28` [test-suite].

**Function pointers (C04): not supported [test][src].**

- `fn(u32) -> u32` parameters give "Arrow types are not supported yet" (`SymbolicToPureTypes.ml:190,422,540,675`).
- Calls through them give "Function pointers are not supported yet" (`InterpStatements.ml:1230`).
- Turning a `fn` item into a pointer gives "Invalid input for unop: fn_def_to_fn_ptr".

## 3.3 Borrows and lifetimes

**Supported [test]:**

- `&mut &mut T` parameters (B01);
- a struct with a lifetime holding `&'a mut [u32]`, with methods (B02);
- returning a borrow chosen between two inputs, and borrowing a field (B03);
- two-phase borrow `v.push(v.len())` (B05);
- recursion returning `&mut` into a `Box` list (B06).

**Generic instantiated with `&mut` (B04) [doc][test].** The README lists this case as unsupported (`aeneas/README.md:132-135`).

- In the experiment, Aeneas reports **no error** (exit 0) but emits ill-typed Lean: `let (i, id_back) ← id 0#u32`, where `id` returns a plain `T`.
- Lean reports "expected a product type". The documentation and experiment agree that the case is unsupported, but the failure is **silent at the Aeneas level**.

**Other limits [src]:**

- "ADTs containing nested mutable borrows are not supported yet" (`SymbolicToPureTypes.ml:295-300`).
- "Mutable references for lifetime 'static are not supported" (`aeneas/src/interp/InterpExpressions.ml:459-461`).
- Open [issue]s: #929, #1186, #1503 (nested borrows).

## 3.4 Arrays, slices, Vec, strings, Option/Result, Box

**Arrays and slices (D01, D02) [test].**

- Clean: indexing, update, `len`, 2-D arrays, sub-slicing `&s[1..3]`, `split_at_mut`, `swap`.

**`Vec` (D03) [test].**

- `new`, `push`, index, index-assign and `extend_from_slice` are modelled.
- `insert` is modelled.
- `pop` and `remove` are **axioms**.
- `vec![1,2,3]` introduces an axiom for `MaybeUninit`.
- `vec![0; n]` translates.

**Strings (D04) [test].** `str::len`, `&str == &str`, `String::new` and `String::push_str` are all **axioms**; only string literals translate.

**`Option`/`Result` (D05) [test].**

- `match`, `unwrap_or` and `map` with a closure are modelled.
- `?` on `Result` is modelled.
- `?` on `Option` uses the axiom `Option…Try.branch`.

**`Box` (D06) [test].** Box is erased (`Box<T>` becomes `T`); recursive enums through `Box` work.

**Derives (D08) [test].**

- `#[derive(Clone, PartialEq, Default)]` translate.
- `Debug` introduces a `Formatter` axiom.

## 3.5 Integer arithmetic, casts, overflow, panics

**Arithmetic (A01, A02, A04) [test].**

- Every `+ - * / %` and unary minus on `u8…u128`/`i16`/`i32` is translated to a checked operation in `Result`: overflow or division by zero give `fail`.
- `wrapping_*`, `checked_add`, `overflowing_add`, `saturating_sub`, bit ops, shifts and `rotate_left` are modelled.
- `count_ones` is an axiom.

**Casts (A03) [test].**

- Integer↔integer `as`, `bool as u8` and `u32::from(u16)` are modelled.
- `u8::try_from(u32)` is an axiom.
- **`u8 as char` crashes Aeneas** with an internal "Unreachable" error. Aeneas then writes a *partial* Lean file whose `c4` body is cut off, so **the whole file fails to parse** (`unexpected token '/--'`).

**Panics (A05) [test].** `panic!`, `assert!`, `assert_eq!`, `unwrap`, `expect` and `unreachable!` all become `fail` in the `Result` monad.

- [src] Unwinding is not supported: "unwinding is not supported by Aeneas" (`InterpStatements.ml:794`).

**Floats (A06) [test].** "Invalid inputs for binop", and the body becomes `sorry`.

- [src] `aeneas/src/symbolic/SynthesizeSymbolic.ml:79` "Float are not supported in Aeneas yet".
- [issue] #828 (open).

## 3.6 Recursion and mutual recursion

**Supported [test].** Simple recursion (`fact`, list length; R01) and mutual recursion between functions (`even`/`odd`; R02).

- Both are emitted as `partial_fixpoint` definitions, inside a `mutual … end` block for R02. Termination is left to the user, exactly as for loops.

**Mutually inductive types (R03) [test].** Mutual recursion over a mutually inductive pair of types (`Tree`/`Forest`) translates, but **Lean rejects it**:

```text
Could not prove 'r03_mutual_inductive.tsum' to be monotone in its recursive calls:
  Cannot eliminate recursive call `fsum f` enclosed in  let ⟨v, f⟩ := t ...
```

**Related [src].**

- `-decreases-clauses` (generate `termination_by` templates) is refused for Lean together with mutually recursive groups (`aeneas/src/Main.ml:604-616`).
- "Mutually recursive globals are not supported" (`aeneas/src/Translate.ml`).

## 3.7 Unsafe code

The README states that unsafe code is out of scope for now, pending separation-logic work (`aeneas/README.md:120-124, 137-139`) **[doc]**.

| Case | Result |
|---|---|
| U01 `&mut x as *mut u32` + deref | "Unsupported operation: &raw mut (*x)"; body becomes `sorry` [test]. Deref also rejected in source: `InterpPaths.ml:191-197` "Aeneas does not yet support dereferencing raw pointers." [src] |
| U02 pointer casts, pointer→`usize` | "Unsupported operation: &raw const (*x)" [test] |
| U03 `union` | "unions are not supported" (`TypesAnalysis.ml:544`), then internal errors [test][src]; [issue] #1199 open |
| U04 `mem::transmute` | **No error**: emitted as `axiom core.intrinsics.transmute` [test]. Source also has "Unsupported: transmute" for transmute *casts* (`SymbolicToPureExpressions.ml:625`) [src] |
| U05 `unsafe fn` + `ptr::read` | `ptr::read` becomes an axiom over `ConstRawPtr`; the caller fails on `&raw const` [test] |
| U06 `static`/`const` | Reading `static K` and `const C` works; `static mut` gives "Unexpected access to a global" [test] |

**[issue]** PR #1428 "Heap model for raw pointers" (merged 2026-10-13) added library groundwork. The experiments show that the translator does not yet use it for ordinary raw-pointer code.

## 3.8 Interior mutability and reference counting

**`Cell`, `RefCell`, `Rc`, `Arc` (I01–I04) [test].** All translate with the types and every method as **axioms**, for example:

- `axiom core.cell.Cell (T : Type _) : Type`
- `axiom core.cell.Cell.set … : Cell T → T → Result Unit`

**Consequence.** In I01, `fn c(x: &Cell<u32>) { x.set(x.get() + 1) }` becomes:

```lean
def c (x : core.cell.Cell Std.U32) : Result Unit := do
  let i ← core.cell.Cell.get core.marker.CopyU32 x
  let i1 ← i + 1#u32
  core.cell.Cell.set x i1
```

The write is not threaded back to the caller, so the model cannot express the mutation.

- [issue] #1525 (open, 2026-11-03) reports the same problem for `Cell::set` and `Atomic*::fetch_add` and calls it potentially unsound.
- [doc] `aeneas/docs/user/src/README.md:15` lists interior mutability as "ongoing work".

## 3.9 Concurrency and async

**Threads and synchronisation (X01–X03) [test].**

- `Mutex::lock`, `MutexGuard`, `AtomicU32::fetch_add` and `thread::spawn`/`join` are all axioms.
- `thread::spawn(..).join()` additionally produces Lean that does not type-check ("contains universe level metavariables").

**async (X04) [test].**

- Charon warns "Coroutine types are not supported yet" and "Coroutines are not supported" (`charon/src/bin/charon-driver/translate/translate_types.rs:332`, `translate_bodies.rs:1487,2174` [src]).
- Aeneas then reports "Found type error in the output of charon" plus internal errors.
- [issue] #1265 (open) depends on charon#609 (open). charon PR #1602 "Translate coroutines" was opened 2026-11-09.

**Scope [doc].** README: concurrency will be addressed by the separation-logic work (`aeneas/README.md:137-139`).

## 3.10 FFI, extern functions, opaque declarations

**What happens [test].**

- `extern "C" { fn ext_add(..) }` (F01) becomes `axiom ext_add : Std.U32 → Std.U32 → Result Std.U32`. The caller translates normally.
- A module marked opaque with Charon's `--opaque 'crate::hidden'` (F02) becomes an axiom in the same way.
- `#[no_mangle] pub extern "C" fn` (F03) translates normally.

**How opacity works [src].**

- Charon has four opacity levels: Transparent, Foreign, Opaque, Invisible (`charon/src/ast/meta.rs`, per the source survey).
- Options and attributes control them: `--include`, `--opaque`, `--exclude`, and `#[charon::opaque]`/`#[aeneas::opaque]`/`#[verify::opaque]` (`charon/src/bin/charon-driver/translate/translate_meta.rs:692-711`).
- `--preset=aeneas` (`charon/src/options.rs:566-581`) does **not** extract bodies of external crates. Every std function that lacks a hand-written Lean model therefore becomes an axiom.

**Recommended practice [doc].** Use `-split-files` so that missing definitions go to a template the user fills in (`aeneas/README.md:153-174`).

## 3.11 Standard-library coverage (summary)

**Size [src].**

- The Lean library `aeneas/backends/lean/Aeneas/Std/` is about 15.5k lines.
- Per the docs survey, `aeneas/src/extract/ExtractBuiltinLean.ml` lists 517 Rust-to-Lean mappings: 39 types, 345 functions, 47 trait declarations, 86 trait impls. The largest groups are `core::iter` (78), `core::slice` (55) and `core::num` (54).

**Coverage.**

- **Modelled:** scalars and their methods; arrays; slices; much of `Vec`; `Option`/`Result` basics; ranges and several iterators; `Box` (erased); `cmp`/`min`/`max`; `mem::swap`/`replace`; `From`/`Into`; `to_le_bytes`/`from_le_bytes` [test S02, S03, S07].
- **Not modelled (axioms) [test]:**
  - `Cell`, `RefCell`, `Rc`, `Arc`;
  - `HashMap`, `BTreeMap`, `VecDeque`;
  - `String` operations;
  - `format!`/`println!`;
  - `slice::sort`, `mem::take`;
  - `Vec::pop`/`remove`;
  - most iterator adaptors.

## 3.12 Drops

**`Drop` impls (S08) [test].** A struct with a `Drop` impl translates, but `drop` is never called: the function body is just `ok 1#u32`.

- [src] By default drops are no-ops: `drop_as_no_op = ref true`, `aeneas/src/Config.ml:625-627`. The option `-eval-drops` exists.
- [issue] #1410 (open): "user `Drop` impls never run, so a destructor's effect is silently lost".

\newpage

# 4. Known open issues that affect users

All issues below were verified against the GitHub API dump in `notes/gh/`: state as of 2026-11-10, and the date is the creation date. The full classified list (237 open Aeneas issues, 69 open Charon issues) is in `notes/github-issues.md`.

## 4.1 Loops

| # | Title (abridged) | Opened |
|---|---|---|
| 822 | Support for early returns inside loops that can be replaced with breaks | 2026-03-04 |
| 1501 | "Breaks to outer loops are not supported yet" for two returning loops in sequence (reproduced here, L29) | 2026-10-30 |
| 1221 | Lean backend silently emits a bodyless axiom for a function with a `return` nested two loops deep | 2026-07-18 |
| 1367 | No `partial_fixpoint` monotonicity lemma for `loop`; recursion through a loop fails in Lean | 2026-09-26 |
| 1395 | Fieldless enum variant written into an array is translated as `()` when a later loop borrows the array (miscompilation) | 2026-10-03 |
| 1513 | "Unimplemented" when a loop reads an array after an indexed write | 2026-10-30 |
| 1519 | Early break causes problems when joining abstractions | 2026-11-02 |
| 1162 | Internal error on a loop inside a trait method | 2026-06-24 |
| charon#955 | More stable looping control flow | 2025-12-22 |

**Related history.** PR #958 "Support nested loop exits" was closed **unmerged** on 2026-06-22.

**Recently fixed.**

- #964 and #1206: "Could not match the contexts" with a conditional `break` in a `for` loop. Closed 2026-07-15 and 2026-09-16.
- charon#1051: control-flow misreconstruction with loops. Closed 2026-06-26 via charon PR #1306.

## 4.2 Iterators

| # | Title (abridged) | Opened | State |
|---|---|---|---|
| 1043 | Iterator trait impl generates `map :=` and `collect :=` fields (reproduced here, L13/L21) | 2026-05-20 | open |
| 1132 | `IteratorRange::next` has no reasoning lemma (a lemma now exists, see §2.4c) | 2026-06-17 | open |
| 1192 | Second loop zipping two mutable slice iterators | 2026-07-07 | open |
| 1193 | Trait method passed to `.map` is "Unimplemented" | 2026-07-08 | open |
| 1222 | `Iterator::try_for_each` aborts with "Internal error" | 2026-07-20 | open |
| 1053 | `Iterator::zip` test fails with `RErased` | 2026-05-22 | open |
| 1435 | Uncaught exception `CFailure` | 2026-10-13 | open |
| 1437 | Missing backwards function with iterators | 2026-10-14 | open |
| 1152, 1153 | Ill-typed `Iterator.next` / `IntoIter.zip` output | 2026-06-22 | open |
| 1511 | No model for `Iterator::chain` | 2026-10-30 | open |
| 1514 | `Step` lacks `forward_overflowing`/`backward_overflowing` (warning seen here) | 2026-10-30 | open |
| 1408, 1287 | Signed `Step::backward_checked` modelled wrongly (`rev()` over signed ranges) | 2026-10-09, 2026-08-24 | open |
| 1534 | `RangeFrom::get_mut` model writes back to the wrong part of the slice | 2026-11-03 | open |

**Open PRs adding iterator models:**

- #1510 (`map`, `chain`)
- #1542 (`fold`, `rfold`)
- #1544 (`any`)
- #1552 (`count`)
- #1057 (`DoubleEndedIterator`/`Rev`)
- #1314 (iterators over arrays)
- #1133 and #1468 (range step specs)

**Recently merged iterator work:**

- #698 basic iterators (2026-01-14)
- #786 `enumerate`/`take` (2026-02-19)
- #1143 iterator transformers (2026-06-20)
- #1172 std models and theorems (2026-06-30)
- #1131 `Iterator` extraction fix (2026-09-03)
- #1386 `next_back` (2026-10-28)
- #1373 `TakeWhile` (2026-11-07)
- #1486 `IntoIterator for &mut [T; N]` (2026-11-09)

## 4.3 Other

| # | Topic | State |
|---|---|---|
| 960 | `FnMut` closure call returning unit is mistyped (reproduced, C03) | open |
| 1046 | `FnOnce` `call_once` return type wrong | open |
| 1119 | Internal error: returning a closure that borrows its environment | open |
| 1500 | Method calls through `&dyn Trait` (reproduced, T04) | open |
| 1343 | `Box<dyn Iterator>` field rejected | open |
| 1112 | GATs | open |
| 176, 1264 | Mutually recursive traits / functions + trait impls | open |
| 1227 | Full higher-ranked trait bounds | open |
| 929, 1186, 1503 | Nested borrows | open |
| 1199 | Unions | open |
| 1396 | Inline asm | open |
| 1525 | `Cell`/atomics modelled as pure, which can be unsound (cf. I01) | open |
| 1410 | `Drop` impls never run (reproduced, S08) | open |
| 828 | Floats (reproduced, A06) | open |
| 1265 / charon#609 | `async` (reproduced, X04) | open |
| charon#142 | Charon's tracking issue for unsupported Rust features | open |
| charon#1040 | Missing lifetime information for closure captures | open |

**Stale Charon documentation [doc vs issue].** `charon/docs/limitations.md:13-15` lists charon#583 as a "known unsoundness", but charon#583 was closed on 2025-12-08.

\newpage

# 5. How to reproduce

Everything lives in the work directory (the root of this repository); nothing outside it was modified. Toolchains are isolated:

- `RUSTUP_HOME` and `CARGO_HOME` under `tools/`;
- `ELAN_HOME` under `tools/elan`;
- `OPAMROOT` under `tools/opam`;
- `gh` 2.63.2 and `pandoc` 3.5 binaries under `tools/`.

All of them are set by `env.sh` (copy it from `env.sh.example`).

```bash
cd aeneas-limitations                                     # repo root
git clone https://github.com/AeneasVerif/aeneas.git        # tested at 0d21a9e
git clone https://github.com/AeneasVerif/charon.git        # main b9da4e3 (recorded only)
git -C charon worktree add ../charon-pinned $(tail -1 aeneas/charon-pin)   # 454586e
cp env.sh.example env.sh && source env.sh
# OCaml deps (isolated opam root; pins required by Charon's nix files):
opam init --bare -y --disable-sandboxing --no-setup
opam switch create default ocaml-base-compiler.5.3.0
opam pin add -n visitors.20260520 "git+https://gitlab.inria.fr/fpottier/visitors.git#20260520"
opam pin add -n ppx_deriving_hash.0.1.5 "git+https://github.com/sim642/ppx_deriving_hash.git#0.1.5"
opam install dune calendar domainslib easy_logging logs menhir ocamlgraph ppx_deriving \
  ppx_deriving_yojson ppxlib progress unionFind visitors yojson zarith ppx_deriving_hash re
(cd charon-pinned && make build-charon-rust build-charon-ml)
(cd aeneas && ln -sfn ../charon-pinned charon && make)
(cd aeneas/backends/lean && lake exe cache get && lake build)
# Experiments
experiments/run_charon.sh                 # Charon on every experiments/cases/*.rs
experiments/run_aeneas.sh                 # Aeneas (Lean) + `lake env lean` type-check
OUT=out-loops-to-rec AENEAS_EXTRA=-loops-to-rec experiments/run_aeneas.sh L01_while ...
python3 -I experiments/classify.py experiments/out > experiments/results.md
# Proofs: concatenate generated Lean + proof, then check
cd experiments/leancheck && lake env lean ../proofs/FindLoopComb.lean
```

**Commands used per case.**

```bash
charon rustc --preset=aeneas --dest-file out/C/C.llbc -- --crate-type=lib --edition=2021 cases/C.rs
aeneas -backend lean out/C/C.llbc -dest out/C/lean
```

F02 additionally passes `--opaque 'crate::hidden'` to Charon.

**Problems met while building, and how they were solved.**

1. The system opam index was stale: `visitors 20251114` and `ppx_deriving_hash 0.1.3` are too old, and the latter causes `Error: This kind of expression is not allowed as right-hand side of "let rec"` in charon-ml. I did not update the global index; instead I used a private `OPAMROOT` plus git pins.
2. A Lean toolchain was accidentally downloaded into the global elan home (`.elan` in the home directory) once, when `env.sh` had not been sourced. It was removed immediately with `elan toolchain uninstall`.

**Output layout.**

| Path | Contents |
|---|---|
| `experiments/cases/*.rs` | 100 Rust programs |
| `experiments/out/<case>/` | `.llbc`, `charon.log`, `aeneas.log`, generated `lean/*.lean`, `lean.log`, exit codes |
| `experiments/out-loops-to-rec/` | Same, for L-cases with `-loops-to-rec` |
| `experiments/results*.md`, `summary*.txt` | Tables and per-case error/axiom summaries |
| `experiments/proofs/` | Proofs and their check logs |
| `notes/` | Survey notes (docs and tests, source, GitHub). The raw GitHub API JSON (`notes/gh/`) is not included; `scripts/fetchall.sh` refetches it |
| `logs/` | Build logs |

**What was not checked.**

- I did not run Aeneas's own test suite (`make test`). Its contents and known-failure markers were read from the test files instead.
- I did not test the F\*, Coq/Rocq and HOL4 backends.
- Charon upstream `main` was not run, because Aeneas pins an older Charon.
- GitHub data covers all open issues and those closed in the last 12 months. Comments were read for 17 threads only, and 200 of about 400 tags were fetched (all releases were fetched).
- The behaviour of the `-decreases-clauses` mode for Lean was not exercised.
- How exactly Aeneas decides, in default mode, between the `loop` combinator and recursion was read from source but not characterised experimentally beyond L23.
