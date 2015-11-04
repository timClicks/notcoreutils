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
