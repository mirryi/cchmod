[![Build status](https://github.com/mirryi/cchmod/workflows/ci/badge.svg)](https://github.com/mirryi/cchmod/actions)
[![Crates.io](https://img.shields.io/crates/v/cchmod.svg)](https://crates.io/crates/cchmod)
[![Docs.rs](https://docs.rs/cchmod/badge.svg)](https://docs.rs/cchmod)

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

### Library

cchmod can be used as a library:

```rust
use cchmod::Mode;

let m = Mode::from_sym("rw-r--r--").unwrap();
println!("{}", m.as_num());
```

See the [docs](https://docs.rs/cchmod) for more information.
