# ROT13

ROT13 encoding utility.

Also a project to try to make the same implementation in different programming languages.

## Usage

Encode arguments with ROT13.

```sh
rot13 ROT13 Utility
# EBG13 Hgvyvgl
```

You can pass the `-` character as an argument to encode stdin.
This is also the default behaviour without arguments.

```sh
echo "shining" | rot13 Stars - in the sky
# Fgnef fuvavat va gur fxl

echo "Hello world!" | rot13
# Uryyb jbeyq!
```

## Installation

1. [Install poetry][poetry_url] and [pipx][pipx_url]

poetry_url:[https://python-poetry.org/docs/#installation]
pipx_url:[https://pipx.pypa.io/stable/installation/]

2. Run these commands:

```sh
git clone https://github.com/4ngelf/rot13.git rot13 \
    && rot13/python/scripts/install
```

## License

[MIT license](../LICENSE)
