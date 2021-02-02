# log-highlight

log-highlight - a universal log text to colorized text converter

## Dependency
Please see Cargo.lock.

## setup

- amd64

```bash
$ docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
```

- x86

```bash
$ docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:i686-musl cargo build --release
```

## Usage

```bash
Usage: log-highlight [OPTION]... [FILE]...
highlight each FILE to standard output based on rules.
With no FILE, or when FILE is -, read standard input.

  -c, --config=FILE    load highlight rules from FILE
```

## License

This software is released under the [MIT license](https://en.wikipedia.org/wiki/MIT_License), see LICENSE.

## References

https://github.com/messense/rust-musl-cross

Enjoy it!

Thank you!
