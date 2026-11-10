# Generate Markdown appendices with the Rust input, status, Aeneas errors, axioms and
# (stripped) generated Lean for each experiment. Usage: python3 -I gen_appendix.py > ../report/appendix.md
import os, re, glob
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "out")
ANSI = re.compile(r"\x1b\[[0-9;?]*[a-zA-Z]")
status = {}
for line in open(os.path.join(HERE, "results.md")):
    p = [x.strip() for x in line.split("|")]
    if len(p) > 7 and p[1] not in ("case", "---"):
        status[p[1]] = p[7]

def rd(c, f):
    p = os.path.join(OUT, c, f)
    return open(p, errors="replace").read() if os.path.exists(p) else ""

def strip_lean(src, maxlines):
    m = src.find("\nnamespace ")
    body = src[m + 1:] if m >= 0 else src
    body = re.sub(r"/--(.*?)-/", lambda mm: "-- " + mm.group(1).strip().splitlines()[0], body, flags=re.S)
    out, skip = [], False
    for l in body.splitlines():
        s = l.strip()
        if s.startswith("Source:") or s.startswith("Visibility:") or s.startswith("Name pattern:"):
            continue
        if s == "":
            continue
        out.append(l)
    if len(out) > maxlines:
        out = out[:maxlines] + [f"-- ... ({len(out) - maxlines} more lines, see experiments/out/)"]
    return "\n".join(out)

def errors(c):
    log = ANSI.sub("", rd(c, "aeneas.log"))
    msgs = []
    for l in log.splitlines():
        for m in re.finditer(r"\[(Error|Warn) *\] (.*)", l):
            t = f"[{m.group(1)}] {m.group(2).strip()}"
            if t not in msgs:
                msgs.append(t)
    return msgs

def section(c, maxlines):
    s = [f"### {c}  —  **{status.get(c, '?')}**", ""]
    s += ["```rust", open(os.path.join(HERE, "cases", c + ".rs")).read().rstrip(), "```", ""]
    s.append(f"Exit codes: charon={rd(c,'charon.exit').strip()}, aeneas={rd(c,'aeneas.exit').strip()}, lean={rd(c,'lean.exit').strip()}.")
    s.append("")
    ch = [l for l in rd(c, "charon.log").splitlines() if l.startswith("warning: ") and "unused" not in l and "warnings emitted" not in l and "warning emitted" not in l and "unnecessary" not in l]
    if ch:
        s += ["Charon:", "```text"] + sorted(set(ch)) + ["```", ""]
    e = errors(c)
    if e:
        s += ["Aeneas messages:", "```text"] + [x[:300] for x in e[:8]] + ["```", ""]
    le = [l[:300] for l in rd(c, "lean.log").splitlines() if ": error" in l][:4]
    if le:
        s += ["Lean errors:", "```text"] + le + ["```", ""]
    lean = "".join(open(f).read() for f in glob.glob(os.path.join(OUT, c, "lean", "*.lean")))
    if lean and maxlines:
        s += ["Generated Lean:", "```lean", strip_lean(lean, maxlines), "```", ""]
    return "\n".join(s)

cases = sorted(x[:-3] for x in os.listdir(os.path.join(HERE, "cases")))
print("# Appendix A — Loop and iterator experiments (full)\n")
print("Every case below was produced by `experiments/run_charon.sh` and `experiments/run_aeneas.sh` (default options). Lean output is shown with each `/-- ... -/` doc comment collapsed to a one-line `--` comment and blank lines removed; nothing else is edited. Long outputs are truncated (marked).\n")
for c in cases:
    if c.startswith("L"):
        print(section(c, 90)); print()
print("# Appendix B — Other feature experiments\n")
for c in cases:
    if not c.startswith("L"):
        print(section(c, 45)); print()
