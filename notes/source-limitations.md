# Aeneas / Charon: source-code survey of user-reachable limitations

Checkouts:
- Aeneas `0d21a9ec7bbbb110110cacc14464a47fd71c8705`: `aeneas/` (paths below are relative to `aeneas/src/` unless prefixed)
- Charon `454586e9b770231e66d2062993e632dfc06463e2`: `charon-pinned/` (paths prefixed `charon/src/...`)

Method: grep for unsupported / not supported / not implemented / unimplemented / craise / cassert / internal_error / failwith / TODO / unreachable, then read the code around each hit. Every claim below cites file:line. Anything I inferred without directly confirming it is marked **[UNCERTAIN]**.

---

## 0. Aeneas error machinery: which errors are user-facing, and what happens after one

`Errors.ml` defines these helpers. They are invoked through PPX macros `[%craise]`, `[%cassert]`, and so on, which pass in `__FILE__`/`__LINE__` (Errors.ml:1-8; aeneas-ppx/AeneasPpx.ml:75-95).
- `craise` / `craise_opt_span` (Errors.ml:121-135): records the error with `push_error` and raises `CFailure`. Under `-abort-on-error` (`Config.fail_hard`) it raises `Failure` instead and the run aborts.
- `cassert` (Errors.ml:154-160): raises through `craise` when the condition is false. `classert` is the lazy-message variant (Errors.ml:139-152).
- `craise_recover` / `cassert_recover` (Errors.ml:162-176): when `recover=true` they raise the silent `RFailure`, which the caller catches to fall back to another strategy.
- `sanity_check`, `internal_error` (Errors.ml:178-193) all print the same message, "Internal error, please file an issue". These are bugs rather than documented limitations, but a user can still hit them.
- `warn` (Errors.ml:195-201): only prints. It becomes an error under `-warnings-as-errors`.
- `save_error` (Errors.ml:107-115): records the error and continues.
- Extraction helpers `extract/ExtractErrors.ml:13-50`: `admit_raise` / `admit_string` save the error and print `sorry` (Lean) or `admit` (Coq/F*/HOL4) into the generated code (ExtractErrors.ml:24-28).

Granularity: an error does not stop the run. It degrades the affected item.
1. **Pre-pass failure**: the function body is replaced by `ErrorBody` and the error "Ignoring the body of '<name>' because of previous error" is recorded (PrePasses.ml:2395-2417). `ErrorBody` is not "translatable" (llbc/LlbcAstUtils.ml:24-26), so the function is treated as opaque, which means it is extracted as an `axiom` in Lean (extract/ExtractBase.ml:1449-1450, Translate.ml:852-855).
2. **Symbolic-execution failure**: caught in interp/Interp.ml:676-692 with the warning "Could not translate the body of function ... because of previous error". The function then gets an `Error` symbolic AST, which becomes `EError` (symbolic/SymbolicToPureExpressions.ml:144, 253) and is extracted as `sorry` in Lean (extract/Extract.ml:703-707, 741). So the definition exists but its body is `sorry`.
3. **Failure while translating to pure**: caught in Translate.ml:283-302 with "Could not translate the function ...". The function is dropped (`None`, filtered by `parallel_filter_map`, Translate.ml:477-502). Uses of a dropped function may be printed as `sorry /- ERROR: unknown identifier ... -/` (extract/ExtractBase.ml:471, 481) **[UNCERTAIN that this is the exact path for callers of a dropped fun]**.
4. Failures in type, global, trait-decl or trait-impl translation are handled the same way with warnings (Translate.ml:344-391, 541-590; symbolic/SymbolicToPure.ml:136-150).
5. `-abort-on-error` makes the first error fatal (Main.ml:148-150; Errors.ml:121-128).

In practice most "Unimplemented"/"Not implemented" messages in `interp/InterpAbs.ml`, `InterpMatchCtxs.ml`, `SymbolicToPureAbs.ml` and `SymbolicToPureValues.ml` are internal corner cases of the borrow/abstraction machinery. Their trigger is almost always nested borrows or unusual borrow shapes. They are user-reachable, but they don't map one-to-one to Rust features.

---

## 1. Aeneas: categorized user-reachable limitations

