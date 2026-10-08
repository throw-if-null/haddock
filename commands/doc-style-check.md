---
description: Run the doc-style checker on every Markdown file in the change set. Use it before you open a pull request.
allowed-tools: Bash(git diff *) Bash(xargs *)
---

The checker output for every Markdown file that the branch changes against `main`:

!`git diff --name-only --diff-filter=d --merge-base main -- '*.md' | xargs -r "${CLAUDE_PLUGIN_ROOT}/skills/doc-style/scripts/check" 2>&1 || true`

Each line of the output names the file, the line, the rule, the matched text, and the
source line. The checker reports candidates, not errors. Rewrite each one, or keep it and
state the reason. Empty output means that the checker found no candidate.
