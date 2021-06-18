# Wait-For

A rust utility to wait that a program exits with 0.

You need to wait for something to start up and don't know when it finishes?
You want to chain some other commands after it?

I've wrote it so that I can start docker, wait for it to boot up and run some commands once it's started
without the need to finish:

Just run: `wait-for docker ps && npm run <script> && npm run <other-script>` or anything alike.

Or you want to run something when an URL becomes available:

`wait-for 'curl --fail <non-existing-url>' && ./script.sh `

# Usage

```
wait-for 0.1.0
Max Strübing <mxstrbng@gmail.com>
Waits until the exit code of a program is zero

USAGE:
    wait-for [FLAGS] [OPTIONS] <COMMAND>...

FLAGS:
        --debug       Outputs debug information
    -h, --help        Prints help information
    -n, --no-retry    Don't try to rerun the command in case it fails with non-zero exit code
    -V, --version     Prints version information
        --verbose     Outputs verbose information

OPTIONS:
    -i, --interval <interval>    in which interval the command should be retried in milliseconds [default: 1000]

ARGS:
    <COMMAND>...    Which command should be waited for
```

# Contribution

- Fork this project
- Create a branch
- Provide a pull request

The CI will lint your commit message with [commitlint](https://commitlint.js.org/#/).
