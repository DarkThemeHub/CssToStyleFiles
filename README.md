# Css-To-StyleFile

[![Rust](https://github.com/Snazzie/CssToStyleFiles/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Snazzie/CssToStyleFiles/actions/workflows/rust.yml)

## How to build

`cargo run build --release`

## Usage

```yaml
USAGE:
    CssToStyleFiles.exe [OPTIONS] <Input> <Config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <Output>              Overrides default output path [default: ./Output]
    -r, --release-version <Release>    Overrides the use of version in config file

ARGS:
    <Input>     Path to css file
    <Config>    Path to config file
```
