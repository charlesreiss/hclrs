# Introduction

This is an implementation of a eductional hardware description language
interpreter.

It is intended for use with the textbook _Computer Systems: A Programmer's
Perspective_ by Bryant and O'Hallaron and is similar to the language described
in Chapter 4, with some notable changes.

The language implemented is intended to be compatible "[HCL2D](https://www.cs.virginia.edu/~cr4bd/3330/F2017/hcl2d.html)" (by
Luther Tychonievich), though there are some minor differences. At present, that link is
the best description of the language implemented. This reimplementation should:

*  improve error messages;
*  enforce wire width agreement more rigorously;
*  adjust the precedence of the "in" operator so `!foo && bar in {QUUX}` parses as `!foo && (bar in {QUUX})`;
*  allow distribution of prebuilt interpreter binaries rather than requiring installation of a new language compiler;

This implementation includes some fixed functionality and output formatting which is only really useful
for implementating the 64-bit Y86 processor described in Bryant and O'Hallaron.

# Building from Source

To build this program from source, first follow the instructions on [the Rust website](https://www.rust-lang.org/en-US/install.html)
to install Rust and the Rust package manager Cargo. Then, you can build the program with `cargo`:

    cargo build --release

This will create an `hclrs` binary in `target/release`.

# Feature flags to vary language strictness

HCLRS supports several feature flags to control how picky its language is:

*  `strict-boolean-ops` (enabled by default): require arguments for `&&`, `||`, etc. to have width 1 bit or no width
*  `strict-wire-widths-binary` (disabled by default): require arguments for `+`, `-`, etc. to have the same width (or any one to have no width)
*  `require-mux-default` (enabled by default): require every case expression ("MUX") to have a default case of the form `1: value`
*  `disallow-multiple-mux-default` (enabled by default): require every case expression to not have multiple defaults. This catches errors where constants are used instead of comparing a wire to a constant (like `[ REG_NONE: 0; 1 : reg_outputA; ]` instead of `[ some_signal == REG_NONE : 0; 1 : reg_outputA; ]`
*  `disallow-unreachable-options` (enabled by default): give an error if a case expression has a case after a case which is always true

You can build a version of HCLRS with these all disabled using something like:

    cargo build --release --no-default-features

and with particular features enabled with something like:

    cargo build --release --no-default-features --features="strict--boolean-ops"

# Tests

The HCLRS source includes some (but probably too few) tests. There is an test which is ignored by default that uses
reference files which are not included with this source distribution. These references include reference solutions
which the author would rather not distribute publicly and prior student submissions which the author cannot redistribute.

For the other tests, you can run them with something like

    cargo test

Many tests will produce debug output if you set an environment variable like `RUST_LOG=hclrs::tests=debug`. 

# Debug logging

Running `hclrs` (or its tests) with an environment variable like `RUST_LOG=hclrs=debug` will produce
a lot of the debugging output. You can turn this on and off on a per-module basis (e.g.
`RUST_LOG=hclrs::ast=debug,hclrs::program=debug`.)

# License

To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring rights to this software to the public domain worldwide. This software is distributed without any warranty.

You should have received a copy of the CC0 Public Domain Dedication along with this software. If not, see <http://creativecommons.org/publicdomain/zero/1.0/>. 
