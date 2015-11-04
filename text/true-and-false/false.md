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

## Code

To create our new project, we use our trusty `cargo` command:

```
C:\>cd %TEMP%
C:\Users\Tim\AppData\Local\Temp>cargo new --bin false
...
```

I'm using Windows here, for reasons that will become clearer once we touch
upon conditional compilation.

### Returning non-zero

How would we go about building something like this in Rust? Let's start by
focusing on returning a non-zero return code from a Rust program.

Let's start with searching Google for "return non zero from rust". The most
promising link is a Stack Overflow post entitled,
[system - Exit Rust program early - Stack Overflow](http://stackoverflow.com/q/21569718).
Click. From the looks of things, there wasn't a clear answer until Rust 1.1. Now
there's a function that does exactly what we need.

In `false/src/main.rs` that cargo created, replace the hello world with this:

```rust
use std::process;

fn main() {
    process::exit(1);
}
```
Does it work? Let's try `cargo run`. This is what it looks like on my Windows
machine.

```
C:\Users\Tim\AppData\Local\Temp\false\src>cargo run
   Compiling false v0.1.0 (file:///C:/Users/Tim/AppData/Local/Temp/false)
     Running `C:\Users\Tim\AppData\Local\Temp\false\target\debug\false.exe`
An unknown error occurred

To learn more, run the command again with --verbose.
```

Huh? "An unknown error occurred"? Let's try adding `--verbose` and seeing what
happens.

```
C:\Users\Tim\AppData\Local\Temp\false\src>cargo run --verbose
       Fresh false v0.1.0 (file:///C:/Users/Tim/AppData/Local/Temp/false)
     Running `C:\Users\Tim\AppData\Local\Temp\false\target\debug\false.exe`
Process didn't exit successfully: `C:\Users\Tim\AppData\Local\Temp\false\target\debug\false.exe` (exit code: 1)
```
OH! Rust is telling us that `false` wasn't happy, which is perfect. That's
exactly what we wanted. Great. That was easy.

## Returning non-1 for "non-GNU hosts"

One of the most interesting snippets from the official documentation
(TODO add link) was this sentence:

    Portable programs should not assume that the exit status of false is 1,
    as it is greater than 1 on some non-GNU hosts.

This gives us a great opportunity to introduce some of Rust's conditional
compilation options. Try updating `main.rs` to match the following:

```rust
use std::process;

#[cfg(unix)]
const EXIT_CODE: i32 = 1;
#[cfg(not(unix))]
const EXIT_CODE: i32 = 2;

fn main() {
    process::exit(EXIT_CODE);
}
```

There are many ways to structure the code, but I've taken the opportunity here
to introduce another Rust feature, `const`. Perhaps misnamed, `const`s do not
live in a fixed place in memory for the life of a program's execution. Instead,
they are inlined where they're used.

I now receive a slightly different error message from `cargo run`:

```
C:\Users\ms_000\AppData\Local\Temp\false>cargo run --verbose
       Fresh false v0.1.0 (file:///C:/Users/ms_000/AppData/Local/Temp/false)
     Running `target\debug\false.exe`
Process didn't exit successfully: `target\debug\false.exe` (exit code: 2)
```

Great! The operating system received `2` back from my program. If you're running
this on Linux, OS X or another Unix-like operating system, hopefully the final
line looks something closer to:

```
Process didn't exit successfully: `target\debug\false` (exit code: 1)
```

TODO: explain variables, naming conventions, type declarations, use keyword, ...


## Processing command line arguments

The final requirement for our program will be to accept command line arguments.
We need to display something for `--version` and `--help`. Note that we still
return a non-zero code, so we don't need to touch anything else.

After a quick Google, command line arguments are available via an Interable
from `std::env`. Here is an updated `main.rs`:

```rust
use std::env;
use std::process;

#[cfg(unix)]
const EXIT_CODE: i32 = 1;
#[cfg(not(unix))]
const EXIT_CODE: i32 = 2;

fn main() {
    // see https://doc.rust-lang.org/std/env/fn.args_os.html
    for argument in env::args() {
        match argument {
            "--version" => println!("0.1.0"),
            "--help" => println!("false\nUsage: false\n"),
            _ => {}
        }
    }
    process::exit(EXIT_CODE);
}
```

```
C:\Users\Tim\AppData\Local\Temp\false>cargo run
   Compiling false v0.1.0 (file:///C:/Users/Tim/AppData/Local/Temp/false)
src\main.rs:11:5: 17:6 error: type mismatch resolving `<std::env::Args as core::iter::Iterator>::Item == &str`:

 expected struct `collections::string::String`,
    found &-ptr [E0271]
src\main.rs:11     for argument in env::args() {
src\main.rs:12         match argument {
src\main.rs:13             "--version" => println!("0.1.0"),
src\main.rs:14             "--help" => println!("false\nUsage: false\n"),
src\main.rs:15             _ => {}
src\main.rs:16         }
               ...
note: in expansion of for loop expansion
src\main.rs:11:5: 17:6 note: expansion site
src\main.rs:11:5: 17:6 help: run `rustc --explain E0271` to see a detailed explanation
error: aborting due to previous error
Could not compile `false`.

To learn more, run the command again with --verbose.
```

Rust is angry. From what I can discern, `argument` (which we know is an Iterator)
is not happy matching `&str` type, which is what string literals are useful for.

I've seen a lot of people doing things like `"hi".to_owned()` and `"bye".to_string()`. Maybe that will help...

```rust
fn main() {
    // see https://doc.rust-lang.org/std/env/fn.args_os.html
    for argument in env::args() {
        match argument {
            "--version".to_owned() => println!("0.1.0"),
            "--help".to_owned() => println!("false\nUsage: false\n"),
            _ => {}
        }
    }
    process::exit(EXIT_CODE);
}
```

```
C:\Users\Tim\AppData\Local\Temp\false>cargo run
   Compiling false v0.1.0 (file:///C:/Users/Tim/AppData/Local/Temp/false)
     Running `target\debug\false.exe`
An unknown error occurred
```

Excellent. This is progress. But now I'm left wondering if we can do better.
Surely matching strings doesn't need to look that ugly all the time? Let's ask
the Internet for help. Thankfully, as it turns out, I am not the only person
who has wanted to know how to match on strings. See if you can spot
the difference with our earlier attempt:

```rust
fn main() {
    // see https://doc.rust-lang.org/std/env/fn.args_os.html
    for argument in env::args() {
        match argument.as_ref() {
            "--version" => println!("0.1.0"),
            "--help" => println!("false\nUsage: false\n"),
            _ => {}
        }
    }
    process::exit(EXIT_CODE);
}
```

`argument.as_ref()` asks rust to provide `argument` as a reference, rather than
as a `String`. Let's try running things now.

```
C:\Users\Tim\AppData\Local\Temp\false>cargo run
   Compiling false v0.1.0 (file:///C:/Users/Tim/AppData/Local/Temp/false)
     Running `target\debug\false.exe`
An unknown error occurred
```

Looks like we're golden. To actually get some of that output, we should execute
the binary directly rather than going through `cargo`. Here's what I see:

```
C:\Users\Tim\AppData\Local\Temp\false>target\debug\false.exe --version
0.1.0
```

TODO: explain match, string types
