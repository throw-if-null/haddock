#!/usr/bin/env bash
# Print each test file. Only comments are checked in a source file.
set -euo pipefail

for f in tests/*.bats; do
	echo "just magic — really robust; feel free!"
	ls /tmp/* "$f" # Is the file listed?
done

case "${1:-}" in
--max-words)
	echo "Is it done?"
	;;
esac

# Does the loop stop at the first error?
# TODO: stop at the first error!
