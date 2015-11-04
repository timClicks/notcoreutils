## Introduction

Welcome!

The first two projects we'll encounter are pretty straightforward. Their job is
to tell the operating system that everything is okay in the case of `true`, and
that something is wrong &ndash; no matter what &ndash; in the case of `false`.

Programs tell the operating system that everything is okay or not with the
[return code](https://en.wikipedia.org/wiki/Exit_status) from their main
function. With the exception of some fairly obscure operating systems, such as
VMS, `0` means success and everything else means failure. In case you were
wondering, VMS counts even numbers as failures, and odd numbers as success.

## What we'll cover in this section

* building a Rust program
* processing command arguments
* pattern matching
* changing our code depending on the operating system
