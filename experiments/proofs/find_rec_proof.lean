
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
