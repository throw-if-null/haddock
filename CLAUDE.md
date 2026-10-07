# haddock

This repository holds the `doc-style` skill. Every Markdown file in it, and every commit
message, MUST follow that skill. Load it from `skills/doc-style/SKILL.md` before you write
or edit prose.

## Verification

- Run `tests/run` after a change to the checker, the hook, the tests, or the snippets.
- Run `tests/mutate` after a change to a checker rule. It takes several minutes.
- Run `shellcheck` and `shfmt -d` on every changed shell script and `.bats` file.
- Run `skills/doc-style/scripts/check` on every changed Markdown file.

## Constraints

- Do not add a rule, remove a rule, or change a rule's behavior without a request.
- A fail fixture in `tests/fail/` MUST state its rule in an HTML comment on line 1.
- Do not mention other agents. The skill targets Claude Code only.
