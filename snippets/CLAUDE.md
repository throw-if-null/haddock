## Documentation style

Use the `doc-style` skill before you write or edit documentation: Markdown files, READMEs,
ADRs, runbooks, long code comments, commit message bodies, and pull request descriptions.
The skill holds the full rules. In summary:

- State one fact, requirement, or instruction per sentence. Use 25 words or fewer.
- Do not join clauses with an em dash or a semicolon.
- Replace idioms, vague qualifiers, filler, and promotional words with literal statements.
- Do not attribute intent to software. A component sends, reads, writes, or rejects.
- Use one term for one concept.
- Use MUST, MUST NOT, SHOULD, and MAY for requirements, and plain statements for behavior.
- Use a list for independent requirements or steps. State each fact in one place.
- Mark each assumption and each recommendation as such.

Run the skill's checker, `scripts/check` in the skill directory, on each changed Markdown
file. Rewrite each finding, or keep it and state the reason.
