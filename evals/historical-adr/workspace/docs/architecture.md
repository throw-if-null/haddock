# Architecture

## Components

- `api` accepts jobs over HTTP and pushes them to the job queue.
- `worker` claims jobs from the job queue and runs them.

## Job queue

Jobs are stored in the PostgreSQL `jobs` table. A worker claims a job with
`SELECT ... FOR UPDATE SKIP LOCKED`, so two workers never claim the same job. See
[ADR 4](adr/0004-store-jobs-in-postgresql.md).