### 1.1 Loops (detail in section 2)
| Limitation | Location | Message |
|---|---|---|
| Labeled `break` to an outer loop | PrePasses.ml:636-638; also :589-592; interp/InterpLoopsFixedPoint.ml:289; interp/InterpLoops.ml:182 | "Breaks to outer loops are not supported yet" / "Breaks to outer loops not supported yet" / "Nested loops are not supported yet" |
| Labeled `continue` to an outer loop | PrePasses.ml:640-642; InterpLoopsFixedPoint.ml:174; InterpLoops.ml:310 | "Continue to outer loops are not supported yet" / "Continues to outer loops not supported yet" / "Nested loops are not supported yet" |
| `return` inside a loop nested in another loop | PrePasses.ml:644-648 | "Returns inside of nested loops are not supported yet" |
| Early `return` inside a loop that also has `break`s, when the code after the loop does not end in return/panic | PrePasses.ml:571-576 | "Early returns inside of loops are not supported yet" |
| Loops with no `break` at all (infinite loops; loops whose only exits are `return`/panic after rewriting) | interp/InterpLoops.ml:409-412 | "(Infinite) loops which do not contain breaks are not supported yet" |
| Fixed point not reached within 2 iterations | interp/InterpLoopsFixedPoint.ml:152-160; Config.ml:176-182 (`loop_fixed_point_max_num_iters = 2`) | "Could not compute a loop fixed point in N iterations" |
| Leftover `Return` in a loop body during symbolic execution | InterpLoops.ml:166-168 | "Unexpected return" |
| Loop joins are not recoverable: the code-duplication fallback used for `if`/`match` joins (Config.ml:504-507; interp/InterpStatements.ml:978-984) is unavailable for loops | InterpLoops.ml:96, 118, 322; InterpLoopsFixedPoint.ml:140; InterpJoin.ml:1077-1078 (`~recoverable:false`) | any join error is fatal for the function |
| Joining abstractions with shared loans whose value contains borrows | interp/InterpReduceCollapse.ml:1372-1376 | "Joining region abstractions containing shared loans whose shared value contains borrows is not supported yet" |
| Join of nested borrows / shared vs. mutable loan | interp/InterpMatchCtxs.ml:412-430, 970, 1082, 1124, 1169, 1242, 1594 | "The join of nested borrows is not supported yet", "Nested borrows are not supported yet.", "Unimplemented" |
| `destructure_shared_loans` (applied at loop breaks) with borrows inside shared values | interp/InterpJoin.ml:1217-1221 | "Not implemented" |

Nested loops are supported when they don't break, continue or return across loop levels. The test `tests/src/loops-nested.rs:3-26` has nested `while` loops, and the checks above only fire for index `i <> 0` or return depth > 1.

### 1.2 Borrows and lifetimes
| Limitation | Location | Message |
|---|---|---|
| ADTs whose definition contains nested mutable borrows (a `&mut` under a `&mut`) | symbolic/SymbolicToPureTypes.ml:295-300; definition of "nested mut": llbc/TypesAnalysis.ml:350-352 | "ADTs containing nested mutable borrows are not supported yet" |
| Nested borrows in values or abstractions (many sites) | interp/InterpAbs.ml:25-422 (≈25 sites), 1421, 1427; interp/InterpBorrows.ml:1996, 2005, 2025, 2132; SymbolicToPureExpressions.ml:1244 | "Nested borrows are not supported yet" (InterpAbs.ml:1421 and 1427 say "... yet in this case") |
| Mutable borrows inside shared borrows (e.g. `&&mut T`) when destructuring abstractions | interp/InterpBorrows.ml:1963-1966 | "Mutable borrows inside shared borrows are not supported yet" |
| Other nested-borrow cases | interp/InterpBorrows.ml:2048, 2061 | "Found a case of unsupported nested borrows" |
| `&'static mut` constants | interp/InterpExpressions.ml:459-461 | "Mutable references for lifetime 'static are not supported" |
| Constants and globals containing `'static mut` borrows | interp/InterpUtils.ml:320-325 | "Unimplemented: constants and globals whose value contains mutable borrows with the 'static lifetime ..." |
| Loan and borrow in the same region abstraction (involves `'static`) | interp/InterpReduceCollapse.ml:779-784 | "Unimplemented: found a loan and its corresponding borrow in the same region abstraction ..." |
| Higher-ranked lifetime related to a free lifetime by an outlives constraint | llbc/TypesAnalysis.ml:987-992 (confirmed by tests/src/higher_ranked_implied_bounds_*.lean.out) | "Unimplemented: found an occurrence of a lifetime constraint relating a higher-ranked lifetime to a free lifetime." |
| Bound regions in some type positions | llbc/TypesAnalysis.ml:736, 799, 820, 939 | "Not handled yet", "Bound regions are not handled yet", "Bound regions are not supported yet" |
| `Body` regions | llbc/TypesAnalysis.ml:240, 477, 739; llbc/TypesUtils.ml:490, 516; InterpBorrowsCore.ml:2264; SymbolicToPureTypes.ml:810 | "unsupported: Body region" |
| Generic check `check_ty_is_supported` | interp/InterpUtils.ml:90-97 | "Unsupported use of borrows found in type" (never fires: `ty_is_supported` always returns `true`, llbc/TypesUtils.ml:279-284) |

