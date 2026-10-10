---
name: doc-maintenance
description: Keep code comments, docstrings, and documentation accurate when code changes. Update, consolidate, or delete the prose that a change makes wrong or redundant, and write documentation that is direct, concise, and free of change history.
when_to_use: Use whenever you edit code (a feature, bug fix, refactor, rename, optimization, or removal), even when the user asks only for the code change and does not mention documentation or comments. Also use it before you write or edit a README, docs/ page, runbook, ADR, CLAUDE.md, code comment, docstring, commit message, or pull request description, and when you review prose or comments for clarity, concision, duplication, or outdated content.
---

# Documentation maintenance

Keep documentation and code comments an accurate, concise description of the current
system. Documentation is maintained, not accumulated: edit, consolidate, or delete existing
prose before you add more.

## Scope

Apply this skill whenever you change code, even when the task does not mention
documentation. A feature, a fix, a refactor, a rename, or a removal can make comments,
docstrings, and documents wrong or redundant.

Apply it also when you write, edit, or review:

- READMEs, `docs/` pages, runbooks, ADRs, and `CLAUDE.md` files.
- Code comments and docstrings.
- Commit messages and pull request descriptions. The [writing style](#writing-style)
  applies to them. The [current-state rules](#describe-the-current-system) do not, because
  these texts describe a change.

Do not apply it to quoted third-party text, reproduced command output, generated files,
vendored code, or license text.

Project instructions and established project conventions take precedence over this skill.

## Workflow

Scale each step to the change. A one-line fix requires a review of the comments on that
line and of the documentation of the changed behavior. It does not require a review of the
repository.

1. Identify the behavior, interface, or constraint that the change affects.
2. Find the prose that describes it:
   - Comments and docstrings in and next to the changed code.
   - Documents that name a changed identifier: a function, type, command, flag,
     configuration key, environment variable, endpoint, or error message. Search the
     repository for these names.
   - Documents that describe the changed behavior without naming an identifier, for
     example a table of supported values. A search for the changed names does not find
     them. Read the README section and the documentation page that cover the changed
     component, even when the search finds no match.
3. Compare each passage with the code after the change. Decide what a reader needs to know
   now.
4. Edit the affected passages, and only those, so that they are accurate. When two of them
   state the same fact, keep the fact in the passage that owns the subject, and replace the
   other with a link. See [Edit before you add](#edit-before-you-add).
5. Confirm that each passage is accurate, necessary, concise, and stated in one place.
6. In the task summary, report meaningful documentation changes in a sentence or two. Do
   not list every comment you reviewed.

No documentation change is often the correct result. Leave correct documentation as it is.

## Edit before you add

Prefer these actions, in this order:

1. Delete prose that is no longer true or no longer needed.
2. Correct prose that is partly wrong.
3. Consolidate passages that state the same fact.
4. Replace an explanation of a superseded design.
5. Add prose for information that a reader needs and no passage holds. Put it where a
   reader looks for the subject, usually in an existing section.

The order is a preference. It is not an instruction to delete useful information.

- Rewrite an outdated passage. Do not append a correction, an "Update:" note, or a
  paragraph that explains what changed.
- Revise an existing paragraph when the revision can hold the new information. Do not add a
  paragraph beside it.
- State each fact in one place: the document or comment that owns the subject. Link to it
  from other places. Before you remove a copy, confirm that the copies state the same fact.
  When they disagree, the code decides which statement is correct.

## Describe the current system

Documentation and comments describe current behavior, interfaces, requirements,
constraints, and the rationale for the current design. They are not a change log.

Do not write narration such as the following. Remove it from a passage that you edit:

- "Previously, the implementation used ..."
- "This was changed to ..."
- "We recently introduced ..."
- "The old approach ..."
- "During the migration ..."
- "This was added to fix ..."

Change history belongs in commit messages, pull request descriptions, changelogs, and
migration guides.

Keep historical information when a current constraint or decision depends on it:

- "Timestamps use the `Z` suffix because the v1 mobile client rejects `+00:00`" states a
  current constraint. Keep it.
- "We moved from MySQL to PostgreSQL in 2023" states history. Remove it, unless a reader
  needs it to understand the current design.
- When narration contains a current reason, keep the reason and remove the narration.
- An issue or ticket ID can stay as the source of a workaround.

Some documents record history by design: ADRs, changelogs, migration guides, and release
notes. Preserve their historical content. Do not rewrite an ADR to describe the current
state. When a decision changes, follow the project's ADR convention, for example a new ADR
and a "Superseded" status on the old one. Update the current-state documents separately.

## Comments and docstrings

A comment gives information that the code does not show:

- A constraint or an invariant.
- The reason for a design decision.
- A compatibility requirement, or an external contract or behavior.
- The reason that an apparently simpler approach is incorrect.
- A defect workaround, with a reference to its source.

Do not write a comment that restates the code, names the next step, or marks a change, such
as "new", "updated", or "moved here". When code needs a comment only because its names or
structure are unclear, prefer clearer code, within the scope of the task.

A docstring documents the interface: what a caller can rely on. When a docstring is
necessary, document the non-obvious caller-visible contract, including relevant
constraints, errors, and side effects. Do not repeat information already clear from the
signature, types, or surrounding code. Do not describe how the function works. A
performance or thread-safety statement belongs in a docstring only when the interface
guarantees it and the project documents such guarantees.

When you change code, reconsider its comments:

- Delete a comment that the change made obsolete.
- Move a comment whose rationale still applies to the code that it now describes.
- Do not add a comment that explains the change. The commit message explains the change.

## Accuracy over brevity

- Keep every qualification, assumption, warning, and constraint that is still true. A
  shorter text that loses one of them is wrong.
- Do not change executable behavior in a documentation-only edit.
- Do not invent behavior, requirements, or rationale. When a passage gives no reason, do
  not supply one.
- When a rewrite can change technical meaning, establish the meaning from the code, the
  tests, or the commit history. Ask the user when the evidence is not sufficient.
- Do not edit documentation that the change does not affect, for style or for any other
  reason. When you find an error in it, report the error.

## Writing style

- State the subject directly. Do not open with an introduction, a summary of what follows,
  or rhetorical framing.
- Use literal language. Replace an idiom or a metaphor with the behavior it describes.
- Describe what software does: a component reads, writes, sends, or rejects. Do not
  attribute intent to software when the phrasing hides the mechanism.
- Remove words that carry no information: filler, empty qualifiers, and promotional words.
  Judge each word in context. "Usually" is correct when the behavior has exceptions.
- Use one term for one concept. Do not change terms for variety.
- Prefer short sentences. Split a sentence that holds unrelated facts. Keep a longer
  sentence when a split separates a condition from its consequence.
- Use a list for independent items or steps, a table when items have the same fields, and
  an example for exact behavior: a command, a path, a value.
- Use MUST, MUST NOT, SHOULD, and MAY when a document states requirements and their
  strength matters. Describe behavior in the plain indicative.
- Distinguish facts, requirements, assumptions, and recommendations when the distinction
  affects a decision.
- Match the terminology, heading style, and formatting of the surrounding document.

## Examples

Read the relevant file when the rules above do not settle a decision:

- [examples/maintenance.md](examples/maintenance.md): what to change during code changes
  and document edits.
- [examples/rewrites.md](examples/rewrites.md): sentence rewrites for the writing style.
