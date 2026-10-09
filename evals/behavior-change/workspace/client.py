"""HTTP client for the inventory service."""

import time

import requests

MAX_RETRIES = 3
RETRY_DELAY_SECONDS = 1.0


def get(url):
    """GET `url` and return the response. Retry a failed request up to MAX_RETRIES times."""
    for attempt in range(MAX_RETRIES + 1):
        try:
            response = requests.get(url, timeout=10)
            response.raise_for_status()
            return response
        except requests.RequestException:
            if attempt == MAX_RETRIES:
                raise
            time.sleep(RETRY_DELAY_SECONDS)
