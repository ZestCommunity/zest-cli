# zest-cli

> [!IMPORTANT]
> zest-cli is a minimal fork of [cargo-v5](https://github.com/vexide/cargo-v5)

> Build, upload, run, and simulate ZestCode projects!

zest-cli is a command line tool that simplifies working with [ZestCode](https://github.com/ZestCommunity/ZestCode) projects

## Installation

zest-cli comes with 2 optional features that enable extra functionality:

- `field-control`: Adds a field control tui accesible through `zest field-control` or `zest fc`.
- `fetch-template`: With this feature enabled, `zest new` will attempt to fetch the most recent upstream version of vexide-template instead of a built-in one. The command will always fall back to the built-in template.

If you wish to enable both, you can simply enable the `full` feature.

### All Features

```bash
cargo install zest-cli --features "full"
```

### Specific Feature

```bash
cargo install zest-cli --features "field-control"
```

### No Features

```bash
cargo install zest-cli
```

## Usage

Build a ZestCode project for the V5's platform target:

```bash
zest build --release
```

Upload a ZestCode project over USB (you may be prompted to provide a slot number):

```bash
zest upload
```

View serial output from the current user program:

```bash
zest terminal
```

## Configuration

Upload behavior can be configured through either your `zest.toml` file or by providing arguments to `zest-cli`.

`zest-cli` will attempt to find `zest.toml` files with the following structure for providing defaults to some upload options.

```toml
[package.metadata.v5]
name = "ZestCode Project"
description = "a ZestCode Project"
slot = 1
icon = "cool-x"
compress = true
```

### Properties

- `package.metadata.v5.name` (string): Set the program name
- `package.metadata.v5.description` (string): Set the program description
- `package.metadata.v5.slot` (integer): Set the default program slot to upload to.
- `package.metadata.v5.icon` (string) (default `"question-mark"`): Set the default program icon. (see `zest upload -h` for a list of icon strings)
- `package.metadata.v5.compress` (boolean) (default `true`): Configure if program binaries should be gzipped before uploading. It is strongly recommended to keep this at default (`true`), as disabling compression will greatly increase upload times.

For a full list of arguments, check

```
zest help
```
