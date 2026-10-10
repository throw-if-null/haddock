# haddock

A Claude Code plugin that holds the `doc-maintenance` skill. The skill makes Claude
maintain documentation and code comments as an accurate, concise description of the
current system. Documentation is maintained, not accumulated.

## What the skill does

When Claude changes code, the skill directs it to:

- Find the comments, docstrings, and documents that describe the changed code.
- Update or delete the prose that the change made wrong or redundant.
- Edit, consolidate, or delete existing prose before it adds new prose.
- Describe the current system, and leave the change history to commit messages and
  changelogs.
- Keep the constraints, invariants, and design rationale that are still true.
- Leave correct documentation unchanged, and leave documentation that the change does not
  affect unchanged.

The skill also sets a writing style for documentation, comments, commit messages, and pull
request descriptions: direct, literal, and concise. The style draws on ASD-STE100,
Simplified Technical English. It does not implement that standard.

| Path | Content |
| --- | --- |
| `skills/doc-maintenance/SKILL.md` | The workflow and the rules. |
| `skills/doc-maintenance/examples/maintenance.md` | Maintenance decisions, with the text before and after. |
| `skills/doc-maintenance/examples/rewrites.md` | Sentence rewrites for the writing style. |
| `snippets/CLAUDE.md` | The primer: a summary that directs Claude to apply the skill to every code change. |
| `hooks/hooks.json` | The `SessionStart` hook that prints the primer into the session context. |
| `evals/` | The eval cases for `claude plugin eval`. |

`.claude-plugin/` holds the plugin manifest and the marketplace file.

## Install as a plugin

The repository is a Claude Code plugin and its own marketplace. Add the marketplace once,
then install the plugin:

```text
/plugin marketplace add throw-if-null/haddock
/plugin install haddock@throw-if-null
```

Claude Code loads the skill when a task matches its description, and
`/haddock:doc-maintenance` loads it on demand. A task that asks only for a code change does
not reliably match the description. The plugin's `SessionStart` hook therefore prints the
primer into the context of each session, including a session that resumes or compacts. The
primer directs Claude to apply the skill to every code change.

To try the plugin without installing it, start a session with
`claude --plugin-dir /path/to/haddock`.

## Install by hand

Clone the repository, then link the skill into the directory that Claude Code scans for
personal skills:

```bash
git clone https://github.com/throw-if-null/haddock.git ~/.local/share/haddock
ln -s ~/.local/share/haddock/skills/doc-maintenance ~/.claude/skills/doc-maintenance
```

`/doc-maintenance` then loads the skill on demand, and `/skills` lists it.

To install the skill for one project instead, link it into the project's `.claude/skills/`
directory.

The install by hand does not install the hook. Append the primer to the `CLAUDE.md` of each
project that uses the skill instead:

```bash
cat ~/.local/share/haddock/snippets/CLAUDE.md >> CLAUDE.md
```

## Evaluate the skill

Each directory in `evals/` holds one case: a small workspace, a coding or documentation
task, and graders. The `description` in each `case.yaml` states the behavior that the case
tests. The graders check the files that Claude leaves in the workspace.

```bash
claude plugin eval . --scaffold --allow-tools Edit Write --judge-model sonnet
```

- `--scaffold` runs the `scaffold.sh` of each case. The script copies the case's
  `workspace/` directory into the run directory. It runs as you, outside the sandbox.
- `--allow-tools Edit Write` grants the file edits that the cases request. Without the
  grant, Claude cannot change the workspace. The cases do not request Bash.
- `--judge-model sonnet` sets the model of the LLM graders. The default judge, Haiku,
  answers without extended thinking and fails correct code in the `retry-implemented`
  grader.
- Each run starts a Claude session with your credentials. By default, each case runs 3
  times with the plugin and 3 times without it. `--runs`, `--case`, and `--max-cost-usd`
  limit the cost.
- The `skill-loaded` grader records whether Claude loaded the skill. In a run with the
  no-plugin baseline, it is an indicator and does not count toward the score.
- `--no-publish` keeps the HTML report local.
- The results go to `evals/results/`, which git ignores.

## Validate the plugin

```bash
claude plugin validate .
```

## License

[MIT](LICENSE)