Supported forms (confirmed by code paths): two-phase borrows (`BTwoPhaseMut`), shallow and unique-immutable borrows are accepted at InterpStatements.ml:738-745. Reborrows inside loops are handled by `reborrow_ashared_loans_symbolic_borrows`, which runs before and inside loops (InterpLoops.ml:516-536; InterpLoopsFixedPoint.ml:54-56). Returning borrows from functions and loops goes through backward functions and loop output continuations (pure/PureMicroPassesLoops.ml:2782-2830).

### 1.3 Traits, generics, dyn, associated types, const generics
| Limitation | Location | Message |
|---|---|---|
| GATs (an `ItemClause` survives associated-type lifting) | SymbolicToPureTypes.ml:128-131 | "Generic Associated Types are not supported yet" |
| Associated types that were not lifted (mutually recursive traits, GATs) | symbolic/SymbolicToPure.ml:222-230 (warning; see tests/src/mutually-recursive-traits.lean.out) | "Found an associated type in a trait declaration; ... Aeneas cannot handle such types today, and the generated code will likely be incorrect." |
| Associated types with generic arguments | SymbolicToPureTypes.ml:185-187, 535-537, 670-672 | "Unimplemented" |
| `Dyn` trait-ref kind | SymbolicToPureTypes.ml:132 | "Dynamic trait types are not supported yet" |
| `dyn Trait` in type declarations or signature types (`translate_sty`, used for struct fields at SymbolicToPureTypes.ml:250-254) and in backward types | SymbolicToPureTypes.ml:191-192, 541-542, 676-677 | "Dynamic trait types are not supported yet" |
| `dyn` in forward types is supported (`Dyn` wrapper) | SymbolicToPureTypes.ml:423-427; tests/lean/Dyn.lean:59-62 | — |
| Casts to dyn: only `Box<T>`→`Box<dyn>` and `&T`→`&dyn` | interp/InterpExpressions.ml:786-957 | "Higher-ranked vtables are not supported", "Unimplemented use of dynamic traits", "Unsupported cast to a dynamic trait" |
| dyn upcasts other than the identity | InterpExpressions.ml:958-972 | "Unsized cast between dynamic traits: unimplemented case" |
| `dyn Trait` concretization cast | SymbolicToPureExpressions.ml:626-627 | "Unsupported: `dyn Trait` concretization" |
| Calling a method on a `dyn Trait` **[UNCERTAIN, inferred]**: Charon rewrites dyn method calls into calls through vtable function pointers (charon/src/transform/normalize/transform_dyn_trait_calls.rs:1-17, run unconditionally at charon/src/transform/mod.rs:153-155). Aeneas rejects calls through function pointers (FnOpDynamic) | interp/InterpStatements.ml:1249 | "Function pointers are not supported yet" |
| Vtables are filtered out of the output | PrePasses.ml:1535-1540 (`remove_vtables`) | — |
| Unsupported dyn usage at extraction | extract/ExtractTypes.ml:528 | "Unsupported use of dyn traits" |
| Recursive and mutually recursive trait declarations | Translate.ml:1194-1206 (warning; "model will not type-check") | "Recursive trait declarations are not supported; ..." / "Mutually recursive trait declarations are not supported; ..." |
| Mutually recursive trait impls (single recursive impls are OK for Lean) | Translate.ml:1238-1255 | "Mutually recursive trait implementations are not supported; ..." |
| Const generic parameters of non-scalar type | SymbolicToPureTypes.ml:48-53 | "Unsupported constant expression type" |
| Unusual const-generic expression kinds | SymbolicToPureTypes.ml:28-38 | "Unsupported constant expression kind" |
| `-parameterize-trait-types`-style trait types at extraction | extract/ExtractTypes.ml:472-473 | "Unimplemented" (emits `sorry`) |
| Const generics and trait types in HOL4 only | ExtractTypes.ml:432-434, 498, 580, 1179, 1336, 1687, 1691; Extract.ml:2408 | "Constant generics are not supported yet when generating code for HOL4" |
| Unknown trait | SymbolicToPureTypes.ml:133 | "Unknown trait found: ..." |
| Trait method referenced as a function value | SymbolicToPureTypes.ml:421; SymbolicToPureExpressions.ml:1658 | "Unimplemented" |

