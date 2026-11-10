#!/usr/bin/env bash
# Build report/aeneas-support-report.{md,html} and aeneas-support-report.pdf from report/*.md.
# Needs pandoc (3.5 used) and a Chromium-based browser (set CHROME to override).
set -eu
cd "$(dirname "$0")/../report"
CHROME=${CHROME:-chromium}
{ cat main.md; printf '\n<div class="pb"></div>\n\n'; cat appendix.md; printf '\n<div class="pb"></div>\n\n'; cat appendix-proofs.md; } \
  | sed 's|^\\newpage$|<div class="pb"></div>|' > aeneas-support-report.md
pandoc aeneas-support-report.md -s --toc --toc-depth=2 --embed-resources --css style.css -o aeneas-support-report.html
"$CHROME" --headless --no-pdf-header-footer --print-to-pdf="$PWD/../aeneas-support-report.pdf" "file://$PWD/aeneas-support-report.html"
