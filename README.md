# assembler

[assembler] provides a fast, run-time assembler for x86-64 long mode instructions using function calls for Rust. In using a design that Rust's release build optimizations can work effectively with, it provides the ability to embed the assembled machine code instructions as templates inside Rust code, so as to make specialized code generation as fast as possible. It is particularly suited to being the backend to a JIT.

This technique differs to that used in [dynasm], but was driven by the need to programmatically generate complex assembler to optimize message filter functions at runtime. The code for the instruction generation is inspired by that from Stanford's [x64asm].

As a consequence, jump (and similar) instruction relaxation is not performed, ie all jumps use 32-bit displacements instead of being optimized for 8-bit displacements. Additional dedicated support is also included (eg `BitMemory`) to work with code that might be placed outside of the first 2Gb (eg on Mac OS X).


## Getting Started

Add the crate `assembler` to your Cargo.toml file as usual and add an `extern crate assembler`. Create a new instance of `ExecutableAnonymousMemoryMap` and use this to get an instance of `InstructionStream`. This contains just about everything you could want, from outputing instruciton permutations to emitting raw bytes, obtaining function pointers, using labels and dedicated support for indexed computed jumps.


## Features

* All valid x64 instructions
* Multi-byte NOPs
* All instruction set extensions except for AVX512, 3D Now!, XOP and AMD's deprecated bit manipulation
* All 8-bit (including high 8-bit), 16-bit, 32-bit and 64-bit general purpose registers
* All floating point, SSE (MM), XMM and YMM registers
* All segment registers
* Labels
* Computed Jumps
* Short cuts for C _bool returns
* Short cuts for function prologues and epilogues


## What's Missing


## Desirable

Pull requests implementing these would be much appreciated\*.
* Any support at all of the AVX512 instructions and associated memory operands.
* 3D Now!'s `PREFETCH`.
* Support for using the debug, control and bound registers.
* `if` clauses inside some instruction generation sequences to output more efficient known register forms, eg those that default to `RAX`.


<sup>\* With copyright assignment to the project, of course!</sup>


## Possible

* The instruction pointer should be an `u64` rather than `usize` so that assembling 64-bit code on 32-bit platforms is possible. That said, the main use of this project is to generate assembler to be used at runtime.


### Unlikely to be added

* Intel Xeon Phi specific instructions.
* Legacy AMD 3D Now! instructions including `PREFETCH`.
* AMD's deprecated bit manipulation instructions and `XOP` encoding prefix.
* Instruction relaxation; requires using a linked list to manage 'bundles' of instructions
* Dynamic relocation
* 32-bit compatibility mode
* 32-bit legacy mode, 16-bit mode, real mode, etc


## Licensing

The license for this project is Affero GNU Public License 3.0 (AGPL-3.0). Note that very early, unreleased versions used source code forked from the [dynasm] project; this code is no longer in use within the code base. However, the authors of [assembler] are grateful to the authors of [dynasm] for the inspiration behind this work.


[assembler]: https://github.com/lemonrock/assembler "assembler GitHub page"
[dynasm]: https://github.com/CensoredUsername/dynasm-rs "dynasm GitHub page"
[x64asm]: https://github.com/StanfordPL/x64asm "x64asm GitHub page"
