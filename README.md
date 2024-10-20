# tee by rust

`tee` is a command-line utility that reads from standard input and writes to standard output and files.
Source code is available at <https://github.com/coreutils/coreutils>.

`tee-rs` is a Rust implementation of `tee` command.

## Development

Requirements

- uv

## Test

### Interactive Input

```sh
uv run --package interactive-input-sample python ./tools/interactive-input-sample/main.py
```
