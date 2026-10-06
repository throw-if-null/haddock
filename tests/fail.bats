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

@test "chain: an em dash, a double hyphen, and a semicolon are reported" {
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
