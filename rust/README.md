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

### Rust version

```sh
git clone https://github.com/4ngelf/rot13.git rot13
cargo install --path rot13/rust
```

## License

[MIT license](./LICENSE)
