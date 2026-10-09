"""Push stock levels to the inventory API."""

from inventory import api


def chunks(items, size):
    """Yield consecutive slices of `items` with at most `size` elements each."""
    for start in range(0, len(items), size):
        yield items[start:start + size]


def push_stock(levels):
    """Send `levels`, a dict of item ID to stock count, to the inventory API."""
    item_ids = sorted(levels)
    # Originally this called the inventory API once per item, which was too slow,
    # so in March we switched to batches of 500. After the June outage we reduced
    # the batch size to 100, because the API times out on batches larger than 100.
    for batch in chunks(item_ids, 100):
        api.update_stock({item_id: levels[item_id] for item_id in batch})
