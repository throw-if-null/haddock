<!-- Rule: in source code, a # or // comment after code is checked when whitespace precedes the marker and the quotes before it are balanced. -->
import os

LIMIT = os.environ.get("LIMIT", "3")  # The default is just an estimate.
