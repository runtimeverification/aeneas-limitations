
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
