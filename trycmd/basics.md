# cargo-mutants basics

This is normally run as `cargo mutants`. `cargo` passes the verb through. We should get a clean error if it's missing:

```trycmd
$ cargo-mutants
? 1
usage: cargo mutants <ARGS>
   or: cargo-mutants mutants <ARGS>

```

Also a reasonable message with the wrong subcommand:

```trycmd
$ cargo-mutants list
? 1
unrecognized cargo subcommand "list"

```

You can get help specifically for cargo-mutants:

```trycmd
$ cargo-mutants mutants --help
Usage: mutants [<cargo_test_args...>] [--all-logs] [-v] [--check] [--diff] [-d <dir>] [-f <file...>] [--json] [--list] [--list-files] [--no-copy-target] [--no-times] [-o <output>] [--shuffle] [--no-shuffle] [-t <timeout>] [-V] [--version]

Find inadequately-tested code that can be removed without any tests failing. See <https://github.com/sourcefrog/cargo-mutants> for more information.

Positional Arguments:
  cargo_test_args   pass remaining arguments to cargo test after all options and
                    after `--`.

Options:
  --all-logs        show cargo output for all invocations (very verbose).
  -v, --caught      print mutants that were caught by tests.
  --check           cargo check generated mutants, but don't run tests.
  --diff            show the mutation diffs.
  -d, --dir         rust crate directory to examine.
  -f, --file        glob for files to examine; with no glob, all files are
                    examined; globs containing slash match the entire path.
  --json            output json (only for --list).
  --list            just list possible mutants, don't run them.
  --list-files      list source files, don't run anything.
  --no-copy-target  don't copy the /target directory, and don't build the source
                    tree first.
  --no-times        don't print times or tree sizes, to make output
                    deterministic.
  -o, --output      create mutants.out within this directory.
  --shuffle         run mutants in random order.
  --no-shuffle      run mutants in the fixed order they occur in the source
                    tree.
  -t, --timeout     maximum run time for all cargo commands, in seconds.
  -V, --unviable    print mutations that failed to check or build.
  --version         show version and quit.
  --help            display usage information


```
