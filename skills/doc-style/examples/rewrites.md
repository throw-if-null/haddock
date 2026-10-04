# Rewrite catalogue

Each entry is one defect class. It states the rule, then gives before and after pairs.

Read this file when rewriting an existing document, or when a checker hit has no obvious
literal replacement.

The `Before` lines contain the constructions this skill excludes. A checker run over this
file reports them. That is expected.

## 1. Idiom and metaphor

Replace the figure of speech with the behaviour it described, and with the failure it
prevents.

```text
Before: The retry limit in `worker.yaml` is load-bearing.
After:  The retry limit in `worker.yaml` stops one failing job from blocking the queue.

Before: The second checksum is belt and braces.
After:  The second checksum detects corruption after the upload. The first checksum does
        not cover that interval.

Before: Two details in that script are not decoration.
After:  Two details in the script above are required, not stylistic.

Before: The distinction has teeth in two places.
After:  Two mechanisms depend on the distinction.

Before: It also sets the blast radius of a leaked API key, which is account-wide.
After:  It also defines the exposure of a leaked API key, which is account-wide.
```

## 2. Rhetorical framing

A heading or an opener names its content. It does not predict the reader's reaction.

```text
Before: Two consequences that surprise people:
After:  Two consequences follow:

Before: The `smoke` profile deliberately starts no workers, so the queue stays empty. That
        is the point.
After:  The `smoke` profile starts no workers, so the queue stays empty. This tests the API
        and the database connection in isolation.

Before: Reading docs/architecture.md before changing anything here is worth the five
        minutes.
After:  Read docs/architecture.md before changing anything here.
```

## 3. Clause chain

Split a chain joined by an em dash or a semicolon. Use a colon to introduce a list.

```text
Before: The controls that actually matter are on the identity provider — single sign-on,
        multi-factor authentication, group membership, session lifetime — not on the
        network path.
After:  The effective controls are on the identity provider, not on the network path:
        single sign-on, multi-factor authentication, group membership, and session
        lifetime.

Before: ... in a single transaction, and running statements individually defeats the
        guards — the version check in `migrate.sh`, and the lock check in the SQL.
After:  ... in a single transaction. Running statements individually defeats two guards:
        the version check in `migrate.sh`, and the lock check in the SQL.

Before: ... with any region suffix removed — `dev`, `prod-eu1`.
After:  ... with any region suffix removed: `dev`, `prod-eu1`.
```

## 4. Requirement strength

A requirement takes MUST, MUST NOT, SHOULD, or MAY. A description takes the plain
indicative.

```text
Before: Both scripts are run by `db_admin`. Neither is ever run by the application role.
        That role is the subject of the grants, not their author, and it is the point that
        it cannot run either script.
After:  Both scripts MUST be run by `db_admin`. Neither is run by the application role.
        The application role is the subject of the grants, not their author. It MUST NOT
        be able to run either script.

Before: The same reasoning makes `.gitignore`'s exclusion of build artifacts load-bearing.
After:  `.gitignore` MUST continue to exclude build artifacts for the same reason.

Before: Nothing else belongs in that vault that CI has no use for.
After:  An environment vault MUST NOT hold anything CI has no use for.
```

## 5. Vague qualifier and hedged recommendation

State the recommendation with SHOULD. State the current fact separately.

```text
Before: ... narrowing an API key to the endpoints its client actually calls is worth doing
        where the provider allows it — but the keys in use today are account-scoped, so
        state the blast radius accurately rather than aspirationally.
After:  ... an API key SHOULD be narrowed to the endpoints its client calls where the
        provider allows it. The keys in use today are account-scoped. A leaked key
        therefore exposes every endpoint in that account.

Before: Only the form is unchecked, so it is worth being deliberate about which tool a file
        is written for.
After:  Only the form is unchecked. Each file MUST therefore be written for one tool, and
        state which.
```

## 6. Prose that should be a list

Three or more independent statements in one paragraph become a list.

```text
Before: `build.sh` derives the image tag from the branch name, so the same branch in two
        repositories takes the same tag in different registries. And `deploy.sh` reads the
        service directories under an environment to decide which images that environment
        needs — which is why an environment that does not list a service never gets its
        image.
After:  Two mechanisms depend on the distinction:

        - `build.sh` derives the image tag from the branch name. The same branch in two
          repositories therefore takes the same tag, in different registries.
        - `deploy.sh` reads the service directories under an environment to determine
          which images that environment requires. An environment that does not list a
          service does not receive its image.
```

## 7. Attributed intent and evaluative adverbs

Software sends, reads, writes, or rejects. Remove `quietly`, `happily`, `actually`, and
`never gets`.

```text
Before: ... a typo in a path cannot quietly create a new bucket beside the real one.
After:  ... a typo in a path cannot create a new bucket beside the real one.

Before: ... an environment that does not list a service never gets its image.
After:  ... an environment that does not list a service does not receive its image.

Before: | One service in one environment. This is what the scheduler actually runs, and
        what owns exactly one database. |
After:  | One service in one environment. The scheduler runs a deployment, and a
        deployment owns exactly one database. |
```

## 8. Duplication

State a fact once. For example, two documents each explain how the backup job deletes old
snapshots. Before deleting a duplicate, confirm the two passages state the same fact. Keep
the one in the document that owns the subject, and link from the other.
