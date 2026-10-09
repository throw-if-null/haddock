# orders-api

The HTTP API for orders.

## Run

```bash
python server.py
```

## Configuration

The server reads its port from `APP_PORT`. When `APP_PORT` is not set, it uses `port`
from `config.yaml`, and then 8080.

[docs/configuration.md](docs/configuration.md) describes each setting.
