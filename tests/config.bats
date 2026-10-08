#!/usr/bin/env bats
# The .doc-style project file. Each test builds a git repository in a temporary directory.

setup() {
	load test_helper
	cd "$BATS_TEST_TMPDIR" || return 1
	mkdir -p repo/.git repo/docs/sub
	cd repo || return 1
}

@test "a pattern disables the listed rules in the files it matches" {
	cat >.doc-style <<'CONFIG'
# Comment lines and blank lines are ignored.

docs/*.md   tone
CONFIG
	printf 'Is it load-bearing?\n' >docs/a.md
	printf 'Is it load-bearing?\n' >README.md
	run -1 "$CHECK" docs/a.md README.md
	[[ "$output" == *"docs/a.md:1: [idiom]"* ]]
	[[ "$output" != *"docs/a.md:1: [tone]"* ]]
	[[ "$output" == *"README.md:1: [tone]"* ]]
}

@test "the pattern matches the path relative to .doc-style, not to the working directory" {
	printf 'docs/*.md tone\n' >.doc-style
	printf 'Is it done?\n' >docs/a.md
	cd docs || return 1
	run -0 "$CHECK" a.md
	[ -z "$output" ]
}

@test "* in a pattern also matches /" {
	printf 'docs/* idiom\n' >.doc-style
	printf 'The idiom is load-bearing.\n' >docs/sub/b.md
	run -0 "$CHECK" docs/sub/b.md
	[ -z "$output" ]
}

@test "all disables every rule" {
	printf 'README.md all\n' >.doc-style
	printf 'Is it load-bearing? It is just robust.\n' >README.md
	run -0 "$CHECK" README.md
	[ -z "$output" ]
}

@test "only the nearest .doc-style applies" {
	printf '* idiom\n' >.doc-style
	printf '* tone\n' >docs/.doc-style
	printf 'Is it load-bearing?\n' >docs/a.md
	run -1 "$CHECK" docs/a.md
	[[ "$output" == *"docs/a.md:1: [idiom]"* ]]
	[[ "$output" != *"[tone]"* ]]
}

@test "a .doc-style above the git root does not apply" {
	printf '* idiom\n' >../.doc-style
	printf 'The idiom is load-bearing.\n' >README.md
	run -1 "$CHECK" README.md
	[[ "$output" == *"README.md:1: [idiom]"* ]]
}

@test "a .git file also marks the git root" {
	mkdir -p ../outer/worktree
	touch ../outer/worktree/.git
	printf '* idiom\n' >../outer/.doc-style
	printf 'The idiom is load-bearing.\n' >../outer/worktree/README.md
	run -1 "$CHECK" ../outer/worktree/README.md
	[[ "$output" == *"[idiom]"* ]]
}

@test "outside a git repository the search continues upward" {
	mkdir -p ../plain/docs
	printf '* idiom\n' >../plain/.doc-style
	printf 'The idiom is load-bearing.\n' >../plain/docs/a.md
	run -0 "$CHECK" ../plain/docs/a.md
	[ -z "$output" ]
}

@test "an inline enable comment does not enable a rule that .doc-style disables" {
	printf 'README.md idiom\n' >.doc-style
	printf '<!-- doc-style-enable idiom -->\nThe idiom is load-bearing.\n' >README.md
	run -0 "$CHECK" README.md
	[ -z "$output" ]
}

@test "allow: no pattern rule reports a word on an allow line, and other matches are reported" {
	cat >.doc-style <<'CONFIG'
# The name of the certificate authority starts with an excluded word.
allow let's
allow robust
CONFIG
	printf "The parser is robust. It requests certificates from Let's Encrypt. Let us check.\n" >README.md
	run -1 "$CHECK" README.md
	[[ "$output" == *"README.md:1: [filler] Let us |"* ]]
	[[ "$output" != *"[filler] Let's"* ]]
	[[ "$output" != *"[hype]"* ]]
}

@test "allow: the comparison ignores case" {
	printf "allow LET'S\n" >.doc-style
	printf "It requests certificates from Let's Encrypt.\n" >README.md
	run -0 "$CHECK" README.md
	[ -z "$output" ]
}

@test "allow: a word matches the whole text of a match, not a part of it" {
	printf 'allow magic robustly\n' >.doc-style
	printf 'The step works magically. The parser is robust.\n' >README.md
	run -1 "$CHECK" README.md
	[[ "$output" == *"README.md:1: [anthropomorphism] magically |"* ]]
	[[ "$output" == *"README.md:1: [hype] robust |"* ]]
}

@test "allow without a word in .doc-style: exits 2 and names the line" {
	printf 'allow\n' >.doc-style
	printf 'Text.\n' >README.md
	run -2 --separate-stderr "$CHECK" README.md
	[[ "$stderr" == *"/.doc-style:1: no word after allow" ]]
}

@test "unknown rule in .doc-style: exits 2 and names the line" {
	printf 'docs/*.md tone\nREADME.md tnoe\n' >.doc-style
	printf 'Text.\n' >README.md
	run -2 --separate-stderr "$CHECK" README.md
	[[ "$stderr" == *"/.doc-style:2: unknown rule: tnoe" ]]
}

@test "pattern without a rule in .doc-style: exits 2 and names the line" {
	printf 'README.md\n' >.doc-style
	printf 'Text.\n' >README.md
	run -2 --separate-stderr "$CHECK" README.md
	[[ "$stderr" == *"/.doc-style:1: no rule after pattern: README.md" ]]
}
