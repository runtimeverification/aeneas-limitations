# Aeneas / Charon: documentation and test-suite survey

Scope: documentation statements, test-suite conventions, known failures, loop and iterator tests, and the Lean standard-library models.
Checkouts: `aeneas/` = Aeneas @ 0d21a9ec, `charon-pinned/` = Charon @ 454586e9. All paths are relative to the repository root.
Rule followed: every claim cites file:line. Claims I could not verify are marked **[UNVERIFIED]**.

---

## 1. Documentation statements

### 1.1 Aeneas README.md (`aeneas/README.md`)

- Scope is safe Rust: "Aeneas currently functionalizes a subset of safe Rust", with separation-logic work planned for unsafe and concurrent code (README.md:120-124).
- Limitations of the safe subset (README.md:126-135):
  - **loops**: "`return` inside *nested* loops, or `break`/`continue` to *outer* loops are not supported yet (e.g., `'a : loop { loop { break 'a; } }`)". The README calls this a technical limitation, not a fundamental one (README.md:128-131).
  - **mutable references and generic types**: you cannot instantiate a generic type, including `Self` through a trait impl, with a mutable reference, e.g. `id(&mut x)` where `fn id<T>(x: T) -> T` (README.md:132-135).
- Waiting on the separation-logic work: **unsafe code** and **concurrency** (README.md:137-139).
- Backends: F*, Coq, HOL4, Lean. Lean and HOL4 are "most mature", with "support for partial functions and extrinsic proofs of termination" (README.md:144-151). The README points to `./backends/lean/Base/Diverge/Elab.lean` and `./backends/lean/Base/Progress/Progress.lean` (README.md:149-150). **These paths are stale:** `backends/lean/Base` does not exist in this checkout (verified with `ls`).
- External or std definitions: with `-split-files`, Aeneas generates `FunsExternal_Template.lean` and similar files listing the missing definitions, and the user writes the models by hand (README.md:153-165). Models can be upstreamed with the attributes `rust_type` / `rust_const` / `rust_fun` / `rust_trait` / `rust_trait_impl` plus `make extract-lean-std`, which regenerates `src/extract/ExtractBuiltinLean.ml` (README.md:167-174).
- Charon must be run with `charon cargo --preset=aeneas` (README.md:82-87).

### 1.2 `aeneas/docs/user/` (mdBook; older material)

- `docs/user/src/README.md:5-16`, under "What it cannot do?":
  - "loops: no nested loops for now" (line 11). This contradicts the test suite: nested loops are translated (see §2.6, `loops-nested.rs`, Iterators.lean `*_loop0_loop0`). **Treat it as outdated.**
  - "no functions pointers/closures: ongoing work" (line 12). Also outdated: closures are translated (tests/lean/Closures.lean). Function-pointer *types* are still unsupported according to `tests/src/fn_ptr_generic.rs:4`.
  - "limited type parametricity: it is not possible for now to instantiate a type parameter with a type containing a borrow" (line 13). This matches README.md:132.
  - "no nested borrows in function signatures: ongoing work" (line 14).
  - "interior mutability: ongoing work … ghost states" (line 15).
  - "no concurrent execution: long-term effort" (line 16).
- `docs/user/src/lean/tactics/step.md` contains only the heading `# Step` (2 lines).

### 1.3 `aeneas/CLAUDE.md`

An index only. Skill files live in `documentation/skills/` (CLAUDE.md:5-22), and the key docs are listed at CLAUDE.md:24-31. It makes no capability claims.

### 1.4 `aeneas/documentation/*.md`: loops, termination and proof mechanisms

**Translation model**
- `aeneas-overview.md:135-139`: "Rust loops (`loop`, `while`, `for`) are translated to auxiliary `_loop` functions … Always write a separate theorem for each `_loop` function."
- Pattern A, recursive `_loop` with `partial_fixpoint` (aeneas-overview.md:143-175). The example `zero_loop` ends with `partial_fixpoint` (line 154). The proof uses `unfold`/`split`/`step` plus `termination_by x.length - i.val` and `decreasing_by scalar_decr_tac` (lines 160-174). In other words, the definition is a `partial_fixpoint`, and termination is proved *extrinsically*, inside the spec proof, by well-founded recursion on the theorem.
- Pattern B, the `loop` fixed-point combinator (aeneas-overview.md:177-191): `def loop (body : α → Result (ControlFlow α β)) (x : α) : Result β`, "typically common with `for` loops which use iterators". You reason about it with `loop.spec_decr_nat` (a Nat measure, an invariant and a postcondition) or `loop.spec` (any well-founded measure).
- The `Result` monad has `ok`, `fail`, and `div` for divergence (aeneas-overview.md:111-119).
- Glossary: `ControlFlow.cont` / `ControlFlow.done` (glossary.md:15). The `loop` combinator, reasoned about with `loop.spec` / `loop.spec_decr_nat` (glossary.md:17).
- Tactics reference: "Loops are translated to a fixed-point operator `loop`" (tactics-reference.md:49-51).
- proof-strategies.md:102-154 restates Patterns A and B. Workflow: identify the invariant, choose a Nat measure, apply `loop.spec_decr_nat`, then prove that the body preserves the invariant and decreases the measure on `cont` (lines 150-154).

