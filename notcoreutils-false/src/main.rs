use std::env;
use std::process;

#[cfg(unix)]
const EXIT_CODE: i32 = 1;
#[cfg(not(unix))]
const EXIT_CODE: i32 = 2;

fn main() {
    // via https://doc.rust-lang.org/std/env/fn.args_os.html
    for argument in env::args() {
        // thanks http://stackoverflow.com/a/29268076
        // for the .as_ref()
        match argument.as_ref() {
            "--version" => println!("0.1.0"),
            "--help" => println!("notcoreutils-false\nUsage: notcoreutils-false\n"),
            _ => {}
        }
    }
    process::exit(EXIT_CODE);
}
