[![Build status](https://github.com/Dophin2009/cchmod/workflows/ci/badge.svg)](https://github.com/Dophin2009/cchmod/actions)
[![Crates.io](https://img.shields.io/crates/v/cchmod.svg)](https://crates.io/crates/cchmod)

# cchmod

cchmod is a mode and permission calculator/converter for chmod.

## Install

Install via Cargo:

```bash
$ cargo install cchmod
```

## Usage

```bash
$ cchmod
cchmod

USAGE:
    cchmod [FLAGS] <input>

ARGS:
    <input>

FLAGS:
    -h, --help       Prints help information
    -n, --num        Output the octal form
    -s, --sym        Output the symbolic form
    -V, --version    Prints version information
```

### Examples

Get the octal form of a mode:

```bash
$ cchmod -n rwxr-xr-x
755
```

Get the symbolic form of a mode:

```bash
$ cchmod -s 644
rw-r--r--
```

The same operations, on permission values:

```bash
$ cchmod -n r-x
5

$ cchmod -s 7
rwx
```
