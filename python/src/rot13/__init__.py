"""ROT13 encoding utility"""

_ROTATED_MAP = dict(
    zip(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm",
    )
)


def rot13(text: str):
    return "".join(map(lambda ch: _ROTATED_MAP[ch], text))
