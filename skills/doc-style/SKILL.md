---
name: doc-style
description: Write and revise documentation in controlled technical English based on ASD-STE100 (Simplified Technical English). Use this skill BEFORE writing or editing any Markdown file, README, ADR, runbook, docs/ page, long code comment, commit message body, or pull request description, and whenever the user asks to review, rewrite, tighten, shorten, or fix the tone or style of existing prose. Use it for a single paragraph or a one-line edit as well as for a full document. Use it when a task ends in a written explanation of a system.
---

# Documentation style

Documentation MUST follow the rules in [rules.md](rules.md). This skill converts those rules
into a procedure, a set of checkable patterns, and a script.

| File | Content |
| --- | --- |
| [rules.md](rules.md) | The full rule set. |
| [deviations.md](deviations.md) | Where the rules differ from ASD-STE100, and why. |
| [examples/rewrites.md](examples/rewrites.md) | Before and after pairs for each defect class. |
| `scripts/check` | The checker. |

## Scope

Apply this skill to:

- Markdown files: `README.md`, `docs/**`, runbooks, ADRs, `CLAUDE.md`.
- Commit message bodies and pull request descriptions.
- Code comments longer than one line, and docstrings.

Do not apply it to: identifiers, quoted third-party text, or command output reproduced
verbatim.

## Procedure

1. Read the project agent instructions (`CLAUDE.md` or `AGENTS.md`) if they exist. A
   project rule overrides a rule in this skill when the two conflict. Report the conflict.
2. Read the file you are about to change. Read one sibling document in the same directory.
   Match the existing terminology, heading depth, and table conventions.
3. Write the draft.
4. Run the checker on every file you changed:

   ```bash
   ${CLAUDE_SKILL_DIR}/scripts/check docs/example.md
   ```

   The checker reports candidates, not errors. Judge each hit. Rewrite it, or keep it and
   state the reason. A hit inside a quotation, a rules table, or an example of what not to
   write is expected. Keep it.

   The patterns assume Markdown. Run it on a source file to check long comment blocks, and
   ignore the hits produced by the file's own syntax.
5. Run the manual pass. The checker cannot detect the items in
   [Manual pass](#manual-pass).
6. Report what changed. State explicitly when a rule was not applied and why.

## Manual pass

The checker cannot detect these. Verify each one before reporting the work complete:

- **Terminology.** One term per concept across the whole document set.
- **Duplication.** The same explanation MUST NOT appear in two documents. Keep one, link
  the other.
- **Requirement strength.** Every MUST is a real requirement. Every real requirement has a
  keyword.
- **Verification.** Every claim about behaviour is either verified or marked as an
  assumption.
- **Speculation.** No requirement, abstraction, or section exists for a hypothetical future
  need.
- **Rhetorical framing.** Headings and openers describe content, not the reader's expected
  reaction.

## Constraints

- Change prose only. A style rewrite MUST NOT change technical content. State separately
  when a rewrite exposed a factual error.
- Do not rewrite documents the task did not ask you to touch.
