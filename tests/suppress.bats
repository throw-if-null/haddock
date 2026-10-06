#!/usr/bin/env bats
# Invalid inline directives. Each test writes its input to a temporary git repository.

setup() {
	load test_helper
	cd "$BATS_TEST_TMPDIR" || return 1
	mkdir .git
}

@test "unknown rule in a directive: exits 2 and names the rule" {
	printf '<!-- doc-style-disable tnoe -->\nThe idiom is load-bearing.\n' >doc.md
	run -2 --separate-stderr "$CHECK" doc.md
	[ "$stderr" = "doc.md:1: unknown rule: tnoe" ]
	[ -z "$output" ]
}

@test "misspelled directive: exits 2 and names the directive" {
	printf '<!-- doc-style-disabel tone -->\n' >doc.md
	run -2 --separate-stderr "$CHECK" doc.md
	[ "$stderr" = "doc.md:1: unknown directive: doc-style-disabel" ]
}

@test "directive without -->: exits 2" {
	printf '<!-- doc-style-disable tone\n' >doc.md
	run -2 --separate-stderr "$CHECK" doc.md
	[ "$stderr" = "doc.md:1: directive does not end with -->" ]
}

@test "invalid directive in one file: the other files are still checked" {
	printf '<!-- doc-style-disable tnoe -->\n' >bad.md
	printf 'The idiom is load-bearing.\n' >good.md
	run -2 --separate-stderr "$CHECK" bad.md good.md
	[[ "$output" == *"good.md:1: [idiom] load-bearing"* ]]
}
