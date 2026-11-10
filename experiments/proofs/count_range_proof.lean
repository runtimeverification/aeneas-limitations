
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
