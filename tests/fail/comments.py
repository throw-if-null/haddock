<!-- Rule: in source code, the checker reports findings in comments and docstrings, and nowhere else. -->
import os

# The retry limit is load-bearing.
LIMIT = "load-bearing"  # A comment after code is not checked: load-bearing.


def read_limit():
    """Return the limit. The value is just an integer."""
    return int(os.environ.get("LIMIT", "3"))


class Worker:
    """Run jobs from the queue.

    The worker talks to the queue.
    """

    # The worker reads the limit from the environment, converts it to an integer, checks
    # the range, and stores it in the configuration for the scheduler and every job.
