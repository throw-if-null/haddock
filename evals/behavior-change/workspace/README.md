# inventory-client

A Python client for the inventory service.

## Usage

```python
import client

response = client.get("https://inventory.internal/items/42")
```

## Retries

`client.get` retries a failed request up to three times. It waits one second before each
retry.

## Timeouts

Each request times out after 10 seconds.
