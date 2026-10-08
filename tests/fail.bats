#!/usr/bin/env bats
# Each fixture in fail/ contains text that the checker reports. The checker MUST exit 1,
# and its output MUST match the fixture's .expected file exactly.

setup() {
	load test_helper
	cd "$BATS_TEST_DIRNAME/fail" || return 1
}

@test "idiom: every idiom in the list is reported" {
	run -1 "$CHECK" idiom.md
	assert_snapshot idiom.expected
}

@test "qualifier: every vague qualifier in the list is reported" {
	run -1 "$CHECK" qualifier.md
	assert_snapshot qualifier.expected
}

@test "filler: every filler phrase in the list is reported" {
	run -1 "$CHECK" filler.md
	assert_snapshot filler.expected
}

@test "anthropomorphism: every phrase that attributes intent to software is reported" {
	run -1 "$CHECK" anthropomorphism.md
	assert_snapshot anthropomorphism.expected
}

@test "hype: every promotional word in the list is reported" {
	run -1 "$CHECK" hype.md
	assert_snapshot hype.expected
}

@test "tone: exclamations, questions, and informal words are reported" {
	run -1 "$CHECK" tone.md
	assert_snapshot tone.expected
}

@test "chain: an em dash, a double hyphen, a semicolon, and a trailing , which are reported" {
	run -1 "$CHECK" chain.md
	assert_snapshot chain.expected
}

@test "length: a sentence over the limit is reported" {
	run -1 "$CHECK" length.md
	assert_snapshot length.expected
}

@test "length: --max-words sets the limit" {
	run -1 "$CHECK" --max-words 10 length-max-words.md
	assert_snapshot length-max-words.expected
}

@test "code: the text after a code block or an HTML comment is reported on its own line" {
	run -1 "$CHECK" code-end.md
	assert_snapshot code-end.expected
}

@test "source code: findings in Python comments and docstrings are reported" {
	run -1 "$CHECK" comments.py
	assert_snapshot comments.py.expected
}

@test "source code: findings in Go line comments and block comments are reported" {
	run -1 "$CHECK" comments.go
	assert_snapshot comments.go.expected
}

@test "source code: a task marker line in a shell script is exempt from the tone rule only" {
	run -1 "$CHECK" comments.sh
	assert_snapshot comments.sh.expected
}

@test "source code: findings in C line comments and block comments are reported, and a directive is not" {
	run -1 "$CHECK" comments.c
	assert_snapshot comments.c.expected
}

@test "source code: a # comment after code is checked, and the string before it is not" {
	run -1 "$CHECK" comments-trailing.py
	assert_snapshot comments-trailing.py.expected
}

@test "source code: a // comment after code is checked" {
	run -1 "$CHECK" comments-trailing.go
	assert_snapshot comments-trailing.go.expected
}

@test "source code: a line that starts with -- is a comment" {
	run -1 "$CHECK" comments.sql
	assert_snapshot comments.sql.expected
}

@test "source code: a line that starts with ; is a comment" {
	run -1 "$CHECK" comments.ini
	assert_snapshot comments.ini.expected
}

@test "suppress: a disable comment suppresses the listed rules until the enable comment" {
	run -1 "$CHECK" suppress-inline.md
	assert_snapshot suppress-inline.expected
}

@test "suppress: a directive inside code is not a directive" {
	run -1 "$CHECK" suppress-code.md
	assert_snapshot suppress-code.expected
}
