# Operations

## Upload retention

The cleanup job deletes an upload when its last modification is more than 30 days old.
The `UPLOAD_RETENTION_DAYS` environment variable sets the number of days.

## Backups

The backup job copies the `uploads` directory to the backup bucket every night at
02:00 UTC. It keeps the last seven backups.
