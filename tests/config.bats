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
