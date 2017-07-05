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

# Tests

The HCLRS source includes some (but probably too few) tests. There is an test which is ignored by default that uses
reference files which are not included with this source distribution. These references include reference solutions
which the author would rather not distribute publicly and prior student submissions which the author cannot redistribute.

# License

To the extent possible under law, the author(s) have dedicated all copyright and related and neighboring rights to this software to the public domain worldwide. This software is distributed without any warranty.

You should have received a copy of the CC0 Public Domain Dedication along with this software. If not, see <http://creativecommons.org/publicdomain/zero/1.0/>. 
