### Tests and linter statuses:
[![RustCI](https://github.com/WeedRap/gendiff/actions/workflows/rustci.yml/badge.svg)](https://github.com/WeedRap/gendiff/actions/workflows/rustci.yml)
[![codecov](https://codecov.io/gh/WeedRap/gendiff/graph/badge.svg?token=1G3H6cxnt0)](https://codecov.io/gh/WeedRap/gendiff)
_____

# Gendiff 🟰 
### Generator of differences
The "gendiff" utility is a command-line tool designed to compare and display the differences between two files in various formats: plain or stylish. It supports files in both **JSON** and **YAML** (including .yml) formats, making it a versatile choice for comparing configuration files, data sets, or any structured documents.


## Content
- [Requirements](#requirements)
- [Used packages](#used-packages)
- [Installation](#installation)
- [How to use](#how-to-use)

## Requirements
- [rust](https://www.rust-lang.org/tools/install)

## Used packages
- [clap](https://crates.io/crates/clap)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [serde_yaml](https://crates.io/crates/serde_yaml)

**DEV SECTION**
- [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) _(for test coverage)_

## Installation
```shell
cargo install --path .
```

## How to use
```shell
gendiff -h

Compares two configuration files and shows a difference.

Usage: gendiff [OPTIONS] <first_file> <second_file>

Arguments:
  <first_file>   
  <second_file>  

Options:
  -f, --format <format>  Set the format of output [default: stylish]
  -h, --help             Print help
  -V, --version          Print version
```
