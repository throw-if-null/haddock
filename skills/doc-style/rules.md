# Rules

This file is the full rule set of the `doc-style` skill. [deviations.md](deviations.md)
lists where these rules differ from ASD-STE100.

## Base rules

### Communication style

Use concise, controlled technical English.

Apply Simplified Technical English (STE) principles where practical. Do not enforce the STE controlled vocabulary when it conflicts with established software engineering terminology.

Apply these rules to text output, code discussions, architecture reviews, commit messages, PR summaries, and documentation.

- Prefer precise technical terminology.
- Use short, direct sentences.
- Prefer one requirement or instruction per sentence.
- Remove conversational filler, greetings, and unnecessary apologies.
- Avoid idioms, metaphors, rhetorical language, and vague qualifiers.
- Prefer literal descriptions of system behavior.
- Use the same term for the same concept.
- Do not use synonyms only to vary wording.
- State assumptions explicitly when they affect implementation or conclusions.
- Distinguish facts, requirements, assumptions, and recommendations.
- Do not simplify established technical terminology.
- Use MUST, MUST NOT, SHOULD, and MAY when requirement strength matters.
- Resolve ambiguity from available context when the decision is safe and reversible.
- Ask for clarification when ambiguity can cause a destructive, irreversible, security-sensitive, or materially incorrect change.

### Code explanations and architecture

- Describe components and relationships literally.
- Prefer "Component A sends data to Component B" over "Component A talks to Component B."
- State technical conclusions directly.
- Separate facts, requirements, assumptions, and recommendations.
- Explain reasoning when it materially helps implementation, debugging, or a technical decision.
- Do not add speculative requirements.
- Do not introduce abstractions for hypothetical future requirements.

### Documentation

- Prefer short sentences.
- Use one instruction per sentence when practical.
- Prefer active voice when it improves clarity.
- Use consistent terminology for the same concept.
- Do not use synonyms only to vary wording.
- Prefer lists for independent requirements or steps.
- Prefer examples when they clarify exact behavior.
- Do not duplicate information without a concrete reason.

## Sentences

- State one fact, requirement, or instruction per sentence.
- Keep sentences to 25 words or fewer. Split a longer sentence.
- Split a clause chain joined by an em dash, a semicolon, or a trailing `, which` into
  separate sentences.
- Use active voice when it names the component that acts.
- Use a colon to introduce a list. Do not use an em dash.

```text
Before: The distinction has teeth in two places. `build.sh` derives the image tag from the
        branch name, and `deploy.sh` reads the service directories — which is why an
        environment that does not list a service never gets its image.
After:  Two mechanisms depend on the distinction:

        - `build.sh` derives the image tag from the branch name.
        - `deploy.sh` reads the service directories under an environment to determine
          which images that environment requires.
```

## Words

- Use one term for one concept, in every document. Never substitute a synonym for variety.
- Replace every idiom and metaphor with the behaviour it described.
- Remove vague qualifiers: `just`, `simply`, `basically`, `obviously`, `of course`,
  `a bit`, `fairly`, `pretty much`, `worth doing`.
- Remove filler openers: `Note that`, `It is worth noting`, `Keep in mind`, `Let us`.
- Do not attribute intent to software. A component sends, reads, writes, or rejects. It
  does not know, want, or talk to.
- Keep established technical terminology. Do not simplify `idempotent`, `quorum`,
  `fail closed`, or `transaction`.

Frequent replacements:

| Do not write | Write |
| --- | --- |
| load-bearing, not decoration, has teeth | the requirement it states, with the failure it prevents |
| belt and braces | the second control, and what it covers that the first does not |
| that is the point | the literal reason |
| blast radius | what the credential can read, stated exactly |
| talks to, knows about, wants to | sends to, reads, requires |
| under the hood, magic | the named mechanism |
| worth knowing, worth the five minutes | the consequence of not knowing it, or nothing |
| surprises people, counterintuitive | the fact alone |
| leverage, utilize, seamless, robust, powerful | use, or the measurable property |

The full catalogue with before and after pairs is in
[examples/rewrites.md](examples/rewrites.md). Read it when rewriting an existing document,
or when a checker hit has no obvious literal replacement.

## Requirement strength

- Use MUST, MUST NOT, SHOULD, and MAY when the text states a requirement.
- Use the plain indicative when the text describes behaviour. A description MUST NOT be
  written as a requirement.
- Keep the keyword in upper case. Bold it only where the surrounding document already does.

```text
Requirement: Both migration scripts MUST be run by `db_admin`.
Description: The build copies the lock file to the output directory.
```

## Structure

- Use a list for independent requirements or steps.
- Use a table when every item has the same fields.
- Use an example when it fixes exact behaviour: a command, a path, a value.
- State each fact in one place. Link to it from anywhere else.
- Give each section a heading that names its subject, not its rhetorical role.

## Facts, requirements, assumptions, recommendations

Mark which one a sentence is. Use these frames:

| Type | Frame |
| --- | --- |
| Fact | `build.sh` writes the image tag to `tag.txt`. |
| Requirement | A service MUST NOT set its own image tag. |
| Assumption | This assumes the `staging` vault exists. |
| Recommendation | An API key SHOULD be narrowed to the endpoints its client calls. |

Do not state a recommendation as a fact. Do not state an assumption as a requirement.
