#!/usr/bin/env bats
# Checks of the repository content as a whole.

setup() {
	load test_helper
	cd "$BATS_TEST_DIRNAME/.." || return 1
}

# list_words ITEM prints each code span of the rules.md list item that starts with
# "- ITEM", one per line. The item ends at the next top-level list item or blank line.
list_words() {
	awk -v item="- $1" '
		on && (/^- / || /^$/) { exit }
		index($0, item) == 1 { on = 1 }
		on {
			rest = $0
			while (match(rest, /`[^`]+`/)) {
				print substr(rest, RSTART + 1, RLENGTH - 2)
				rest = substr(rest, RSTART + RLENGTH)
			}
		}
	' skills/doc-style/rules.md
}

# assert_listed_words_reported RULE ITEM writes each word of the rules.md list ITEM on its
# own line and runs the checker. Each line MUST have a finding of RULE.
assert_listed_words_reported() {
	local rule="$1" file="$BATS_TEST_TMPDIR/$1.md" report word n=0 missing=0
	list_words "$2" >"$file"
	if [ ! -s "$file" ]; then
		echo "rules.md: no code span in the list item that starts with: - $2"
		return 1
	fi
	report="$("$CHECK" "$file")" || true
	while IFS= read -r word; do
		n=$((n + 1))
		if [[ "$report" != *"$file:$n: [$rule] "* ]]; then
			echo "rules.md lists '$word' under '$2', and the checker has no $rule alternative for it"
			missing=1
		fi
	done <"$file"
	((missing == 0))
}

@test "repo: the checker reports every word in the rules.md qualifier and filler lists" {
	mkdir "$BATS_TEST_TMPDIR/.git"
	assert_listed_words_reported qualifier "Remove vague qualifiers"
	assert_listed_words_reported filler "Remove filler openers"
}

@test "repo: no tracked file mentions Codex" {
	# The skill targets Claude Code only. A Codex mention is an unmaintained claim.
	run git grep -i -n -E 'codex|agents\.md|apply_patch' -- . ':!tests/repo.bats'
	[ "$status" -eq 1 ]
	[ -z "$output" ]
}
