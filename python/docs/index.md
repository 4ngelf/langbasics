# rot13

ROT13 encoding utility.

This is also a project to try to make the same implementation in different programming languages.

## Usage

Encode arguments with ROT13.

### Command line interface

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

### Library

Example of using rot13 as a library.

```python
from rot13 import rot13

assert rot13("Hello encoded python!") == "Uryyb rapbqrq clguba!"
```

## Installation

1. Install [__pipx__][pipx_url] and [__poetry__][poetry_url]

[pipx_url]:https://pipx.pypa.io/stable/installation/
[poetry_url]:https://python-poetry.org/docs/#installation

2. Run these commands:

```sh
git clone https://github.com/4ngelf/rot13.git rot13 \
    && rot13/python/scripts/install
```

## Packaging

### Dependencies

The packages are listed in `pyproject.toml`. Dependencies useful
for building and testing the library are installed through the
`dev` and `docs` groups.

[Poetry][poetry-home] is required to resolve and install these dependencies.

[poetry-home]:https://python-poetry.org/

```sh
scripts/prepare  # or poetry install
```

### Testing

When testing just run:

```sh
scripts/test  # or poetry run pytest
```

### Build

To build the tarball and/or wheel run:

```sh
poetry install --only main && poetry build
```

### Docs

To build the documentation run:

```sh
poetry run mkdocs build
```
