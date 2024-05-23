"""ROT13 encoding utility"""

VERSION = "1.0.0"

_ROTATED_MAP = dict(
    zip(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm",
    )
)


def rot13(text: str):
    return "".join(map(lambda ch: _ROTATED_MAP.get(ch, ch), text))