### 1.4 Closures and function pointers
- Closures are supported. Charon encodes them as a state struct plus impls of `Fn`/`FnMut`/`FnOnce`. Aeneas carries ad-hoc lifetime repairs for these encodings: `fix_closure_lifetimes` (PrePasses.ml:1995-2014) and `fix_closure_signature_regions` (PrePasses.ml:2080-2094), both marked "TODO: remove once the Charon issue is fixed". Tests: tests/src/closures.rs. Some cases there are commented out as TODO (closures.rs:13, 21-23).
- Function pointer types (`fn(..) -> ..`, `TFnPtr`): "Arrow types are not supported yet" (SymbolicToPureTypes.ml:189-190, 422, 539-540, 674-675). The test header says "function pointer types (`fn(u8) -> u8`) are not supported by Aeneas; we use function items instead" (tests/src/fn_ptr_generic.rs:1-4).
- Function items (`TFnDef`) are supported in forward types (SymbolicToPureTypes.ml:397-421), but not in type declarations or signatures (SymbolicToPureTypes.ml:189).
- Calls through function pointers: "Function pointers are not supported yet" (InterpStatements.ml:1230, 1249, 1270).
- Casts from fn item to fn pointer and fn to unsafe fn: "TODO: function casts" (SymbolicToPureExpressions.ml:618-619).
- A function item whose effect is not the default one: "Unimplemented" (SymbolicToPureTypes.ml:410-417; SymbolicToPureExpressions.ml:1645-1655).

### 1.5 Unsafe code, raw pointers, unions, transmute
- Raw pointer dereference: "Aeneas does not yet support dereferencing raw pointers." (interp/InterpPaths.ml:191-197; tests/src/raw_pointers.lean.out). Creating raw pointers (`RawPtr` rvalue) is accepted (InterpStatements.ml:747-752). The raw-pointer type maps to a builtin `TRawPtr` (SymbolicToPureTypes.ml:173-180).
- Raw-pointer casts are supported only between pointers to literal types: "Raw ptr casts are only supported between pointers to literal types; found: ..." (SymbolicToPureExpressions.ml:590-606). HOL4 additionally rejects them: "Raw pointer casts are not implemented for HOL4" (extract/Extract.ml:421).
- Aggregated raw pointers (fat-pointer construction): "Aggregated raw pointers are not supported yet" (InterpExpressions.ml:1375-1376).
- Pointer/address casts: "Unsupported: pointer to address casts", "Unsupported: address to pointer casts" (SymbolicToPureExpressions.ml:614-617).
- Transmute: "Unsupported: transmute" (SymbolicToPureExpressions.ml:625).
- Subtype casts: "Unsupported: subtype casts" (SymbolicToPureExpressions.ml:620).
- Place projections `PtrMetadata` and `Offset`: "Unsupported place projection: pointer metadata" / "... offset" (symbolic/SynthesizeSymbolic.ml:20-23).
- Unions: "unions are not supported" (llbc/TypesAnalysis.ml:544), "Unions are not supported" (InterpExpansion.ml:721; pure/PureMicroPassesGeneral.ml:1715).
- Inline assembly: Charon translates it into an `InlineAsm` statement (charon/src/bin/charon-driver/translate/translate_bodies.rs:1880-1902). Aeneas has no case for it, so it reaches "unsupported statement: ..." (PrePasses.ml:734-736 in `remove_useless_joins`; also InterpStatements.ml:803-804). **[UNCERTAIN which of the two fires first; the pre-pass comes first in apply_passes, PrePasses.ml:2380-2391]**. The same applies to `DropEmptyBox` **[UNCERTAIN whether it appears with --preset=aeneas]**.

### 1.6 Arrays, slices, strings
- `Repeat` (`[x; N]`) must be removed by a pre-pass; otherwise "Repeat should have been removed in a micropass" (InterpStatements.ml:735-737; pre-pass `update_array_default`, PrePasses.ml:154).
- Slice literal extraction: `[%cassert] span is_array "Unimplemented"` (extract/Extract.ml:841).
- String constants are supported only as `str` or `&'static str`: "Unsupported type for string constant" (InterpExpressions.ml:441-444).
- Literal extraction: floats, non-ASCII chars and byte strings print `sorry` with "Float, string, non-ASCII chars and byte string literals are unsupported" (extract/ExtractTypes.ml:102-106). Byte-string literals also fail in pure/PureUtils.ml:263 ("Byte string literals are unsupported").

### 1.7 Arithmetic and casts
- Scalar casts (`CastScalar`) are supported. They are infallible in Lean and fallible otherwise (SymbolicToPureExpressions.ml:581-589).
- Branching (`switch`) on a float: "Float are not supported in Aeneas yet" (symbolic/SynthesizeSymbolic.ml:78-79). Switch on other non-int/bool scrutinees: "Unsupported switch scrutinee" (InterpStatements.ml:988).
- Lean binops: checked (`OPanic`) add/sub/mul/div/rem/shl/shr, bitwise, comparisons, and `wrapping_*` for `OWrap` add/sub/mul/div/shl/shr (Extract.ml:596-671). Anything else goes to `named_binop_name` (Extract.ml:672-690). Some paths print `sorry` with "Unimplemented binary operation: ..." (Extract.ml:629-631) **[UNCERTAIN exactly which binops reach this]**.
- Concrete-mode-only (`-test-units`) gaps: "Unimplemented binary operation", BitXor/BitAnd/BitOr raise `Unimplemented` (InterpExpressions.ml:1113-1129).
- The catch-all for other rvalues is "Unsupported operation: ..." (InterpExpressions.ml:1494-1496).

