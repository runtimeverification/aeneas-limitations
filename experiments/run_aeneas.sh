#!/usr/bin/env bash
# Run Aeneas (Lean backend) on each case's .llbc, then type-check the generated Lean
# against the Aeneas Lean library. Output: out/<case>/{aeneas.log,aeneas.exit,lean/,lean.log,lean.exit}
set -u
cd "$(dirname "$0")"
source ../env.sh
cases=("$@"); [ ${#cases[@]} -eq 0 ] && cases=($(ls cases | sed 's/\.rs$//'))
for c in "${cases[@]}"; do
  o=${OUT:-out}/$c; mkdir -p $o; [ -f $o/$c.llbc ] || cp out/$c/$c.llbc out/$c/charon.exit $o/ 2>/dev/null; rm -rf $o/lean; mkdir -p $o/lean
  [ -f $o/$c.llbc ] || { echo "no llbc" > $o/aeneas.log; echo 99 > $o/aeneas.exit; continue; }
  aeneas -backend lean $AENEAS_EXTRA $o/$c.llbc -dest $o/lean > $o/aeneas.log 2>&1
  echo $? > $o/aeneas.exit
  f=$(ls $o/lean/*.lean 2>/dev/null | head -1)
  if [ -n "$f" ]; then
    (cd leancheck && timeout 600 lake env lean ../$f) > $o/lean.log 2>&1; echo $? > $o/lean.exit
  else echo "no lean file" > $o/lean.log; echo 98 > $o/lean.exit; fi
done
