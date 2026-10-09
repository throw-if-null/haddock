# haddock

This repository holds the `doc-style` skill. Every Markdown file in it, and every commit
message, MUST follow that skill. Load it from `skills/doc-style/SKILL.md` before you write
or edit prose.

## Verification

- Run `claude plugin validate .` after a change to `.claude-plugin/`, to the skill
  frontmatter, or to the repository layout.
- After a change to the skill, its examples, or the primer, run the affected eval cases:
  `claude plugin eval . --scaffold --allow-tools Edit Write --case NAME`. Ask the user
  before a run. Each run starts Claude sessions with the user's credentials.
- Run `shellcheck` and `shfmt -d` on every changed shell script.

## Constraints

- Do not add a rule, remove a rule, or change a rule's behavior without a request.
- Do not mention other agents. The skill targets Claude Code only.
