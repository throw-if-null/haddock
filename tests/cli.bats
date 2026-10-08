#!/usr/bin/env bats
# Command-line handling of the checker: usage errors, missing files, and exit codes.

setup() {
	load test_helper
	cd "$BATS_TEST_DIRNAME/fail" || return 1
}

@test "no arguments: exits 2 and prints usage" {
	run -2 --separate-stderr "$CHECK"
	[[ "$stderr" == usage:* ]]
}

@test "unknown option: exits 2 and names the option" {
	run -2 --separate-stderr "$CHECK" --nope chain.md
	[[ "$stderr" == *"unknown option: --nope"* ]]
}

@test "non-numeric --max-words: exits 2 and prints usage" {
	run -2 --separate-stderr "$CHECK" --max-words x chain.md
	[[ "$stderr" == usage:* ]]
}

@test "--comments: a Markdown file is read as source code" {
	cd "$BATS_TEST_TMPDIR" || return 1
	mkdir .git
	printf '# The heading is robust.\n\nThe idiom is load-bearing.\n' >doc.md
	run -1 "$CHECK" --comments doc.md
	[[ "$output" == "doc.md:1: [hype] robust | # The heading is robust."* ]]
	[[ "$output" != *"load-bearing"* ]]
}

@test "a .markdown file is read as Markdown" {
	cd "$BATS_TEST_TMPDIR" || return 1
	mkdir .git
	printf 'The idiom is load-bearing.\n' >doc.markdown
	run -1 "$CHECK" doc.markdown
	[[ "$output" == "doc.markdown:1: [idiom] load-bearing"* ]]
}

@test "missing file: exits 2 and names the file" {
	run -2 --separate-stderr "$CHECK" missing.md
	[[ "$stderr" == *"no such file: missing.md"* ]]
}

@test "missing file with another file: exits 2 and still reports the other file" {
	run -2 --separate-stderr "$CHECK" missing.md chain.md
	[[ "$stderr" == *"no such file: missing.md"* ]]
	[[ "$output" == *"chain.md:3: [chain]"* ]]
}
