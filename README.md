# Pust (Password + Rust)

_A simple static password generator for rust_

## Motivation

I just wanted to create a static password manager in Rust

## Design

By default, passwords are 38 characters long.

Obviously, static passwords of any length can be generated for other uses as well.

## Installation

TODO: Add installation instructions

## Usage

```
Usage: pust [OPTIONS] <ACCOUNT_NAME>

Arguments:
  <ACCOUNT_NAME>

Options:
      --length <LENGTH>  [default: 38]
      --enable-digit
      --enable-lower
      --enable-special
      --enable-upper
  -h, --help
```

## Acknowledgments

Got inspired by (Jarusk's pass-rs)[https://github.com/Jarusk/pass-rs]