# assembler

[assembler] provides a fast, build-time assembler for x86-64 instructions using a NASM-similar dialect. It provides an ability to embed the assembled machine code instructions as templates inside Rust code, so as to make specialized code generation as fast as possible. Relocation can then happen at runtime.

Since it is very brittle to try to parse or integrate with Rust code at compile time, this crate tries to generate code at build time (or at runtime, if used lazily) by parsing strings. This allows one to keep assembler logic outside of Rust files, so enabling the use of regular assembler tools.


## Origin and Credits

[assembler] partly forks [dynasm] as of version 0.2.2 but with different goals and needs. [dynasm] is a superb Rust crate but is orientated around compile type code generation using compiler plugins.

[assembler] would not be possible with the extensive efforts of the authors of [dynasm], Alexander Stocko and CensoredUsername, for which the authors of this crate are extremely grateful. For the avoidance of doubt these authors retain copyright in their original and derived code.


## Licensing

The license for this project is Mozilla Public License Version 2.0 (MPL-2.0), as it was for the source code of [dynasm] from which it was forked.


[assembler]: https://github.com/lemonrock/assembler "assembler GitHub page"
[dynasm]: https://github.com/CensoredUsername/dynasm-rs "dynasm GitHub page"
