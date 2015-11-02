## Introduction

So I'm hoping that these two will be easy enough. Their help pages are quite
promising:


## `true`

`true` is a utility that returns 0, e.g. true as far as operating systems are
concerned. This will be a pretty useful test to make sure that Rust is up and
running and that I've figured out how to invoke the compiler. It's also nice
to start on level 1.

### What we'll be building

A program that runs successfully.

### Attempt 1

We can start by setting up a new project with `cargo`.

```bash
cargo new --bin noncoreutils-true
```
Inside `noncoreutils-true/src/main.rs`, let's start by removing the
`println!("Hello, world!");` "boilerplate" on line 2. I expect that `rustc`
will complain. Here goes..

```
$ cd noncoreutils-true
$ cargo run
Compiling noncoreutils-true v0.1.0 (file:///.../noncoreutils-true)
  Running `target\debug\noncoreutils-true.exe`
```
Sweet as! It worked first time. Look, for extra power let's cut a release build.

```
$ cargo run --release
Compiling noncoreutils-true v0.1.0 (file:///.../noncoreutils-true)
  Running `target\release\noncoreutils-true.exe`
```

Okay, I think we can move onto the next step.

### Things achieved here

- downloaded a copy of Rust that worked
- able to edit and save text file &ndash; which implies I didn't do something
  stupid like try to create a software package inside of a directory I don't
  have write permissions to
- able to invoke `cargo` to create a new project and compile software

## `false`

### About

If it wasn't obvious, here is a description of `false` from its [manual][f-man]

    `false` does nothing except return an exit status of 1, meaning *failure*.
    It can be used as a place holder in shell scripts where an unsuccessful
    command is needed. &hellip;

The interesting thing, is that looking at the source code makes it clear that
`false` does a little bit more than "nothing". There are quite a few macros
being called here. As a matter of fact, `false` is mostly implemented in
`true.c`.

There are also some hints is also evident from the manual that there is more to
`false` than meets the eye:

    Note that `false` (unlike all other programs documented herein) exits
    unsuccessfully, even when invoked with `--help` or `--version`.

    Portable programs should not assume that the exit status of false is 1,
    as it is greater than 1 on some non-GNU hosts.

TODO: add links to C source

[f-man]: http://www.gnu.org/software/coreutils/manual/html_node/false-invocation.html#false-invocation

### What we'll build

Judging from the documentation above, we need a program that will do three
things:

1. return a non-zero return code upon invocation
2. accept `--help` and `--version` arguments, and then still return a non-zero
   return code
3. return something different than 1 if we're compiled on a "non-GNU host"

This means we're going to learn quite a bit about Rust in a short amount of
code.

- It'll be our first touch of the standard library. Accessing command line
  arguments will be in there somewhere
- We'll need to learn how to exit from a command with a defined error code
- We'll touch upon conditional compilation in Rust

In Python 2.7, I have a fair idea of how I would go about writing `false`.
Here's something off the top of my head:

```python
import sys
import platform

RETURN_CODE = 1

if '--version' in sys.argv:
    print '0.1'
elif '--help' in sys.argv:
    print 'Usage:', sys.argv[0]

if platform.system() != 'Linux':
    RETURN_CODE = 2

sys.exit(RETURN_CODE)
```
