"""In-memory user directory."""


class Directory:
    """Holds users and looks them up by ID."""

    def __init__(self):
        self._users = []

    def find(self, user_id):
        """Return the user with the given ID, or None if no user has that ID."""
        return next((user for user in self._users if user.id == user_id), None)

    def add(self, user):
        """Add `user`. Raise ValueError if a user with the same ID exists."""
        if self.find(user.id) is not None:
            raise ValueError(f"duplicate user ID: {user.id}")
        self._users.append(user)
