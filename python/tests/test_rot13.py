import string

from rot13 import rot13

ROTATED = "nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM"


def test_random_chars():
    assert rot13("a") == "n"
    assert rot13("j") == "w"
    assert rot13("i") == "v"
    assert rot13("O") == "B"
    assert rot13("#") == "#"


def test_chars():
    assert rot13(string.ascii_letters) == ROTATED


def test_phrase():
    assert rot13("Hello world!") == "Uryyb jbeyq!"