### 1.8 Recursion
- Mutually recursive globals: "Mutually recursive globals are not supported" (Translate.ml:1163-1164, 1174-1175).
- Mixed declaration groups (for example a type and a function in the same recursive group): "Mixed-recursive declaration groups are not supported" (Translate.ml:1266-1268).
- Lean with `-decreases-clauses` and any mutually recursive function group: "The Lean backend doesn't support the use of decreasing_by/termination_by clauses with mutually recursive definitions" (Main.ml:604-616).
- Recursive and mutually recursive functions are otherwise supported. Lean emits `partial_fixpoint` (section 3).

### 1.9 Statics and constants
- Only shared references to globals: "Can only create shared references to global values" (InterpStatements.ml:820-822). Anything else gives "Unsupported case of rvalue accessing a global" (InterpStatements.ml:812-816). This rules out `&mut STATIC` and `static mut` writes **[inferred from the check]**.
- Globals whose type contains regions other than `&'static T` with region-free `T`: "Unimplemented" (InterpStatements.ml:604, 616, 624).
- Globals are not borrow-checked by default: `-borrow-check-globals` (Main.ml:205-209). Its help text says Charon's LLBC "over-simplification ... prevents us from borrow-checking globals which contain static references".

### 1.10 Drops
- By default `drop(p)` is a no-op and drops are not borrow-checked (Config.ml:625-627, `drop_as_no_op = true`). `-eval-drops` switches this off (Main.ml:255-258). User `Drop` impls are therefore not modelled at drop sites **[inferred from the no-op evaluation at InterpStatements.ml:768-773]**.

### 1.11 Panics and unwinding
- `Panic`, `UnwindTerminate` and `UndefinedBehavior` become a `Panic` node in the symbolic AST (InterpStatements.ml:787-792).
- `UnwindResume`: "unwinding is not supported by Aeneas" (InterpStatements.ml:794).

### 1.12 Other
- Charon must be run with `--preset=aeneas`, otherwise Aeneas fails with "Invalid option detected: the serialized crate was generated by Charon without the `--preset=aeneas` option" (Main.ml:597-602).
- `#[verify::test]` on a function whose signature is not `() -> Result<()>`: "#[verify::test] on function with unsupported signature" (extract/Extract.ml:3803-3808).
- `#[target_feature]` assertions on a non-monadic body (PureMicroPassesGeneral.ml:2885-2891).
- Charon type errors: "Found type error in the output of charon" (SymbolicToPureTypes.ml:193-194).
- Traits in concrete mode (`-test-units`): "Traits are not supported yet in concrete mode" (InterpStatements.ml:1290-1291).

---

## 2. How Aeneas translates loops

### 2.1 Pre-pass `update_loops` (PrePasses.ml:392-680)
- The doc comment states its job: "Check that loops: do not contain early returns; do not continue/break to outer loops" (PrePasses.ml:392-395).
- Rewrites applied to `return` inside a loop:
  - **T1**: if the loop has no `break`, each `return` becomes `break` and a `return` is placed after the loop (PrePasses.ml:538-564).
  - **T2/T3**: if the loop has breaks, the code after the loop must end in `return` (T2) or panic (T3). That code is copied in front of each `break` (PrePasses.ml:565-623). Otherwise the pass fails with "Early returns inside of loops are not supported yet" (:574-576).
- Every `Break i` / `Continue i` with `i ≠ 0` (a jump to an outer loop) is rejected (PrePasses.ml:636-642). A `Return` at depth > 1 is rejected (:644-648).

