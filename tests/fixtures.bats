#!/usr/bin/env bats
# Checks of the test suite itself. bats does not discover fixture files, so a fixture
# without a test is never run. These tests detect that.

setup() {
	cd "$BATS_TEST_DIRNAME" || return 1
}

@test "every fixture is run by a test" {
	local fixture missing=0
	for fixture in pass/*.md fail/*.md; do
		if ! grep -q -F " ${fixture#*/}" "${fixture%%/*}.bats"; then
			echo "$fixture: no test in ${fixture%%/*}.bats runs it"
			missing=1
		fi
	done
	((missing == 0))
}

@test "every fail fixture states its rule on line 1" {
	local fixture missing=0
	for fixture in fail/*.md; do
		if ! head -n 1 "$fixture" | grep -q '^<!-- Rule: .* -->$'; then
			echo "$fixture: line 1 is not a <!-- Rule: ... --> comment"
			missing=1
		fi
	done
	((missing == 0))
}
