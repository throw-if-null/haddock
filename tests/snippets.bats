#!/usr/bin/env bats
# The primer snippets in snippets/. Each snippet MUST have 20 lines or fewer, and the
# checker MUST report nothing on it.

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
