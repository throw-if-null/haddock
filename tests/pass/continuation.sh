#!/usr/bin/env bash
# Run the idiom tests. A continuation line that starts with a double hyphen is read as a
# comment, so the wrapped options below are checked as comment text.
set -euo pipefail

bats \
	--filter idiom \
	--print-output-on-failure \
	tests
