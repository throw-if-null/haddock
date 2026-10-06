#!/usr/bin/env bats
# Each fixture in pass/ contains text that the checker MUST NOT report. The checker MUST
# exit 0 and print nothing.

setup() {
	load test_helper
	cd "$BATS_TEST_DIRNAME/pass" || return 1
}

@test "code: front matter, fenced code, and inline code are not reported" {
	run -0 "$CHECK" code.md
	[ -z "$output" ]
}

@test "length: sentences within the limit, a bold lead-in, and short list items are not reported" {
	run -0 "$CHECK" length.md
	[ -z "$output" ]
}

@test "proper noun: Let's Encrypt is not reported" {
	run -0 "$CHECK" proper-noun.md
	[ -z "$output" ]
}

@test "structure: a heading, a table row, and a blockquote that end with ? are not reported" {
	run -0 "$CHECK" structure.md
	[ -z "$output" ]
}

@test "words: a pattern inside a longer word is not reported" {
	run -0 "$CHECK" words.md
	[ -z "$output" ]
}

@test "suppress: a region without an enable comment suppresses every finding to the end of the file" {
	run -0 "$CHECK" suppress-region.md
	[ -z "$output" ]
}

@test "suppress: a directive line ends the paragraph before it" {
	run -0 "$CHECK" suppress-paragraph.md
	[ -z "$output" ]
}
