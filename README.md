# Css-To-StyleFile

[![Rust](https://github.com/Snazzie/CssToStyleFiles/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Snazzie/CssToStyleFiles/actions/workflows/rust.yml)

## Supported file outputs
* stylus
* userscript


## How to build

`cargo run build --release`

## Usage

```
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

#### Example config.yaml

```yaml
themeName: "The Theme Name"
namespace: "github.com/user/repo"
version: "1.0.0"
description: "Theme Description"
author: "Author Name"
homepageUrl: "https://github.com/user/repo"
supportUrl: "https://github.com/user/repo/issues"
updateUrl: "https://raw.githubusercontent.com/user/repo/master/Generated/themename.user.extension"
urlRegex: "/^https?.*/"
```
