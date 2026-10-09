# Rewrite examples

Each entry shows sentence rewrites for one part of the [writing style](../SKILL.md#writing-style).
A word in a `Before` line is not wrong in every sentence. Judge each word in context.

## 1. Idiom and metaphor

Replace the figure of speech with the behavior it describes.

```text
Before: The retry limit in `worker.yaml` is load-bearing.
After:  The retry limit in `worker.yaml` stops one failing job from blocking the queue.

Before: The second checksum is belt and braces.
After:  The second checksum detects corruption after the upload. The first checksum does
        not cover that interval.

Before: Under the hood, the CLI talks to the daemon over a socket.
After:  The CLI sends each command to the daemon over a socket.
```

## 2. Introductions and rhetorical framing

Start with the subject. A heading or an opening sentence names its content. It does not
predict the reader's reaction.

```text
Before: In this section, we will take a look at how the cache works. The cache stores ...
After:  The cache stores ...

Before: Two consequences that surprise people:
After:  Two consequences follow:

Before: The `smoke` profile deliberately starts no workers, so the queue stays empty. That
        is the point.
After:  The `smoke` profile starts no workers, so the queue stays empty. This tests the API
        and the database connection in isolation.
```

## 3. Filler and qualifiers

Remove a word when the sentence means the same without it. Keep a qualifier that states a
real condition.

```text
Before: Note that you just need to run `make install`.
After:  Run `make install`.

Keep:   A read usually returns within 5 ms. A cache miss queries the database and can
        take up to 200 ms.
```

In the second entry, "usually" states that the first sentence has exceptions, and the next
sentence names them.

## 4. Promotional words

Replace the claim with the property that supports it. When no property supports it, delete
the claim.

```text
Before: The scheduler provides seamless, robust job execution.
After:  The scheduler retries a failed job up to three times, and resumes pending jobs
        after a restart.
```

## 5. Attributed intent

Describe the mechanism.

```text
Before: The scheduler knows when a worker dies and wants to move its jobs.
After:  When a worker misses its heartbeat, the scheduler marks it as dead and reassigns
        its jobs.

Before: A typo in a path cannot quietly create a new bucket beside the real one.
After:  A typo in a path cannot create a new bucket beside the real one.
```

## 6. Requirement strength

A requirement takes MUST, MUST NOT, SHOULD, or MAY. A description takes the plain
indicative.

```text
Before: Both scripts are run by `db_admin`, and the point is that the application role
        cannot run either.
After:  Both scripts MUST be run by `db_admin`. The application role MUST NOT be able to
        run either script.

Before: It is worth narrowing an API key to the endpoints its client calls.
After:  An API key SHOULD be narrowed to the endpoints that its client calls.

Before: The build MUST copy the lock file to the output directory.
        (The sentence describes what the build does.)
After:  The build copies the lock file to the output directory.
```

## 7. Prose that is clearer as a list

Independent statements in one long sentence become a list.

```text
Before: `build.sh` derives the image tag from the branch name, so the same branch in two
        repositories takes the same tag in different registries, and `deploy.sh` reads
        the service directories under an environment to decide which images it needs,
        which is why an environment that does not list a service never gets its image.
After:  Two mechanisms depend on the distinction:

        - `build.sh` derives the image tag from the branch name. The same branch in two
          repositories therefore takes the same tag, in different registries.
        - `deploy.sh` reads the service directories under an environment to determine
          which images that environment requires. An environment that does not list a
          service does not receive its image.
```
