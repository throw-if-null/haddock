#!/usr/bin/env bats
# Checks of the repository content as a whole.

setup() {
	cd "$BATS_TEST_DIRNAME/.." || return 1
}

@test "repo: no tracked file mentions Codex" {
	# The skill targets Claude Code only. A Codex mention is an unmaintained claim.
	run git grep -i -n -E 'codex|agents\.md|apply_patch' -- . ':!tests/repo.bats'
	[ "$status" -eq 1 ]
	[ -z "$output" ]
}