### 2.2 Symbolic execution and fixed point (interp/InterpLoops.ml, InterpLoopsFixedPoint.ml)
- Entry point: `eval_loop` (InterpLoops.ml:499-550). It first simplifies the context, then calls `reborrow_ashared_loans_symbolic_borrows`, which introduces "fixed" region abstractions for shared values (:507-536).
- `eval_loop_symbolic` (InterpLoops.ml:340-497) proceeds in these steps:
  1. **Fixed point at loop entry**: `compute_loop_entry_fixed_point` (InterpLoopsFixedPoint.ml:44-235). It evaluates the body from the current context, keeps the contexts that reach `continue` (:163-177), joins them with the current context (`join_ctxs`, :193), reduces the result with `InterpReduceCollapse.reduce_ctx` (:204-209), and checks equivalence with the previous context through `match_ctxs ~check_equiv:true` (:130-145, :219). If the contexts are not equivalent it iterates. After `loop_fixed_point_max_num_iters = 2` iterations it fails (Config.ml:176-182; InterpLoopsFixedPoint.ml:149-160). The fixed point introduces region abstractions of kind `Loop loop_id` for the borrows that flow through the loop (InterpLoops.ml:352-361).
  2. **Break context**: `compute_loop_break_context` (InterpLoopsFixedPoint.ml:246-381) evaluates the body from the fixed point and collects the contexts at `break`. The result is `NoBreak` (an error, InterpLoops.ml:409-412), `Single` (no join), or `Multiple`. For `Multiple` the body is evaluated a second time and all break contexts are joined (`loop_join_break_ctxs`, :315-322).
  3. **Matching the entry context to the fixed point**: `eval_loop_symbolic_apply_loop` (InterpLoops.ml:76-122) calls `prepare_match_ctx_with_target` and `match_ctx_with_target` with `~recoverable:false`. This yields the loop's input values and abstractions.
  4. **Synthesizing the body**: `eval_loop_symbolic_synthesize_loop_body` (InterpLoops.ml:132-336) re-evaluates the body from the fixed point. Each `continue` is matched back to the fixed point, which produces `SA.LoopContinue` (:308-331). Each `break` is matched to the break context, which produces `SA.LoopBreak` (:180-306). A `return` is "Unexpected return" (:166-168).
  5. The result is an `SA.Loop { loop_id; input_svalues; input_abs; break_svalues; break_abs; loop_expr; next_expr; ... }` node (InterpLoops.ml:478-495).
- Region abstractions that leave the loop get `ELoop` continuations (InterpLoops.ml:226-250; InterpLoopsFixedPoint.ml:13-41, 345-371). This is how loops that return borrows are expressed: through backward-function continuations, which are outputs of the loop.

### 2.3 Pure stage: loop → function (pure/PureMicroPassesLoops.ml, PureMicroPasses.ml)
- The micro-passes include `filter_loop_useless_inputs_outputs`, `reorder_loop_outputs` and `loops_to_recursive` (PureMicroPasses.ml:126-153), then `decompose_loops` (:377-385), `decompose_loop_bodies` (:413-418) and `loops_to_fixed_points` (:425-437).
- `loops_to_recursive` (PureMicroPassesLoops.ml:2782-2944) marks a loop `to_rec` only when `loop_satisfies_recursive_criteria` holds (:2461-2581). The criteria are: input continuations are the identity, output continuations are calls to input continuations, and there is a **structural termination measure** (`loop_rel_has_termination_measure`, :2430-2459). That last condition means some input decreases structurally, through a sub-term of a recursive data structure, at every `continue`. `-loops-no-rec` disables this (:2576-2581).
- `decompose_loops` (PureMicroPassesLoops.ml:2946-3266) creates an auxiliary declaration per loop, keyed `(def_id, Some loop_id)` (:3016-3020). `to_rec = !Config.loops_to_recursive_functions || loop.to_rec` (:3030):
  - **to_rec**: `continue` becomes a recursive call to the loop function (`update_rec_continue_breaks`, :3090-3096). The result is a genuinely recursive function.
  - **otherwise**: the function body stays a `Loop` node. `loops_to_fixed_points` (:3268-3314) turns it into `LoopOp body inputs`, and `decompose_loop_bodies` (:3633-3703) lifts the body lambda into its own named function. `update_non_rec_continue_breaks` (:96-170) turns continue/break into `ControlFlow.cont`/`.done` **[cont/done names from the Lean library below]**.

### 2.4 Lean output
- `LoopOp` is printed as `loop` (extract/Extract.ml:803-809). The Lean library defines
  `def loop (body : α → Result (ControlFlow α β)) (x : α) : Result β := ... partial_fixpoint` (backends/lean/Aeneas/Std/Primitives.lean:446-457). `Result` has a CCPO and `MonoBind` instance (Primitives.lean:333-345) and a `div` constructor (Primitives.lean:84).
- Recursive functions, including `to_rec` loop functions, are emitted as `def ... partial_fixpoint`. `fun_decl_kind_to_post_qualif` returns `partial_fixpoint` for SingleRec and MutRec* in Lean (extract/ExtractBase.ml:1452-1461). It is appended unconditionally at Extract.ml:2373-2378. The `def` keyword comes from ExtractBase.ml:1441-1448.
- `-decreases-clauses`: adds `termination_by <name> args => <measure>` and `decreasing_by <proof>` (Extract.ml:2296-2359). A template file is generated unless `-no-template-clauses` is passed (Translate.ml:900-919; Extract.ml:1927ff). **[UNCERTAIN/inconsistent]**: the `partial_fixpoint` post-qualifier is still appended in this mode (Extract.ml:2373-2378 has no guard on `has_decreases_clause`), so the code appears to emit both. I have not tested whether Lean accepts that.
- Lean does not support `-use-fuel` (Main.ml:532-535). Fuel exists only for F* and Coq (PureMicroPassesGeneral.ml:3075-3079, 3227).
- No `divergent` keyword is emitted. Divergence is modelled by `Result.div` and `partial_fixpoint`.

