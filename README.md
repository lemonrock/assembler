# assembler

[assembler] provides a fast, run-time assembler for x86-64 instructions using function calls for Rust. In using a design that Rust's release build optimizations can work effectively with, it provides the ability to embed the assembled machine code instructions as templates inside Rust code, so as to make specialized code generation as fast as possible. Relocation can then happen at runtime.

Since it is very brittle to try to parse or integrate with Rust code at compile time, this crate tries to generate code at build time (or at runtime, if used lazily) by parsing strings. This allows one to keep assembler logic outside of Rust files, so enabling the use of regular assembler tools.

## Licensing

The license for this project is Affero GNU Public License 3.0 (AGPL-3.0). Note that very early, unreleased versions of used source code forked from the [dynasm] project; this code is no longer in use within the code base. However, the authors of [assembler] are grateful to the authors of [dynasm] for the inspiration behind this work.


[assembler]: https://github.com/lemonrock/assembler "assembler GitHub page"
[dynasm]: https://github.com/CensoredUsername/dynasm-rs "dynasm GitHub page"
