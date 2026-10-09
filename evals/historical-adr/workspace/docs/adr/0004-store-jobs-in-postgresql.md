# 4. Store jobs in PostgreSQL

Status: Accepted

## Context

Production runs one PostgreSQL instance. Redis is not available in production, and adding
it requires a new on-call rotation.

## Decision

Store jobs in the `jobs` table. Workers claim a job with `SELECT ... FOR UPDATE SKIP LOCKED`.

## Consequences

The job queue needs no new infrastructure. Each claim holds a row lock until the job
finishes.