---

## 3. CLI options (Main.ml:75-280; Config.ml)
Loops and termination:
- `-loops-to-rec`: always translate loops to recursive functions (Main.ml:230-232; Config.ml:475-476).
- `-loops-no-rec`: never do so, always use the `loop` combinator (Main.ml:238-240; Config.ml:614-615). It is mutually exclusive with `-loops-to-rec` (Main.ml:498-500).
- `-decreases-clauses`: termination measures and decreases proofs (Main.ml:117-120; Config.ml:239-248).
- `-no-template-clauses`: requires `-decreases-clauses` (Main.ml:124-128, 461-465).
- `-use-fuel`: rejected for Lean and HOL4, and incompatible with `-decreases-clauses` (Main.ml:121-123, 467-468, 532-535, 543-548).
- `-strict-joins`: disables the code-duplication fallback for failed joins (Main.ml:198-201; Config.ml:504-507).

Backend and output:
- `-backend {fstar,coq,rocq,lean,hol4}` (Main.ml:96-99; Config.ml:8).
- `-split-files` (Main.ml:129-132). With split files, opaque or external definitions go to `TypesExternal_Template` / `FunsExternal_Template` files that the user renames and fills in (Translate.ml:1895-1915, 2019-2037).
- `-gen-lib-entry` (requires `-split-files`), `-subdir`, `-namespace`, `-dest`, `-use-lean-modules`, `-lean-default-lakefile`, `-emit-json`, `-max-heartbeats`, `-max-recdepth`, `-all-computable` (by default `noncomputable section` is inserted when opaque definitions exist, Translate.ml:2107, 2139), `-impl-namespace`, `-filter-trait-methods`, `-feature-gates`, `-tuple-nested-proj`.

Checking and debugging:
- `-borrow-check` (borrow-check only), `-borrow-check-globals`, `-eval-drops`, `-test-units`, `-checks`, `-abort-on-error`, `-warnings-as-errors`, `-print-error-emitters`, `-print-unknown-externals`, `-print-llbc`, `-print-error-diagnostics`, `-log*`, `-mark-ids`, `-sequential`, `-no-progress-bar`, `-max-error-spans`, `-diagnose-*`.
- **`-backward-no-state-update` does not exist** in this version. A grep of Main.ml for `'( *"-'` lists every flag (Main.ml:78-271) and it is not among them.

---

## 4. Charon (pinned) limitations

### 4.1 Error machinery
- `register_error!` reports an error at WARNING level, or ERROR under `--error-on-warnings`. `raise_error!` is `return Err(register_error!(...))`. `error_assert!` is an assert that raises (charon/src/errors.rs:15-54). When `continue_on_failure` is false, `span_err` panics (errors.rs:264-290).
- Per-item recovery: a failing item is reported as "Item `...` caused errors; ignoring." or "Thread panicked when extracting item" (charon/src/bin/charon-driver/translate/translate_items.rs:36-53). A failing body becomes `Body::Error` (charon/src/ast/bodies.rs:84-86). Aeneas treats that as non-translatable, i.e. opaque, i.e. an axiom.

