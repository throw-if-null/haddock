# Shared setup for the checker tests. Each .bats file loads it in setup().

bats_require_minimum_version 1.7.0

# CHECK selects the checker under test. The default is the checker in this repository.
CHECK="${CHECK:-$BATS_TEST_DIRNAME/../skills/doc-style/scripts/check}"

# assert_snapshot FILE compares the output of the last run with FILE and prints the
# difference.
assert_snapshot() {
	# shellcheck disable=SC2154 # bats run sets output.
	diff -u "$1" <(printf '%s\n' "$output")
}
