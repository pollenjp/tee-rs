# tee by rust

`tee` is a command-line utility that reads from standard input and writes to standard output and files.
Source code is available at <https://github.com/coreutils/coreutils>.

`tee-rs` is a Rust implementation of `tee` command.

```sh
pre-command | tee-rs [file ...]
```

## Support

Features

- [x] Interactive Input (Support Unix Only. WindowsOS is not supported)

OS

- [x] Linux
- [x] MacOS
- [x] WindowsOS
  - powershell `=>7.4` ([install latest powershell](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows))

Character Encoding

- [x] UTF-8

## Install

See [release](https://github.com/pollenjp/tee-rs/releases) page.

## Development

Requirements

- uv

## Test

### Interactive Input

```sh
uv run --package interactive-input-sample python ./tools/interactive-input-sample/main.py
```

```ps1
$env:PYTHONIOENCODING = "utf-8"
uv run --package interactive-input-sample python ./tools/interactive-input-sample/main.py `
  | cargo run fizz.txt
```