**Two loop modes (important)**
- tips-and-tricks.md:13-26 and skills/aeneas-lean-core.instructions.md:411-417:
  - `-loops-to-rec` is recommended. It translates loops to recursive Lean functions with `termination_by` / `decreasing_by`, and proofs use `unfold` + `step`.
  - The fixed-point combinator is the **default**. "The proof infrastructure for this mode is less mature — fewer lemmas, less automation, and less battle-tested."
  - "We are in the process of switching the default translation style to use the fixed-point combinator … Until it matures, **use `-loops-to-rec`** for any project where you need to write proofs."
- Source confirmation: `-loops-to-rec` means "Always extract loops to recursive functions" (src/Main.ml:230-232). `-loops-no-rec` means "Never attempt to extract loops to recursive functions" (src/Main.ml:238-240), and the two flags are mutually exclusive (Main.ml:499-501). The default mode extracts a loop to recursion only when the heuristic `loop_satisfies_recursive_criteria` holds (src/pure/PureMicroPassesLoops.ml:2571-2581), otherwise it uses `loop`. Exactly when that heuristic fires is **[UNVERIFIED]** beyond the conditions listed at lines 2572-2576 (`has_termination_measure && all_inputs_are_id && …`).

**Proof recipes and banned tactics**
- skills/aeneas-lean-core.instructions.md:228-277, "Recursive loop proofs: `unfold` + `step`, never `partial_fixpoint_induct`". It shows an example over `core.ops.range.Range U8` with `termination_by iter.end.val - iter.start.val` and `decreasing_by scalar_decr_tac`. Why not `partial_fixpoint_induct`: it needs an explicit motive and "an `admissible` proof that is typically sorry'd (giving only partial correctness)" (lines 262-265).
  - The same example rewrites with `core.iter.range.IteratorRange.next_U8_def` (line 242). **That lemma name does not exist in `backends/lean`** (grep finds nothing), so the doc is stale or approximate.
- skills/proof-patterns.instructions.md:11-108 gives the canonical loop template: `spec_gen` (arbitrary start plus invariant), then `spec`. Key points:
  - "Never use `partial_fixpoint_induct`" (line 100).
  - The termination pattern is `termination_by iter.end.val - iter.start.val` with `decreasing_by agrind` (lines 104-105).
  - Nested loops: the outer `spec_gen` uses the inner loop's `@[step]` spec (lines 106-108).
