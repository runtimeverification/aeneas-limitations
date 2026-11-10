# Versions tested

Upstream repositories were cloned next to this repository's files and are not included here.
Clone them at these commits to reproduce (see the README).

| Directory | Repository | Commit | Commit date | Role |
|---|---|---|---|---|
| `aeneas/` | https://github.com/AeneasVerif/aeneas | `0d21a9ec7bbbb110110cacc14464a47fd71c8705` | 2026-11-09 | Aeneas upstream `main`, tested |
| `charon-pinned/` | https://github.com/AeneasVerif/charon | `454586e9b770231e66d2062993e632dfc06463e2` | 2026-11-08 | Charon commit pinned in `aeneas/charon-pin`, used by Aeneas |
| `charon/` | https://github.com/AeneasVerif/charon | `b9da4e3a108a5c2092fe80162bbd20ca1986d616` | 2026-11-09 | Charon upstream `main`, recorded only, not used |

Toolchains:

- Lean `v4.36.0-rc2` (from `aeneas/backends/lean/lean-toolchain`), Mathlib `9d581dfff2e41de989072ea63abf3daf69311520`
- Rust `nightly-2026-09-17` (from Charon's `rust-toolchain`)
- OCaml 5.3.0, with `visitors 20260520` and `ppx_deriving_hash 0.1.5` pinned
- pandoc 3.5 (report build only)

Experiments were run on 2026-11-10.
