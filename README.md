# haddock

Claude Code skills for technical writing. The first skill, `doc-style`, writes and checks
documentation in controlled technical English.

The rules are based on ASD-STE100, Simplified Technical English. They are opinionated by
design: they fix one sentence length, one set of requirement keywords, and one word for
each concept. [deviations.md](skills/doc-style/deviations.md) lists where the rules differ
from the standard, and why.

## Three layers

Each layer has one job.

| Layer | Files | Job |
| --- | --- | --- |
| Primer | `snippets/CLAUDE.md` | Names the skill in a project's `CLAUDE.md`, so Claude loads it before it writes. |
| Skill | `skills/doc-style/` | Holds the full rules, the procedure, and the examples. This is the only place the rules are stated. |
| Checker and hook | `skills/doc-style/scripts/check`, `hooks/` | Reports the constructions the rules exclude, on demand and after each edit. |

## Requirements

- Linux. The checker needs Bash 4 or later and GNU grep.
- `jq`, for the hook.
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

The primer names the skill, summarizes the rules, and tells Claude to run the checker. It
does not restate the rules.

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

## Enable the hook

The hook runs the checker after Claude Code edits a Markdown file. Claude reads the
findings as additional context, and the transcript shows them to you.

`hooks/claude-code.json` holds the hook in the shape of the `hooks` object in
`~/.claude/settings.json`. Copy its `hooks` entry into your settings:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "\"$HOME/.local/share/haddock/hooks/check-markdown\"",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

The path MUST point at your clone. The hook needs `jq`.

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

Suppress rules for whole files with a `.doc-style` file. Each line is a path pattern and
one or more rule IDs. The pattern matches the path relative to the `.doc-style` file, and
`*` also matches `/`:

```text
# The glossary lists the words that the rules exclude.
docs/glossary.md   idiom qualifier filler hype
```

The checker uses the nearest `.doc-style` file between the checked file and the root of
its git repository. An unknown rule ID is an error.

## Tests

```bash
tests/run      # the test suite
tests/mutate   # breaks each checker rule in turn and confirms that a test fails
```

`tests/mutate` takes several minutes.

## Planned

- Packaging as a Claude Code plugin, so one install provides the skill and the hook.

## License

[MIT](LICENSE)