- skills/aeneas-tactics-quickref.instructions.md:24-29 is a decision tree: for a loop fixed point, use `loop.spec_decr_nat` or `loop.spec`; for a recursive `_loop`, use `unfold` + `by_cases` + `step` + `termination_by`. `partial_fixpoint_induct` is banned (line 130).
- skills/aeneas-lean-core.instructions.md:60-63: hand-written models must not use `partial`; use `partial_fixpoint` when the function lives in `Result`.
- tips-and-tricks.md:45-60: don't unfold std definitions, including "iterator types". Look for lemmas instead.
- The word `divergent` never appears in the documentation as a mechanism. The only hit is unrelated (`Aeneas/Tactic/Step/StepStar.lean:554`).
- **Not documented in documentation/**: partial-correctness triples `⦃ ⦄div` and the `dspec_induction` tactic (see §3.2). They are used in `tests/lean/Tutorial/Solutions.lean:451-470`.

**Other claims in the docs**
- Data mapping (aeneas-overview.md:213-240, 426-440): `[T;N]` becomes `Array T n`, `Vec<T>` becomes `Vec T`, `Box<T>` becomes `T` (erased), `&mut T` becomes value plus backward function, traits become structures/type classes, and panics/overflow become `fail`.
- emit-json.md:43-50: the JSON manifest has a loop `pos` such as `[0, 1]` for "the second loop nested inside it". This implies nested loops are supported.
- skills/aeneas-compiler-dev.instructions.md:130 contains `[%craise] … "Unsupported: closures with captures"`. This is only an *example* of the error-macro syntax, not a statement about real support.

### 1.5 Aeneas `tests/README.md`

- Per-file options use `//@` comments at the top of single files and the `aeneas-test-options` file for crates (tests/README.md:42-56):
  - `[coq,lean] skip`
  - `[fstar] known-failure`
  - `charon-args=…`, `aeneas-args=…`, `subdir=…`
- `known-failure` tests record Aeneas errors in `test-file.out`, both for "cases that produce an error so we don't forget" and for "cases that _should_ error" (lines 58-70). Charon errors are out of scope, and "there is no option for tests that translate successfully but fail to be verified by the backend" (lines 72-75).
- HOL4 outputs are not regenerated (line 27).

### 1.6 Charon docs (`charon-pinned/`)

- README.md:59-64: Charon is "beta software … poorly documented, has a number of open bugs".
- docs/limitations.md:
  - Known unsoundness: issue #583 (line 13-15).
  - Unsupported features are tracked in issue #142 (line 17-19). **The list itself is not in the repo.**
  - Missing info: lifetimes of captured closure variables (#1040), precise HRTB lifetimes (#1143), and lifetimes inside bodies (lines 21-27).
  - By design: no distinction between `loop`, `while` and `for` (lines 31-33), and no trait solving on the output (lines 34-35).
- docs/faq.md:30-39: by default only the trait methods that are used get translated ("this way `Iterator` only has `next` until another of its methods is used"). `--translate-all-methods` turns this off. faq.md:3-27 covers opaque foreign items, `--extract-opaque-bodies` and `--start-from`.
- docs/what_charon_translates.md:5-12 defines the opacity levels: Transparent / Foreign / Opaque / Invisible. Items outside the current crate are Foreign by default, which means signature only, except structs with all-public fields and enums (line 10). Name-matcher limitations are at line 98.
- docs/transformations.md:35-55: `--remove-associated-types` (lifting) has limitations: "GATs cannot be transformed this way" and some recursive traits cannot be transformed. The Aeneas preset turns on `lift_associated_types` for `*` (charon/src/options.rs:566-567). The test `associated_types/issue-1068-gat-with-assoc-ty.out` says: "GATs cannot work with the `--lift-associated-types` option (implied by `--preset=aeneas`)".
- The Aeneas preset flags are at charon/src/options.rs:566-582: treat_box_as_builtin, ops/index to function calls, reconstruct fallible ops/asserts/matches, hide marker traits and allocator, and others.
- options.rs:44-50: `--no-code-duplication` output "consumers which expect `if`/`loop`-shaped bodies (e.g. aeneas) may not support".
- docs/usage.md:47-49: LLBC is ULLBC with control flow restructured into loops and ifs. docs/reconstructed-information/control-flow.md:7-15: loops are recovered from CFG back-edges, so the structure "may look different from the original Rust code".

---

## 2. Test suites

### 2.1 How Aeneas marks tests (`aeneas/tests/test_runner/Input.ml`)

- Actions are `Normal | Skip | KnownFailure` (Input.ml:4).
- Leading `//@` lines are parsed (Input.ml:37-105, 130-140). Syntax:
  - `[backend,...]` selects backends; `[!backend]` means all backends except those listed (Input.ml:51-66).
  - Recognized keywords: `skip`, `known-failure`, `no-check-output`, `charon-args=`, `aeneas-args=`, `subdir=`.
- Crates read the same keywords from `aeneas-test-options` (Input.ml:127, 141-145).
- run_test.ml:
  - Normal tests run with `-abort-on-error`, so any Aeneas error fails the test (run_test.ml:83-89).
  - KnownFailure tests must fail, and their output goes to the `.out` file (run_test.ml:109-120).
- Charon errors happen before Aeneas runs and cannot be recorded as Aeneas known failures (tests/README.md:72-74).
- Lean verification: `tests/lean/script.sh` adds every `*.lean` in `tests/lean` as a `lean_lib` default target (script.sh:1-15, lakefile.lean). So every generated Lean file is type-checked by `make verify`.
- `#[verify::test]` on a Rust function makes Aeneas emit `#guard (f).reducesTo ()` in Lean (src/extract/Extract.ml:3711-3717, 3771-3784). This *executes* the model. Example: tests/lean/IterAdapters.lean:82-83. 18 files in tests/src use it (grep count).

### 2.2 The `[!lean] skip` convention

Almost every single-file test starts with `//@ [!lean] skip`, i.e. it runs only for the Lean backend. This is a scoping choice, not a failure marker. Tests that run for other backends too: arrays.rs, bitwise.rs, blanket_impl.rs, constants.rs, demo.rs, issue-1138, issue-194, issue-270-loop-list.rs, mini_tree.rs, no_nested_borrows.rs, paper.rs, polonius_list.rs, rename_attribute.rs, switch_test.rs, traits.rs, and vec.rs (skips coq, fstar and borrow-check).

### 2.3 Every Aeneas known-failure and skip-for-a-reason (complete list)

There are 7 known-failure markers in `tests/src` (grep over all `//@` lines). The `.out` files record the errors.

| Test (file:line) | Backend | Recorded error (`.out`) | Nature |
|---|---|---|---|
| `tests/src/borrow-check-negative.rs:3` | borrow-check | "Can not apply a projection to the ⊥ value" / "Can't end abstraction … non-endable" (`borrow-check-negative.borrow-check.out:1-40`) | intended negative test |
| `tests/src/loops-borrow-check-negative.rs:3` | borrow-check | "Can't end abstraction 8 as it is set as non-endable" on `loop_a_b` (`loops-borrow-check-negative.borrow-check.out:1-8`) | intended negative test (lines 6 vs 19 of the .rs: "This succeeds" / "This fails") |
| `tests/src/higher_ranked_implied_bounds_borrow.rs:2` | lean | "Unimplemented: found an occurrence of a lifetime constraint relating a higher-ranked lifetime to a free lifetime." (llbc/TypesAnalysis.ml:990) (`.lean.out:2-5`) | **real limitation**: HRTB implied bounds |
| `tests/src/higher_ranked_implied_bounds_regions.rs:2` | lean | same as above (`.lean.out:2-5`) | real limitation |
| `tests/src/higher_ranked_implied_bounds_types.rs:2` | lean | same as above (`.lean.out:2-5`) | real limitation |
| `tests/src/mutually-recursive-traits.rs:1` | lean | "Found an associated type in a trait declaration; … this can fail with mutually-recursive traits as well as GATs. Aeneas cannot handle such types today" (symbolic/SymbolicToPure.ml:224) (`.lean.out:2-6`) | **real limitation**: mutually recursive traits and GATs |
| `tests/src/raw_pointers.rs:1` | lean | "Aeneas does not yet support dereferencing raw pointers." (interp/InterpPaths.ml:197) (`raw_pointers.lean.out:2-4`) | **real limitation**: raw pointer deref |

There are no `no-check-output` tests in tests/src. Commented-out code in the tests that documents unsupported features:

- `tests/src/arrays.rs:101-106, 112-117`: "Unsupported by Aeneas for now": `&s[i][j]` on `&[&[T]]` and `s[i][j] = 0` on `&mut [&mut [u32]]`, i.e. slices of borrows.
- `tests/src/closures.rs:13-19`: FnMut closure `call_fn_mut` is commented out ("TODO: monomorphisation in Charon"). Lines 21-28 comment out a closure with a generic param ("typing issue", Charon issue #989).
- `tests/src/fn_ptr_generic.rs:4-5`: "function pointer types (`fn(u8) -> u8`) are not supported by Aeneas; we use function items instead."
- `tests/src/nested-borrows.rs:228-250, 252+`: "TODO: not supported! We overwrite a borrow in a returned value." Lines 55 and 112: "TODO: fix in Charon".
- `tests/src/hashmap.rs:7-17`: an old "Current limitations" header (recursion instead of loops, no closures, no `Option<&T>`). It is likely outdated given the current tests **[UNVERIFIED as to current status]**.
- `tests/src/iterators.rs:2`: "TODO: rev, take". Lines 72-96 comment out generic `const N` array/slice `for` loops, and lines 98-102 comment out `chunks_exact(8).take(32)`. Some of these patterns are now covered by `array-iter-mut.rs` and `vec-iter.rs`.
- `tests/src/iter_adapters.rs:395, 401`: "TODO: Turn this into a `#[verify::test]` when `Option::copied` is modelled."
- `tests/src/closures.rs:50-57` comments out `map_fn_pointer_shared`.

### 2.4 Missing models visible in generated Lean (`axiom`s)

When a used std item has no Lean model, the generated test files emit `axiom`s. Every `^axiom` in tests/lean/*.lean:

- `Iterator::map` default and `Map::next`: Issue1043IteratorMethods.lean:31, 44; Closures.lean:31; Issue1207ClosureSignatureRegions.lean:32, 45.
- `Option<&T>::copied`: IterAdapters.lean:31.
- `IntoIterator for &Vec<T>` (`SharedAVec…into_iter`): VecIter.lean:32 (the `for x in &v` case); LoopSharedBorrowProj.lean:32.
- `Option` `Try::branch` / `FromResidual` (the `?` operator on Option): NestedSharedBorrows.lean:32, 42.
- `core::mem::drop`, `Option::as_mut`: RustBorrowCheckIssues.lean:31, 38.
- `core::slice::first`: MutBorrowInSharedBorrow.lean:31.
- `core::ptr::null`: BuiltinAuto.lean:31.
- `MaybeUninit`: FnPtrGeneric.lean:31.
- `PartialEq::ne` for bool and Box: Derive.lean:31, 38.
- The rest are user-opaque items by design: DropBug.lean:28, OpaqueMutRegion.lean:28/41, LoopSharedBorrowProj.lean:45-55, Issue765HrtbConditionalNegate.lean:141.

### 2.5 Topics of all Aeneas test inputs (119 `.rs` files plus 3 crates)

Classified from the file headers. Crates: `avl/` (469 lines), `bst/` (174 lines), and `tutorial/` (261 lines, run with `-loops-to-rec`, per tutorial/aeneas-test-options:3).

- **Loops (while/loop)**: loops.rs, loops-rec.rs (`-loops-to-rec`), loops-nested.rs, loops-nested-rec.rs (`-loops-to-rec`), loops-issues.rs, loops-sequences.rs, loops-adts.rs, loops-borrow-check-negative.rs, loop-shared-borrow-proj.rs, loop_shared_loan_in_join.rs, issue-134-loop-shared-borrows.rs, issue-270-loop-list.rs, issue-789-loop-ctx-match.rs, issue-1140-global-loop.rs (loops in `static`/`const`), issue-1206-loop-break.rs, issue-1436-loop-generics.rs / -rec.rs, joins.rs, join-duplicate.rs.
- **Iterators / for loops**: iterators.rs, iterators-array.rs, iterators-scalar.rs, iter_adapters.rs, vec-iter.rs, array-iter-mut.rs, step_by.rs, chunks_exact.rs, step_overflowing.rs, range.rs (slice range indexing), string-chars.rs, issue-1043-iterator-methods.rs, issue-1192.rs (borrow-check only, zip with iter_mut).
- **Closures / fn**: closures.rs, fn_ptr_generic.rs, issue-804-closure-return-ref.rs, issue-1207-closure-signature-regions.rs, issue-1141-impl-fn-ref.rs, dyn.rs (dyn_closure).
- **Traits**: traits.rs, defaulted_method.rs, blanket_impl.rs, mutually-recursive-traits.rs, higher_ranked_trait_bounds.rs, higher_ranked_implied_bounds_*.rs, issue-765-hrtb…, issue-801-hrtb.rs, from_to.rs, into.rs, derive.rs, default.rs, const-shadow.rs, constants-lean.rs, multi-target.rs, rename_attribute.rs.
- **dyn**: dyn.rs. It only builds `Box<dyn Trait>`, producing `Dyn.mk` (Dyn.lean:59-62). It does not test calling methods through a dyn.
- **Borrows**: borrow.rs, adt-borrows.rs, nested-borrows.rs, nested-shared-borrows.rs, no_nested_borrows.rs, mut-borrow-in-shared-borrow.rs, list-borrows.rs, opaque-mut-region.rs, polonius_list.rs, rust-borrow-check-issues.rs, borrow-check-negative.rs, drop_bug.rs, issue-1260…, issue-807…, as_mut.rs, deref.rs.
- **Arrays / slices / Vec**: arrays.rs, arrays_defs.rs, array_slice_index.rs, array-constructors.rs, slices.rs, vec.rs, vec-methods.rs, issue-1138, issue-803, dynamic_size.rs.
- **Scalars**: scalars.rs, bitwise.rs, overflowing-ops.rs, cast_signed.rs, uint-div-ops.rs, bool-cmp.rs, order.rs.
- **Std models**: options.rs, results.rs, builtin.rs, builtin-auto.rs, print.rs.
- **Misc / regressions**: adt.rs, constants.rs, static.rs, names.rs, lean-keywords*.rs, switch_test.rs, discriminant.rs, drop.rs, assert-cfg.rs, target-features.rs, generic_unit_output.rs, issue-440/767/815/1044/1250/charon-1172, mini_tree.rs, paper.rs, demo.rs, curve25519.rs, hashmap.rs.
- **Unsafe**: raw_pointers.rs (known-failure).

### 2.6 Loop and iterator tests in detail

**Loop shapes exercised** (tests/src):
- `while` with scalar counter: loops.rs:4-24
- `while` over `Vec` with `v.len()`: loops.rs:72
- `while let List::Cons` returning `&mut T`: loops.rs:97-120
- `loop` with `break`/`return`: loops.rs:122-150, 512-523
- pairs of lists with merged lifetimes: loops.rs:193-350
- loops ignoring a borrowed input: loops.rs:352-375
- nested `while` with an inner empty loop and `return` from the outer loop: loops.rs:546-563
- nested `while` loops: loops-nested.rs:3-30
- an NTT layer: loops-nested.rs:59-80
- nested `for … (0..N).step_by(8)` / `for r in 0..rows`: loops-nested.rs:142-143
- sequences of loops: loops-sequences.rs:29, 39
- loops with arrays of borrows and `Option<&mut T>` results: loops-adts.rs:8-60
- `for i in 0..xs.len()` with `break`: issue-1206-loop-break.rs:4-13
- generic loops over `I: IntoIterator`, `while let Some(x) = it.next()`: issue-1436-loop-generics.rs:11-56

**No test uses labeled `break 'a` / `continue 'a`** (grep over tests/src returns nothing), which is consistent with README.md:128. The source has errors for these cases: "Breaks to outer loops are not supported yet", "Continue to outer loops are not supported yet", "Returns inside of nested loops are not supported yet" (src/PrePasses.ml:636-648; also :592), and src/interp/InterpLoopsFixedPoint.ml:173-174, 288-289. Charon *does* support labeled breaks: `charon/tests/ui/control-flow/loops.rs:44, 71, 283-363`.

**Iterator coverage exercised** (tests/src):
- Ranges:
  - `for _ in 0..32usize` (iterators.rs:5)
  - `(0..n).step_by(2)` (iterators.rs:9)
  - `0u8..5u8`, u16, u32, u64, usize, and empty ranges (iter_adapters.rs:72-125)
  - `RangeInclusive` (iter_adapters.rs:329-350)
  - `Step::forward_overflowing` (step_overflowing.rs)
- Slices:
  - `s.iter()` / `s.iter_mut()` driven manually in `while` loops (iterators.rs:12-45)
  - `chunks_exact(16)` with an inner `key.iter()` loop (iterators.rs:48-51)
  - `a.iter().zip(b.iter())` (iterators.rs:68-69)
  - `chunks_exact` behaviour (chunks_exact.rs)
  - `iter().step_by(k)` (step_by.rs)
  - `iter().rev()` (iter_adapters.rs:414-428)
  - `iter().take_while(closure)` (iter_adapters.rs:358-392)
  - `iter().enumerate()`, `.take()`, `.enumerate().step_by()` (iter_adapters.rs:13-180)
- Arrays:
  - `for i in [..]` by value (iterators-array.rs:3-6)
  - `for elem in x` with `x: &mut [i32; N]` (array-iter-mut.rs:10-45)
- Vec:
  - `for x in v` (IntoIter), `for x in &v`, `iter().step_by/enumerate/take`, `into_iter().step_by/enumerate/take` (vec-iter.rs:3-78)
- Collect / map:
  - `iter.enumerate().map(closure).collect()` (issue-1043-iterator-methods.rs:9-10). The model has axioms for map (§2.4).
  - `s.chars().collect::<Vec<char>>()` (string-chars.rs:3-5). The model is opaque (§3.3).
  - `x.into_iter().map(u8_id)` (closures.rs:47-49)
- **Not exercised anywhere in tests/src** (grep): `filter`, `fold`, `sum`, `skip`, `chain`, `flat_map`, `windows`, `chunks` (non-exact), `any`/`all`/`position`/`find`/`count`, `HashMap`, `RefCell`/`Cell`/`Rc`.

**Representative Lean output.** Default mode, `while` loop → `loop` combinator (tests/lean/Loops.lean:53-78):
```lean
@[rust_loop_body]
def sum_loop.body (max : Std.U32) (i : Std.U32) (s : Std.U32) :
  Result (ControlFlow (Std.U32 × Std.U32) Std.U32) := do
  if i < max
  then let s1 ← s + i
       let i1 ← i + 1#u32
       ok (cont (i1, s1))
  else ok (done s)
@[rust_loop]
def sum_loop (max : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  loop (fun (i1, s1) => sum_loop.body max i1 s1) (i, s)
def sum (max : Std.U32) : Result Std.U32 := do
  let s ← sum_loop max 0#u32 0#u32
  s * 2#u32
```
With `-loops-to-rec` (tests/lean/LoopsRec.lean:44-51):
```lean
@[rust_loop]
def sum_loop (max : Std.U32) (i : Std.U32) (s : Std.U32) : Result Std.U32 := do
  if i < max
  then let s1 ← s + i
       let i1 ← i + 1#u32
       sum_loop max i1 s1
  else ok s
partial_fixpoint
```
Iterator `for` loop (tests/lean/VecIter.lean:152-192, `for (i,x) in v.iter().enumerate()`): the iterator state is threaded explicitly through `core.iter.adapters.enumerate.IteratorEnumerate.next (core.iter.traits.iterator.IteratorSliceIter Std.U32) iter`, followed by `match o with | none => ok (done acc) | some p => … ok (cont (iter1, acc1))`. The setup is `core.slice.Slice.iter s` then `core.iter.traits.iterator.Iterator.enumerate.trait_default …`.

Nested loops produce `f_loop0_loop0` functions, e.g. tests/lean/Iterators.lean:326-375 (`slice_chunks_exact_iter_loop0_loop0`) and tests/lean/Loops.lean:1151-1190 (`decode_loop0_loop0`).

**Hand-written loop proofs in tests/lean:**
- Tutorial/Solutions.lean:229-255: `zero_loop_spec` uses `termination_by x.length - i.val` and `decreasing_by scalar_decr_tac`. More loop specs are at :339-361 and :378-439.
- Tutorial/Solutions.lean:451-470: `pseudo_random_spec` uses the *partial-correctness* triple `⦃…⦄div` and `dspec_induction pseudo_random_loop`, with the comment "if we proceed by unfolding `pseudo_random_loop`, then the proof will be non-terminating".
- Hashmap/Properties.lean:668-806 (`move_elements_loop_spec`, `termination_by slots.val.length - i.val`).
- ArrayIterMut/Properties.lean:15-54: `for elem in x` over `&mut [T;N]` in the default `loop` mode, proved with the library lemma `core.slice.iter.IterMut.loop_updateBody.spec`.

### 2.7 How Charon marks tests (`charon-pinned/charon/tests/ui.rs`)

- Leading `//@` comments are parsed (ui.rs:42-61):
  - `ignore` / `skip` mark the trial as ignored.
  - `known-panic` / `known-failure` / `known-ub` mean stderr is the expected output.
  - `no-check-output` is also recognized.
  - `//@[rev]` applies a comment to one revision, and `revisions=` declares the revisions (ui.rs:47-53, 82-91).
- 325 entries in tests/ui at the top level. Subdirectories: associated_types, consts, control-flow, dyn, filtering, minirust, monomorphization, multi-target, partial-monomorphization, regressions, simple, traits, unsupported.

**All Charon known-failure / known-panic / ignore tests**, grouped by topic (first error line from the `.out` file):

- **dyn / trait objects**
  - `dyn/box-dyn-fnonce.rs:1`: "trying to access the allocator field from Box, but it is being treated as a builtin" (calling `Box<dyn FnOnce()>` with `--include=alloc::boxed::_`).
  - `dyn/dyn-sized.rs:1`: panic at translate_trait_objects.rs:251 for `dyn Sized`.
  - `dyn/dyn-supertrait-projection-vtable.rs:1`: panic at translate_trait_objects.rs:47 (supertrait projection in vtable).
  - `dyn/dyn-trait-non-wf.rs:1`: rustc E0277. The test input itself is ill-formed.
- **Associated types / GATs**
  - `associated_types/issue-1068-gat-with-assoc-ty.rs:1`: "GATs cannot work with the `--lift-associated-types` option (implied by `--preset=aeneas`)".
  - `associated_types/issue-1519-recursive-lifting.rs:1`: "Stack depth limit exceeded while expanding associated types".
- **Traits / trait solving**
  - `traits/impossible-recursive-trait-proof.rs:1`: stack overflow.
  - `traits/incorrect-cyclic-wf.rs:1`: stack overflow.
  - `traits/recursive-projection-normalization-overflow.rs:1`: rustc E0275 then "Hax panicked".
  - `simple/coinductive-perfect-derive.rs:1`: rustc E0275 overflow on `List<u32>: Clone`.
  - `monomorphization/supertrait-with-local-trait-proofs.rs:3`: `[mono]` revision only; "Type error after translation".
- **Coroutines / async**: `simple/coroutine.rs:1`: "Coroutine types are not supported yet" / "Coroutines are not supported".
- **Filtering / CLI misuse** (expected errors): `filtering/match_on_opaque_enum.rs:1` ("reading the discriminant of an opaque enum"), `filtering/start_from_errors.rs:1`, `filtering/start_from_on_module.rs:1`, `rename_attribute_failure.rs:1`, and `simple/conditional-method.rs:1` (uses `#[charon::error]` on purpose to capture stderr).
- **Ill-formed input**: `unsupported/unbound-lifetime.rs:1` (rustc E0261).
- **MiniRust harness**: `minirust/known-panic.rs:2`, `minirust/known-ub.rs:2` (expected).
- **`ignore` (auxiliary files, not real tests)**: consts/ptr-cast-in-bodies-aux.rs, foreign-constant-aux.rs, global-ctors-dep-aux.rs, issue-114-opaque-bodies-aux.rs, macro-spans-aux.rs, macro-spans-included.rs, macro-spans-pathmod.rs, regressions/extern-static-no-initializer-aux.rs, regressions/issue-1393-foreign-static-aux.rs. (`ignore-warnings` is a different, unrelated flag.)
- Other "not supported" text in non-failing Charon outputs: `vtable: unknown // vtables of trait alias impls are not supported` (simple/trait-alias.out, several lines).
- `unsupported/issue-79-bound-regions.rs` has no failure marker; it is an ordinary passing test (`slice.iter().next()`).
- **Closures, unions, unsafe and raw pointers have no known-failure in Charon.** For example, closures.rs, unions.rs, unsafe.rs, raw-boxes.rs and ptr-offset.rs all pass.

Charon loop and iterator UI tests: control-flow/loops.rs (labeled break/continue, interleaved loops, lines 32-363), control-flow/loop-break.rs, empty-loop.rs, ambiguous-loop-exit.rs, issue-1051-*.rs, iterator.rs (`--translate-all-methods`, array into_iter/iter/chunks), simple/range-iter.rs, associated_types/iter-empty.rs, monomorphization/opaque-iterator-closure.rs.

---

## 3. Lean standard library models (`aeneas/backends/lean/Aeneas/Std`)

### 3.1 Files and coverage

There are about 15.5k lines under Aeneas/Std (wc). Main files:
- **Result / monad / errors / loop**: Primitives.lean.
  - `Result α := ITree RustEffect α` (Primitives.lean:68). `fail` and `div` are at :79, :84.
  - `ControlFlow` (:447-450). `loop` is defined with `partial_fixpoint` (:452-457).
  - The Error kinds are assertionFailure, integerOverflow, divisionByZero, arrayOutOfBounds, maximumSizeExceeded, panic, undef (:39-46).
- **WP / specs**: WP.lean.
  - `spec` (total correctness: `spec div p ↔ False`) at :56-57, :106.
  - `dspec` (partial correctness: `dspec div p ↔ True`) at :59-60, :164. The notation `e ⦃ x => p ⦄div` is at :195-196, 283-303.
  - `dspec_admissible` at :65-67, :74-76.
  - `loop.spec` (well-founded measure) at :757-782; `loop.spec_decr_nat` at :784-797.
- **Tactic `dspec_induction`** for partial-fixpoint definitions: Aeneas/Tactic/Step/DspecInduction.lean:11-19, syntax at :164.
- **Scalars**: Scalar/*.lean, the biggest being Core.lean (1295 lines) and CoreConvertNum.lean (844). Covered operations:
  - checked, wrapping, overflowing and saturating ops
  - div_ceil, pow, rotate, leading_zeros, casts, bitwise, Display/Debug fmt
- **Slices**: Slice.lean (1093 lines), SliceDef.lean. Arrays: Array/Array.lean, ArraySlice.lean, FromFn.lean, Core.lean.
- **Vec**: Alloc/Vec.lean (722 lines) and Alloc/Vec/IntoIter.lean (93 lines). Modelled: new, len, is_empty, push, insert, index/index_mut, from_elem, with_capacity, extend_from_slice, deref, resize, clone, eq. The builtins list has **no** pop, remove, truncate, clear, swap_remove or append.
- **Box**: Alloc/Boxed.lean (erased; deref/clone/as_mut/eq).
- **Option**: Core/CoreOption.lean and Core/Core.lean. **Result**: Core/CoreResult.lean.
- **Ranges**: Range.lean, RangeIter.lean (580 lines of specs).
- **Iterators**: Core/Iter.lean (1003 lines), SliceIter.lean (550), StringIter.lean (35), Alloc/Vec/IntoIter.lean.
- **String/str**:
  - `Str := Slice U8` (StringDef.lean:8)
  - `alloc::string::String` is mapped *opaquely* to Lean `String` (Alloc/String.lean:8)
  - `Chars` is opaque, and `Chars::next` / `collect` are `opaque` with `-- TODO:` (StringIter.lean:9-22)
- **Other**: Cmp, Convert, Fmt, Hash (trait only), Ops, Borrow, Default, Discriminant, Marker, Pin, Ptr (Alignment only), Atomic (AtomicBool/AtomicU32 types only), RawPtr.lean (a struct and scalar cast only), Std/Io.lean (`_print`).
- **Absent** (grep for RefCell, core.cell, Rc, Arc, HashMap, BTreeMap and VecDeque in Aeneas/Std finds only an internal Lean `Std.HashMap` in Spec.lean:92): **no models for Cell/RefCell/Rc/Arc/HashMap/BTreeMap/VecDeque.**
- Aeneas/Data/Range/{DivRange,MulRange,SRRange} are *spec-side* Lean range helpers for structural for-loops in specifications (DivRange/Basic.lean:9-17). They are not Rust models.

### 3.2 Iterator model specifics

- `Iterator` trait model (Core/Iter.lean:92-105) has the fields `next`, `step_by`, `enumerate` and `take` only. The comments say "TODO: adding more fields like rev leads to a circularity" (:99-104) and "TODO: collect" (:105).
- Adaptor structs:
  - StepBy (:40-44, modelled as `iter` plus `step_by`, "not exactly the way the implementation is defined")
  - Enumerate (:46-49), Take (:51-54), Rev (:56-58), TakeWhile (:60-64)
  - Zip (:70-74, "omit[s]" the `index`/`len` fields)
  - Map (:1000-1003, a *type only*: no `next` model, hence the axioms in §2.4)
- Default methods with models:
  - `step_by` (panics if 0, :76-80), `enumerate`, `take` (:82-129)
  - `collect` through `FromIterator` (:210-217), `rev`, `take_while`, `zip`
  - Their spec lemmas are at :756-895.
- `next` models exist for:
  - StepBy (:143-156), Enumerate (:541), Take (:574), Zip (:638)
  - Range (:602), RangeInclusive (:685), TakeWhile (:871), Rev (:934)
  - Range/RangeInclusive `next_back` (:947, :981)
  - slice `Iter`/`IterMut`/`ChunksExact` (SliceIter.lean:46, 69, 197)
  - Iter::next_back (SliceIter.lean:94)
  - vec `IntoIter` (Alloc/Vec/IntoIter.lean:14)
- `Step` trait for all 12 integer types: steps_between, forward/backward checked/overflowing (Core/Iter.lean:380-533).
- Traits declared without implementations for standard adaptors: `Sum`, `Product` (:172-178), `Extend` (:219), `ExactSizeIterator` (:230).
- `FromIterator<Vec<T>>::from_iter` is implemented with a helper `iterToList` that is a `partial_fixpoint` (Alloc/Vec/IntoIter.lean:52-73). `IntoIter::map` is modelled (:84-91), but `Map::next` is not.
- IntoIterator impls: `&[T;N]` (SliceIter.lean:137-143), `&mut [T;N]` (:158), `&[T]` (:171-177), `Vec<T>` (Alloc/Vec/IntoIter.lean:37-42), and the blanket `I: Iterator` (Core/Iter.lean:193-208). **There is no `&Vec<T>` impl**, hence the axiom at VecIter.lean:32.
- Proof support for iterator loops:
  - RangeIter.lean:19-556 has `next` specs for Range over UScalar and generic `Step`, `skipN`/StepBy specs, Enumerate specs, Take over ChunksExact, and RangeInclusive specs.
  - SliceIter.lean:399-433 has into_iter and `IterMut.next` specs, and :457-547 has `IterMut.loop_updateBody.spec`. That is a ready-made **`loop.spec_decr_nat`-based lemma for `for x in it { *x = g(*x) }` loops**, with the tactic `iter_mut_update_body`.

### 3.3 `src/extract/ExtractBuiltinLean.ml` (auto-generated, 1844 lines)

- Header: "THIS FILE WAS AUTOMATICALLY GENERATED FROM LEAN" (line 1). The lists are lean_builtin_types (:4), lean_builtin_consts (:196, empty), lean_builtin_funs (:198), lean_builtin_trait_decls (:1373) and lean_builtin_trait_impls (:1574). Each entry carries a `(* file: …, line: … *)` pointer back to its Lean model.
- Counts from parsing the `mk_*` entries (517 in total, matching 517 `(* file:` comments):
  - **types: 39**
  - **consts: 0**
  - **funs: 345**: core::iter 78, core::slice 55, core::num 54, core::fmt 42, alloc::vec 23, core::cmp 23, core::array 21, core::ops 9, core::option 9, core::result 8, alloc::boxed 5, core::convert 4, core::str 3, alloc::slice 2, core::mem 2, others 1 each
  - **trait decls: 47**: core::ops 12, core::iter 10, core::convert 6, core::cmp 4, core::marker 4, fmt 3, hash 2, …
  - **trait impls: 86**: core::iter 27, fmt 18, convert 10, ops 10, cmp 6, clone 5, slice 5, …
- Iterator-related entries (substring iter/range/chunks/step): 184 lines in total. They cover:
  - types IntoIter, Enumerate, Map, Rev, StepBy, Take, TakeWhile, Zip, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, ChunksExact, slice Iter, IterMut, Chars
  - funs: `Iterator::{collect, enumerate, rev, step_by, take, take_while, zip}`, the `next` impls listed above, 60 `Step<int>` functions, `slice::{iter, iter_mut, chunks_exact}`, `ChunksExact::remainder`, `str::chars`, `Chars::{next, collect}`, `Vec::from_iter`/`into_iter`, `IntoIter::{next, map}`, and RangeInclusive `new`/`is_empty`
  - trait decls Iterator, IntoIterator, FromIterator, Extend, DoubleEndedIterator, ExactSizeIterator, Sum, Product, Step, TrustedRandomAccessNoCoerce
  - **Absent**: `map` next, `filter`, `fold`, `sum`, `skip`, `chain`, `any`, `all`, `count`, `windows`, `chunks`, `cloned`, `copied`, `peekable`, `flat_map`.
- Option funs: expect, is_none, is_some, is_some_and, map, ok_or, take, unwrap, unwrap_or. Result funs: expect, is_ok, map, map_err, unwrap, unwrap_or, Try::branch, FromResidual (so `?` works on Result, but there is no Option `Try`, which matches the axiom in §2.4).
- Hand-written builtins that are *not* in the generated file live in `src/extract/ExtractBuiltin.ml`:
  - `checked_{add,sub,mul,div,rem}` (:573-581)
  - From<INT,bool> and From<INT,INT>
  - leading_zeros (:605), to/from_le/be_bytes (:611-633)
  - `Box::new` erased (:635)
  - Clone for bool and ints, Ord for ints
  - saturating_add/sub, wrapping_add/sub/mul, overflowing_add/sub/mul/div, rotate_left/right (:852-879)
  - All of it is concatenated with `lean_builtin_funs` (:880). Builtin types come in via `mk_lean_only lean_builtin_types` (:173).
