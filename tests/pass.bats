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

@test "code: a tilde fence is code, and a backtick fence line does not close it" {
	run -0 "$CHECK" code-tilde.md
	[ -z "$output" ]
}

@test "code: a fence closes only at a fence line of the same length or longer, without an info string" {
	run -0 "$CHECK" code-long-fence.md
	[ -z "$output" ]
}

@test "code: a line indented by four spaces or a tab after a blank line is code" {
	run -0 "$CHECK" code-indented.md
	[ -z "$output" ]
}

@test "code: an HTML comment on one line or on several lines is not reported" {
	run -0 "$CHECK" html-comment.md
	[ -z "$output" ]
}

@test "length: sentences within the limit, a bold lead-in, and short list items are not reported" {
	run -0 "$CHECK" length.md
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

@test "compound: a pattern inside a hyphenated compound is not reported" {
	run -0 "$CHECK" compound.md
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
