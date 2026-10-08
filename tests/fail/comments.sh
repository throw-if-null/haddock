<!-- Rule: in source code, inline code in a comment is not checked, and a task marker line is exempt from the tone rule only. -->
#!/usr/bin/env bash
# Run `just build` before the tests. The build step is load-bearing.
# TODO: the cache is just a workaround!
set -eu
