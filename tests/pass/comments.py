# Only comments and docstrings are checked in a source file.
import os


def just_magic(robust=True):
    """Return the retry limit from the environment.

    The value is an integer.
    """
    very = os.environ.get("LIMIT", "load-bearing — really")  # Is this just magic?
    return very


# Does the worker read the limit at start?
# TODO: read the limit again on SIGHUP!
# FIXME: reject a negative limit!
# XXX: the default limit is an estimate!
# NOTE: the limit applies to each worker!
