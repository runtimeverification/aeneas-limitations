# What Aeneas supports today

An evidence-based check of what [Aeneas](https://github.com/AeneasVerif/aeneas) (Rust → Lean) and its front-end [Charon](https://github.com/AeneasVerif/charon) support today. Claims come from experiments run against upstream `main`, from the documentation and source code at the tested commit, and from GitHub issues. 100 small Rust programs were run through Charon and Aeneas, and the generated Lean was type-checked.

**Report:** [aeneas-support-report.pdf](aeneas-support-report.pdf) (Markdown source in [report/](report/)).

## Versions tested

Tested on 2026-11-10. Details in [VERSIONS.md](VERSIONS.md).

- Aeneas `0d21a9ec` (upstream `main`, 2026-11-09)
- Charon `454586e9` (the commit Aeneas pins, 2026-11-08)
- Lean `v4.36.0-rc2`

## Loops and iterators, in short

Aeneas does translate loops and iterators. Of 38 loop/iterator programs, 24 are clean, 7 have caveats (axioms or `sorry` in the Lean output) and 7 fail.

- Supported: `while`, `loop`/`break`/`continue`, nested loops (without jumps across levels), `for` over ranges, slices and `Vec`, `enumerate`, `zip`, `rev`, `step_by`, user-defined `Iterator` impls.
- Not supported: `break`/`continue` to an outer loop, `return` from a nested loop, loops with no exit.
- Not modelled: `map`, `filter`, `fold`, `sum`, `any`, `count` become axioms; `map` + `collect` gives ill-typed Lean.
- Termination and correctness of loops are proved by the user, after translation.

See §1.2–1.3 of the report for the summary and §2 for details.

## Layout

| Path | Contents |
|---|---|
| `aeneas-support-report.pdf` | The report |
| `report/` | Markdown source (`main.md`, `appendix.md`, `appendix-proofs.md`), generated `.md`/`.html`, CSS |
| `experiments/cases/` | 100 Rust test programs |
| `experiments/out/`, `experiments/out-loops-to-rec/` | Charon/Aeneas output, generated Lean, logs and exit codes per case |
| `experiments/proofs/` | Lean loop proofs and their check logs |
| `experiments/leancheck/` | Lake project used to type-check generated Lean against the Aeneas library |
| `experiments/*.sh`, `*.py` | Scripts that run the experiments and build the result tables |
| `notes/` | Survey notes: documentation and tests, source code, GitHub issues |
| `scripts/` | GitHub data scripts and the report build script |
| `logs/` | Toolchain build logs |
| `VERSIONS.md` | Upstream commits tested |
| `env.sh.example` | Environment for the isolated toolchains |

## How to reproduce

Full instructions are in §5 of the report. In short, from the repository root:

```bash
git clone https://github.com/AeneasVerif/aeneas.git && git -C aeneas checkout 0d21a9ec7bbbb110110cacc14464a47fd71c8705
git clone https://github.com/AeneasVerif/charon.git
git -C charon worktree add ../charon-pinned $(tail -1 aeneas/charon-pin)
cp env.sh.example env.sh && source env.sh
# build Charon, Aeneas and the Aeneas Lean library as described in §5, then:
experiments/run_charon.sh
experiments/run_aeneas.sh
python3 -I experiments/classify.py experiments/out > experiments/results.md
```

`scripts/build_report.sh` rebuilds the HTML and PDF from `report/*.md` (needs pandoc and Chromium). The scripts in `scripts/` that read GitHub data need `notes/gh/`, which is not included; `scripts/fetchall.sh` refetches it.

Results are valid for the tested commits only.
