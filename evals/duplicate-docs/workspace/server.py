"""HTTP server entry point."""

import os

import yaml

# Used when neither APP_PORT nor the `port` key in config.yaml sets a port.
DEFAULT_PORT = 8080


def resolve_port(config_path="config.yaml"):
    """Return the port from APP_PORT, then from config.yaml, then DEFAULT_PORT."""
    if "APP_PORT" in os.environ:
        return int(os.environ["APP_PORT"])
    try:
        with open(config_path) as config_file:
            config = yaml.safe_load(config_file) or {}
    except FileNotFoundError:
        config = {}
    return int(config.get("port", DEFAULT_PORT))
