# upload-service

Stores user uploads and serves them by ID.

## Clean up

`python cleanup.py` deletes the expired uploads. A cron job runs it every hour.

## Retention

An upload expires 30 days after its last modification. Set `UPLOAD_RETENTION_DAYS` to
change the number of days.
