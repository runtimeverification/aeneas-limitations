# Appendix C — Loop proofs (checked with Lean 4.36.0-rc2, exit code 0)

Each file below is appended to the Aeneas-generated Lean file shown in Appendix A and checked with `lake env lean` against the Aeneas library (`experiments/proofs/*.lean`, logs `*.log`, exit codes `*.exit`).

## L04, default mode (loop combinator, loop.spec_decr_nat)

```lean

/-! ## Proof (hand-written): default mode, `find_loop` is defined with the `loop` combinator.
    We apply `loop.spec_decr_nat` and supply: a Nat measure, an invariant, a postcondition,
    and prove that one iteration of `find_loop.body` preserves the invariant and decreases the measure. -/
open Aeneas Aeneas.Std Result l04_early_return

def FindPost (s : Slice Std.U32) (x : Std.U32) : Option Std.Usize → Prop
  | some j => j.val < s.length ∧ s.val[j.val]! = x
  | none => ∀ k, k < s.length → s.val[k]! ≠ x

theorem find_loop_spec (s : Slice Std.U32) (x : Std.U32) (i : Std.Usize)
    (hi : i.val ≤ s.length) (hprev : ∀ k, k < i.val → s.val[k]! ≠ x) :
    find_loop s x i ⦃ r => FindPost s x r ⦄ := by
  unfold find_loop
  apply loop.spec_decr_nat
    (measure := fun (i : Std.Usize) => s.length - i.val)
    (inv := fun (i : Std.Usize) => i.val ≤ s.length ∧ ∀ k, k < i.val → s.val[k]! ≠ x)
  · rintro i' ⟨hi', hprev'⟩
    unfold find_loop.body
    simp only []
    split
    · step as ⟨ v, hv ⟩
      have hlt : i'.val < s.val.length := by scalar_tac
      split
      · simp_all [FindPost]
      · step as ⟨ i3, hi3 ⟩
        refine ⟨by scalar_tac, ?_, by scalar_tac⟩
        intro k hk
        by_cases hk' : k = i'.val
        · subst hk'; simp_all
        · exact hprev' k (by scalar_tac)
    · simp only [WP.spec_ok, FindPost]
      intro k hk; exact hprev' k (by scalar_tac)
  · exact ⟨hi, hprev⟩

theorem find_spec (s : Slice Std.U32) (x : Std.U32) :
    find s x ⦃ r => FindPost s x r ⦄ := by
  unfold find
  apply find_loop_spec <;> simp
```

## L04 with -loops-to-rec (recursion + termination_by)

```lean

/-! ## Proof (hand-written): `-loops-to-rec` mode, recursive `find_loop` defined by `partial_fixpoint`.
    Termination is proved *extrinsically* inside the spec via `termination_by`/`decreasing_by`. -/
open Aeneas Aeneas.Std Result l04_early_return

theorem find_loop_spec (s : Slice Std.U32) (x : Std.U32) (i : Std.Usize)
    (hi : i.val ≤ s.length) (hprev : ∀ k, k < i.val → s.val[k]! ≠ x) :
    find_loop s x i ⦃ r =>
      match r with
      | some j => j.val < s.length ∧ s.val[j.val]! = x
      | none => ∀ k, k < s.length → s.val[k]! ≠ x ⦄ := by
  unfold find_loop
  simp only []
  split
  · step as ⟨ v, hv ⟩
    have hlt : i.val < s.val.length := by scalar_tac
    split
    · simp_all
    · step as ⟨ i3, hi3 ⟩
      apply find_loop_spec
      · scalar_tac
      · intro k hk
        by_cases hk' : k = i.val
        · subst hk'; simp_all
        · exact hprev k (by scalar_tac)
  · simp only [WP.spec_ok]
    intro k hk; exact hprev k (by scalar_tac)
termination_by s.length - i.val
decreasing_by scalar_decr_tac

theorem find_spec (s : Slice Std.U32) (x : Std.U32) :
    find s x ⦃ r =>
      match r with
      | some j => j.val < s.length ∧ s.val[j.val]! = x
      | none => ∀ k, k < s.length → s.val[k]! ≠ x ⦄ := by
  unfold find
  apply find_loop_spec <;> simp
```

## L30, for-over-range, default mode

```lean

/-! ## Proof (hand-written): `for _ in 0..n` loop, default `loop`-combinator mode.
    The iterator state (`Range U32`) is part of the loop state; we reason about
    `IteratorRange.next` with the library's `@[step]` lemma. -/
open Aeneas Aeneas.Std Result l30_for_range_count

theorem count_range_loop_spec (n : Std.U32) (it : core.ops.range.Range Std.U32) (c : Std.U32)
    (hend : it.end = n) (hle : it.start.val ≤ n.val) (hc : c.val = it.start.val) :
    count_range_loop it c ⦃ r => r.val = n.val ⦄ := by
  unfold count_range_loop
  apply loop.spec_decr_nat
    (measure := fun (p : core.ops.range.Range Std.U32 × Std.U32) => n.val - p.1.start.val)
    (inv := fun p => p.1.end = n ∧ p.1.start.val ≤ n.val ∧ p.2.val = p.1.start.val)
  · rintro ⟨it', c'⟩ ⟨hend', hle', hc'⟩
    unfold count_range_loop.body
    step as ⟨ o, it1, hnext ⟩
    split at hnext
    · obtain ⟨⟨rfl, hst⟩, hend1⟩ := hnext
      simp only []
      step as ⟨ c1, hc1 ⟩
      refine ⟨by simp_all, by scalar_tac, by scalar_tac, by scalar_tac⟩
    · obtain ⟨⟨rfl, hst⟩, hend1⟩ := hnext
      simp only [WP.spec_ok]
      scalar_tac
  · exact ⟨hend, hle, hc⟩

theorem count_range_spec (n : Std.U32) : count_range n ⦃ r => r.val = n.val ⦄ := by
  unfold count_range
  apply count_range_loop_spec <;> simp
```

