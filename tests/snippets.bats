#!/usr/bin/env bats
# The primer snippets in snippets/. Each snippet MUST have 20 lines or fewer, MUST NOT
# name a path under ~/.claude/, and the checker MUST report nothing on it.

setup() {
	load test_helper
	cd "$BATS_TEST_DIRNAME/../snippets" || return 1
}

@test "snippets: each snippet has 20 lines or fewer" {
	local snippet lines failed=0
	for snippet in *.md; do
		lines="$(wc -l <"$snippet")"
		if ((lines > 20)); then
			echo "$snippet: $lines lines"
			failed=1
		fi
	done
	((failed == 0))
}

@test "snippets: the checker reports nothing" {
	run -0 "$CHECK" ./*.md
	[ -z "$output" ]
}

@test "snippets: no snippet names a path under ~/.claude/" {
	# The plugin install does not use that directory. The skill directory and the plugin
	# command locate the checker for both installs.
	# shellcheck disable=SC2088 # The tilde is the searched text, not a path.
	run -1 grep -n -F '~/.claude/' ./*.md
	[ -z "$output" ]
}
