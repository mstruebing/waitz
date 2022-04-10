# Waitz

A rust utility to wait that a program exits with 0.

You need to wait for something to start up and don't know when it finishes?
You want to chain some other commands after it? You want to run a bunch of commands and drink a coffee?
Than this is a tool for you.

I initially wrote it to start docker and run some processes after it, but you can do way more with it,
like waiting for a specific URL to become available after booting up a server in the background or anything else.

- `waitz docker ps && npm run <script> && npm run <other-script>`
- `waitz 'curl --fail <non-existing-url>' && ./script.sh`

# Usage

```
waitz 0.2.2
Max Strübing <mxstrbng@gmail.com>
Waits (retries) a command until it exits with 0

USAGE:
    waitz [FLAGS] [OPTIONS] <COMMAND>...

FLAGS:
        --debug       Outputs debug information
    -h, --help        Prints help information
    -n, --no-retry    Don't try to rerun the command in case it fails with non-zero exit code
    -V, --version     Prints version information
        --verbose     Forwards stdout/stderr from the command to the terminal

OPTIONS:
    -i, --interval <interval>    in which interval the command should be retried in milliseconds [default: 1000]

ARGS:
    <COMMAND>...    Which command should be waited for
```

# Installation

## Crates.io

`cargo install waitz`

## Raw

Clone the repository and run `cargo build --release` and you should find the binary in `./target/release/waitz`.

## Release Page

Or grab a binary from the [release page](https://github.com/mstruebing/waitz/releases)

# Contribution

- Fork this project
- Create a branch
- Provide a pull request

The CI will lint your commit message with [commitlint](https://commitlint.js.org/#/).
