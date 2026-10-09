# Maintenance examples

Each example gives a situation, the text before and after the change, and the reason for
the decision.

## 1. A refactor makes a comment obsolete

A refactor moves token caching from the request handler into `AuthClient`.

Before, in `handler.ts`:

```ts
// Cache the token for 5 minutes. A permission change must take effect within
// 5 minutes (SEC-31), and a cached token keeps its old permissions until it expires.
const token = await tokenCache.get(userId, () => fetchToken(userId), { ttl: 300 });
```

After, in `handler.ts`:

```ts
const token = await auth.token(userId);
```

After, in `auth-client.ts`:

```ts
// A permission change must take effect within 5 minutes (SEC-31). A cached
// token keeps its old permissions until the TTL expires.
const TOKEN_TTL_SECONDS = 300;
```

The handler no longer sets the TTL, so the comment no longer describes the code next to it.
The constraint still limits the TTL, so it moves to the constant that sets the TTL. "Cache
the token for 5 minutes" restated the code, so it does not move.

## 2. A feature changes documented behavior

The client changes from a fixed retry delay to exponential backoff.

Before, in `README.md`:

```markdown
## Retries

The client retries a failed request up to three times. It waits one second before each retry.
```

An appended correction. Do not write this:

```markdown
## Retries

The client retries a failed request up to three times. It waits one second before each retry.

**Update:** Retries now use exponential backoff, and the number of retries was increased
to five.
```

After:

```markdown
## Retries

The client retries a failed request up to five times. It waits one second before the first
retry and doubles the wait before each later retry. Set `retry.max_retries` to change the
number of retries.
```

The appended version keeps a false sentence and records history. The rewritten paragraph
describes the current behavior. The commit message and the changelog record the change.

## 3. Two documents state the same fact

A change sets the default port from 8080 to 8000. Two documents state the port rules.

Before, in `README.md`:

```markdown
## Configuration

The server reads its port from `APP_PORT`. When `APP_PORT` is not set, it uses `port`
from `config.yaml`, and then 8080.
```

Before, in `docs/configuration.md`:

```markdown
## Port

Set the port with the `APP_PORT` environment variable or the `port` key in `config.yaml`.
`APP_PORT` takes precedence. The default is 8080.
```

After, in `README.md`:

```markdown
## Configuration

[docs/configuration.md](docs/configuration.md) describes each setting, including the port.
```

After, in `docs/configuration.md`:

```markdown
## Port

Set the port with the `APP_PORT` environment variable or the `port` key in `config.yaml`.
`APP_PORT` takes precedence. The default is 8000.
```

Both passages state the same precedence rule and the same default. With two copies, each
change needs two edits, and a missed edit leaves one copy wrong. The configuration page
owns the subject, so it keeps the fact, and the README links to it. Before you remove a
copy, confirm that the copies agree. When they disagree, the code decides which statement
is correct.

## 4. A comment restates the code

A change adds an attempt limit. A draft with comments. Do not write this:

```go
// Increment the attempt counter.
attempts++
// Stop when the maximum number of attempts is reached.
if attempts >= maxAttempts {
	return ErrTooManyAttempts
}
```

After:

```go
attempts++
if attempts >= maxAttempts {
	return ErrTooManyAttempts
}
```

Each comment repeats the line below it. The names `attempts`, `maxAttempts`, and
`ErrTooManyAttempts` state the behavior. This comment gives a fact that the code does not
show, so it is useful:

```go
// maxAttempts includes the first request: 3 allows 2 retries.
```

## 5. A compatibility constraint survives a request to shorten

The task is "Shorten the comments in `timestamps.py`."

Before:

```python
# Note that we have to be careful here: we really need to use the "Z" suffix
# instead of "+00:00", because the billing exporter parses this field with the
# fixed format "%Y-%m-%dT%H:%M:%SZ" and it will reject any other offset
# notation. See BILL-412 for the details.
return dt.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
```

Too short. Do not write this:

```python
# Format as UTC.
return dt.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
```

After:

```python
# Use "Z", not "+00:00". The billing exporter parses this field with the fixed
# format "%Y-%m-%dT%H:%M:%SZ" and rejects other offsets (BILL-412).
return dt.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
```

`dt.isoformat()` looks like a simpler equivalent, but it writes `+00:00`. The comment is the
only protection against that change. The short version keeps the constraint, the consumer,
and the reference. It removes only the words that carry no information.

## 6. The code changes and the documentation stays correct

A change replaces a linear search with a dictionary lookup.

Before:

```python
def find(self, user_id):
    """Return the user with the given ID, or None if no user has that ID."""
    return next((user for user in self._users if user.id == user_id), None)
```

After:

```python
def find(self, user_id):
    """Return the user with the given ID, or None if no user has that ID."""
    return self._by_id.get(user_id)
```

An added implementation note. Do not write this:

```python
    """Return the user with the given ID, or None if no user has that ID.

    Uses a dict index for O(1) lookup instead of scanning the list.
    """
```

The docstring states the contract, and the contract did not change. The lookup method is
an implementation detail, and "instead of scanning the list" is history. "Runs in constant
time" is also wrong here: it adds a guarantee that the project does not document for any
other method. The correct documentation change is none.

## 7. An ADR keeps its history

The job queue moves from PostgreSQL to Redis. ADR 4 recorded the original decision.

Before, in `docs/adr/0004-store-jobs-in-postgresql.md`:

```markdown
# 4. Store jobs in PostgreSQL

Status: Accepted

## Context

Production runs one PostgreSQL instance. Redis is not available in production, and adding
it requires a new on-call rotation.

## Decision

Store jobs in the `jobs` table. Workers claim a job with `SELECT ... FOR UPDATE SKIP LOCKED`.
```

A rewrite as current state. Do not write this:

```markdown
## Context

Production runs PostgreSQL and Redis.

## Decision

Store jobs in a Redis list.
```

After: only the status line changes. The Context and Decision sections stay as they are.

```markdown
Status: Superseded by [ADR 9](0009-store-jobs-in-redis.md)
```

An ADR records a decision and the context at the time of the decision. A reader needs that
context to understand why the system used PostgreSQL. ADR 9 records the new decision. The
current-state documents, such as `docs/architecture.md`, describe the Redis queue without
narration.

## 8. Narration that contains a current constraint

A change adds a retry to each batch request. The loop has this comment.

Before:

```python
# Originally this called the inventory API once per item, which was too slow,
# so in March we switched to batches of 500. After the June outage we reduced
# the batch size to 100, because the API times out on batches larger than 100.
for batch in chunks(item_ids, 100):
    api.update_stock(batch)
```

After:

```python
# The inventory API times out on batches larger than 100 IDs.
for batch in chunks(item_ids, 100):
    with_retry(api.update_stock, batch)
```

The comment is on the changed code, so the change includes it. The timeout limit is a
current constraint, so it stays. The sequence of earlier batch sizes is history, so it
goes. When narration holds no current reason, delete it. Do not invent a reason.
