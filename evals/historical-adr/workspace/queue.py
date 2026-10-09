"""Job queue backed by a Redis list."""

import json

import redis

QUEUE_KEY = "jobs:pending"


class JobQueue:
    def __init__(self, url):
        self._redis = redis.Redis.from_url(url)

    def push(self, job):
        """Append `job` to the end of the queue."""
        self._redis.rpush(QUEUE_KEY, json.dumps(job))

    def claim(self, timeout=5):
        """Remove and return the job at the front of the queue.

        Return None when no job arrives within `timeout` seconds.
        """
        item = self._redis.blpop(QUEUE_KEY, timeout=timeout)
        return None if item is None else json.loads(item[1])
