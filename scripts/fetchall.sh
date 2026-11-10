#!/bin/bash
# usage: fetchall.sh URL prefix maxpages
url="$1"; pre="$2"; max="${3:-10}"; i=1
mkdir -p "$(dirname "$0")/../notes/gh" && cd "$(dirname "$0")/../notes/gh" || exit 1
while [ -n "$url" ] && [ $i -le $max ]; do
  for t in 1 2 3; do curl -s --retry 3 -D "${pre}_p$i.json.hdr" -o "${pre}_p$i.json" "$url" && break; sleep 2; done
  rem=$(grep -i '^x-ratelimit-remaining' "${pre}_p$i.json.hdr" | tr -d '\r')
  n=$(python3 -I -c "import json,sys;d=json.load(open(sys.argv[1]));print(len(d) if isinstance(d,list) else d)" "${pre}_p$i.json")
  echo "$pre p$i: $n  $rem"
  url=$(grep -i '^link:' "${pre}_p$i.json.hdr" | tr ',' '\n' | grep 'rel="next"' | sed -E 's/.*<([^>]+)>.*/\1/')
  i=$((i+1))
done
