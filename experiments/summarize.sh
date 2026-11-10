#!/usr/bin/env bash
# Print, per case: Aeneas error lines, axioms emitted, Lean errors.
cd "$(dirname "$0")"
for d in ${OUT:-out}/*; do c=$(basename $d)
  echo "### $c  charon=$(cat $d/charon.exit) aeneas=$(cat $d/aeneas.exit) lean=$(cat $d/lean.exit)"
  sed 's/\x1b\[[0-9;?]*[a-zA-Z]//g' $d/aeneas.log | grep -E -A2 "Error|Warning|Unsupported|not supported|Unimplemented" | grep -v "^--$" | head -12 | sed 's/^/  A| /'
  grep -h -E "^ *(axiom|opaque) " $d/lean/*.lean 2>/dev/null | sed 's/^/  X| /'
  grep -h -E "error" $d/lean.log 2>/dev/null | head -4 | cut -c1-260 | sed 's/^/  L| /'
done
