# Classify each experiment from exit codes and generated Lean. Usage: python3 -I classify.py <outdir>
import sys, os, re, glob
out = sys.argv[1]
print("| case | charon | aeneas | lean | axioms | sorry | status |")
print("|---|---|---|---|---|---|---|")
for c in sorted(os.listdir(out)):
    d = os.path.join(out, c)
    rd = lambda f: open(os.path.join(d, f)).read().strip() if os.path.exists(os.path.join(d, f)) else "-"
    ch, ae, le = rd("charon.exit"), rd("aeneas.exit"), rd("lean.exit")
    src = "".join(open(f).read() for f in glob.glob(d + "/lean/*.lean"))
    ax = len(re.findall(r"^\s*axiom ", src, re.M))
    so = len(re.findall(r"\bsorry\b", src))
    if ch != "0" or ae != "0" or le != "0": st = "FAIL"
    elif ax or so: st = "CAVEATS"
    else: st = "CLEAN"
    print(f"| {c} | {ch} | {ae} | {le} | {ax} | {so} | {st} |")
