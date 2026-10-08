# haddock

Claude Code skills for technical writing. The first skill, `doc-style`, writes and checks
documentation in controlled technical English. Its goal is documentation and code
comments that are literal, short, and easy for a human to understand.

The rules are inspired by ASD-STE100, Simplified Technical English. They are not an
implementation of the standard. They are opinionated by design: they fix one sentence
length, one set of requirement keywords, and one word for each concept.
[deviations.md](skills/doc-style/deviations.md) lists where the rules differ from the
standard, and why.

## Three layers

Each layer has one job.

| Layer | Files | Job |
| --- | --- | --- |
| Primer | `snippets/CLAUDE.md` | Names the skill in a project's `CLAUDE.md`, so Claude loads it before it writes. |
| Skill | `skills/doc-style/` | Holds the full rules, the procedure, and the examples. This is the only place the full rules are stated. |
| Checker | `skills/doc-style/scripts/check` | Reports the constructions the rules exclude. |

## Requirements

- Linux. The checker needs Bash 4 or later and GNU grep.
- For the tests: `bats-core` 1.7.0 or later, `shellcheck`, and `shfmt`. `mise install`
  installs the pinned versions from `mise.toml`.

## Install the skill

Clone the repository, then link the skill into the directory Claude Code scans for
personal skills:

```bash
git clone https://github.com/throw-if-null/haddock.git ~/.local/share/haddock
ln -s ~/.local/share/haddock/skills/doc-style ~/.claude/skills/doc-style
```

Claude Code loads the skill when a task matches its description, and `/doc-style` loads
it on demand. `/skills` lists the loaded skills.

To install the skill for one project instead, link it into the project's
`.claude/skills/` directory.

## Add the primer

Append `snippets/CLAUDE.md` to the `CLAUDE.md` of each project that uses the skill:

```bash
cat ~/.local/share/haddock/snippets/CLAUDE.md >> CLAUDE.md
```

The primer names the skill, summarizes the rules, and tells Claude to run the checker.

## Run the checker

```bash
~/.local/share/haddock/skills/doc-style/scripts/check README.md docs/*.md
```

Each line of the output names the file, the line, the rule, the matched text, and the
source line:

```text
README.md:12: [idiom] load-bearing | The retry limit is load-bearing.
```

The exit status is 0 with no findings, 1 with findings, and 2 for a usage error, a missing
file, or an invalid suppression. The checker reports candidates, not errors. Rewrite each
one, or keep it and state the reason.

The checker reads a `.md` or `.markdown` file as Markdown. It reads any other file as
source code, and checks only the comments: lines that start with `//`, `#`, `--`, or `;`,
and blocks between `/*` and `*/` or between `"""` and `"""`. `--comments` reads every file
as source code. In source code, the `tone` rule does not apply to a line that starts with
`TODO`, `FIXME`, `XXX`, or `NOTE`. A trailing `?` is not reported.

```bash
~/.local/share/haddock/skills/doc-style/scripts/check src/worker.py
```

## Suppress a rule

A rule ID is a label the checker prints: `idiom`, `qualifier`, `filler`, `anthropomorphism`,
`hype`, `tone`, `chain`, or `length`. `all` means every rule.

Suppress a rule inside a file with a comment that holds only the directive. The region
ends at the matching enable comment, or at the end of the file:

```markdown
<!-- doc-style-disable tone -->
The installer stops at the prompt Continue?
<!-- doc-style-enable tone -->
```

Suppress rules for whole files with a `.doc-style` file. A line holds a path pattern and
one or more rule IDs. The pattern matches the path relative to the `.doc-style` file, and
`*` also matches `/`:

```text
# The glossary lists the words that the rules exclude.
docs/glossary.md   idiom qualifier filler hype
```

The checker uses the nearest `.doc-style` file between the checked file and the root of
its git repository. An unknown rule ID is an error.

Allow a word with a line of the form `allow WORD...` in the `.doc-style` file. The line
applies to every file that uses this `.doc-style` file. No rule other than `length` reports
a match whose whole text is an allowed word, ignoring case:

```text
# Let's Encrypt is the name of a certificate authority.
allow let's
```

## Tests

```bash
tests/run      # the test suite
tests/mutate   # breaks each checker rule in turn and confirms that a test fails
```

`tests/mutate` takes several minutes.

## Planned

- Packaging as a Claude Code plugin, so one install provides the skill.

## License

[MIT](LICENSE)
