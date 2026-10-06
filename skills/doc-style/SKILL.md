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

   `${CLAUDE_SKILL_DIR}` is the directory that holds this file. Claude Code substitutes it.
   Other agents, for example Codex, use the path of that directory.

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

## Suppression

Suppress a finding only when the text is correct as written and a rewrite cannot remove the
finding. Examples are a quoted prompt and a list of excluded words. State the reason in a
comment next to the suppression.

A rule ID is a label that the checker prints: `idiom`, `qualifier`, `filler`,
`anthropomorphism`, `hype`, `tone`, `chain`, or `length`. `all` means every rule.

A line that holds only a disable comment suppresses the listed rules. The region ends at a
line that holds only the matching enable comment, or at the end of the file. A comment
without a rule ID applies to every rule.

```markdown
<!-- The prompt is quoted from the installer. -->
<!-- doc-style-disable tone -->
The installer stops at the prompt Continue?
<!-- doc-style-enable tone -->
```

To suppress rules in whole files, add a line to a `.doc-style` file. Each line is a path
pattern and one or more rule IDs. The pattern matches the path relative to the
`.doc-style` file, and `*` also matches `/`. A line that starts with `#` is a comment.

```text
# The glossary lists the words that the rules exclude.
docs/glossary.md   idiom qualifier filler hype
```

The checker uses the nearest `.doc-style` file between the checked file and the root of its
git repository. A `.doc-style` file above the repository root does not apply.

## Constraints

- Change prose only. A style rewrite MUST NOT change technical content. State separately
  when a rewrite exposed a factual error.
- Do not rewrite documents the task did not ask you to touch.
