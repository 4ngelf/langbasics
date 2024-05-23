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

1. [Install poetry](https://python-poetry.org/docs/#installation)

```sh
VENV_PATH="${HOME}/.local/share/pypoetry"
mkdir -p "$VENV_PATH" \
    && python3 -m venv "$VENV_PATH" \
    && "$VENV_PATH"/bin/pip install -U pip setuptools \
    && "$VENV_PATH"/bin/pip install poetry
```

2. Run this commands:

```sh
git clone https://github.com/4ngelf/rot13.git rot13 \
    && cd rot13/python \
    && poetry install \
    && poetry build \
    && pip install dist/rot13-1.0.0-py3-none-any.whl
```

## License

[MIT license](../LICENSE)