### 4.2 Categorized (all paths under charon/src/bin/charon-driver/translate/ unless stated)
- **Coroutines, async, generators**: "Coroutine types are not supported yet" (translate_types.rs:331-332); "Coroutines are not supported" (translate_bodies.rs:1485-1487, 2171-2174); "Unsupported terminator: ..." for `Yield`, `CoroutineDrop` and `TailCall` (translate_bodies.rs:1903-1906). So `async fn`, generators and `become` tail calls fail.
- **dyn Trait / vtables**: "`dyn Trait` with multiple method-bearing predicates is not supported" (translate_trait_objects.rs:95-110). Vtable errors for non-dyn-compatible traits and non-vtable-safe methods (translate_trait_objects.rs:466, 620, 1267, 1367, 1374). "vtables of trait alias impls are not supported" (translate_items.rs:1487). In transforms: dyn method calls become vtable function-pointer calls (transform/normalize/transform_dyn_trait_calls.rs:1-17) with their own errors (:83-90, 112, 134, 174-186, 210). "`dyn Trait` is unsupported with `--monomorphize-mut`" (transform/normalize/partial_monomorphization.rs:336-339).
- **Closures**: translated as structs plus trait impls (translate_closures.rs). Internal misuse errors only (translate_closures.rs:220, 257).
- **Function pointers**: `FnOperand::Dynamic` is produced for calls through values (translate_bodies.rs:1990-1999). Calling anything else gives "cannot call a value of type ..." (:2001-2005). fn pointer casts: "Expected function pointers in fn pointer cast" (:1290).
- **Unsafe binders**: "unsupported feature: unsafe binders" (translate_bodies.rs:1015); "charon does not support unsafe lifetime binders" (:1491-1496). Reborrow rvalues: "charon does not support reborrow rvalues (for Reborrow traits)" (:1498-1503).
- **Deref patterns**: "deref patterns are not supported" (translate_bodies.rs:228-230).
- **Thread-locals**: translated via `translate_thread_local_ref` (translate_bodies.rs:1489). Unexpected pointer types give "unexpected type for thread-local reference" (:848-852). On the Aeneas side they presumably hit the global-access restrictions in 1.9 **[UNCERTAIN]**.
- **Inline asm**: translated into an `InlineAsm` terminator/statement (translate_bodies.rs:1880-1902). Aeneas rejects it (1.5).
- **Types**: "Unsupported alias type" (translate_types.rs:191-192, i.e. alias kinds other than projection/opaque; `impl Trait` opaque aliases are resolved to the hidden type at :188-189). Also "Unsupported type: infer type", "Unsupported type: placeholder", "Unsupported type: {:?}" (Todo), and "Type checking error" (translate_types.rs:328-346).
- **Constants**: "Unsupported constant: ..." registers an error and yields `ConstantExprKind::Opaque` (translate_constants.rs:309-311).
- **Predicates / clauses**: "Unsupported clause" (translate_predicates.rs:238); trait-resolution errors (:265, 453).
- **GATs and associated types**: "GATs cannot work with the `--lift-associated-types` option (implied by `--preset=aeneas`). Either stop using this option or `--exclude` this trait." (transform/normalize/expand_associated_types.rs:1145-1150). Also "Could not compute the value of ... type aliases are allowed to make use of unproved trait facts" (:1128-1135) and "Could not figure out type for new associated type" (:1299-1305).
- **Control flow**: "the control-flow graph of this function is not reducible" followed by a panic (transform/control_flow/ullbc_to_llbc.rs:1362-1370).
- **Contracts with --monomorphize on generic items**: "contracts on generic items are not supported with `--monomorphize`" (translate_meta.rs:625-628).
- **Intrinsics / extern / FFI**: the body kinds are `Body::Intrinsic{name, fallback, ...}`, `Body::Extern(symbol)`, `Body::Opaque` and `Body::Missing` (ast/bodies.rs:39-86). They are selected in translate_items.rs:599-649: intrinsic, then extern block symbol, then box/drop-glue special cases, then opaque (by opacity), then a MIR body. None of these is a structured body, so Aeneas treats them all as opaque and extracts them as axioms, unless a builtin model maps the name (Translate.ml:938-958 builtin check; extract/ExtractBuiltin*.ml).
- **Unions**: Charon represents unions (`TypeDeclKind::Union`, referenced by Aeneas at llbc/TypesAnalysis.ml:544). The rejection happens in Aeneas.
- **Specialization, const generics**: no specific "unsupported" messages found in Charon translate/transform for these **[UNCERTAIN; not explicitly found]**.

### 4.3 Opacity: `--include`, `--opaque`, `--exclude`, attributes
- `ItemOpacity` (charon/src/ast/meta.rs:21-41):
  - `Transparent`: translate fully.
  - `Foreign`: types are translated only if they have public fields or are enums; everything else is treated like `Opaque`.
  - `Opaque`: translate the name and signature but not the body or fields. Modules are not explored.
  - `Invisible`: translate nothing.
- Defaults (charon/src/options.rs:930-957):
  - Everything (`_`) is `Foreign` unless `--extract-opaque-bodies` is passed, in which case it is `Transparent`.
  - `crate` is always `Transparent`.
  - `--include` patterns become `Transparent`, `--opaque` patterns become `Opaque`, and `--exclude` patterns are not translated (options.rs:136-170). When patterns overlap, the most precise one wins (options.rs:155-158).
- Item attributes: `#[charon::opaque]`, `#[charon::exclude]`, `#[charon::transparent]`, `rename`, `variants_prefix`, `start_from`, `contract`. They can also be spelled `aeneas::*` or `verify::*` (translate_meta.rs:692-711, 785, 811, 837).
- `--preset=aeneas` (options.rs:566-581) sets: lift all associated types (`*`), treat Box as builtin, ops/index to function calls, reconstruct fallible ops, panics, asserts and matches, hide marker traits and the allocator, remove unused self clauses and ADT clauses, unbind item vars, deallocate all locals, and no generated tuple structs. It does **not** enable `extract_opaque_bodies`, so external crates (including std, except for Aeneas builtin models) are `Foreign` by default and their function bodies are opaque.
- What Aeneas does with opaque functions: they are extracted as `axiom` (Lean) (ExtractBase.ml:1449-1450; Translate.ml:852-855). With `-split-files` they go to a `FunsExternal_Template` file the user fills in (Translate.ml:2019-2037). `noncomputable section` is added unless `-all-computable` is passed (Translate.ml:2107, 2139).
