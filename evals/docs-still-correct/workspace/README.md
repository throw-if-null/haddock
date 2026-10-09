# directory

`Directory` holds users in memory.

- `find(user_id)` returns the user with that ID, or `None`.
- `add(user)` adds a user. It raises `ValueError` when a user with the same ID exists.
