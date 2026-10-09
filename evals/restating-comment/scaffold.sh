#!/usr/bin/env bash
# claude plugin eval runs this script with the run directory as the working directory.
set -euo pipefail
cp -R "$(dirname "$0")/workspace/." .
