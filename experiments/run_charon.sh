#!/usr/bin/env bash
# Run Charon (aeneas preset) on every case, or on the cases given as arguments.
# Output: out/<case>/<case>.llbc and out/<case>/charon.log
set -u
cd "$(dirname "$0")"
source ../env.sh
cases=("$@"); [ ${#cases[@]} -eq 0 ] && cases=($(ls cases | sed 's/\.rs$//'))
for c in "${cases[@]}"; do
  o=out/$c; mkdir -p $o; rm -f $o/$c.llbc
  extra=()
  [ "$c" = F02_opaque_module ] && extra=(--opaque 'crate::hidden')
  crate=$(echo "$c" | tr 'A-Z' 'a-z')
  charon rustc --preset=aeneas "${extra[@]}" --dest-file $o/$c.llbc -- \
    --crate-type=lib --edition=2021 --crate-name "$crate" cases/$c.rs > $o/charon.log 2>&1
  echo $? > $o/charon.exit
done
