// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #assembler
//!
//! This is a new code base to efficiently assemble 64-bit x86 (AMD64 long mode) machine code at runtime.
//!
//! Instructions are emitted by using the mnemonic-like methods on the `InstructionStream`.
//! These are designed to work with Rust's release build optimizations to compile down to just a sequence of byte stores to memory.
//!
//! In addition, labels are supported; use of short (8-bit) labelled jumps is supported, and, where possible, are used.
//!
//! ## Example Usage
//!
//! ```
//! extern crate assembler;
//!
//! use ::assembler::*;
//! use ::assembler::mnemonic_parameter_types::BranchHint;
//! use ::assembler::mnemonic_parameter_types::Label;
//! use ::assembler::mnemonic_parameter_types::memory_operands::*;
//! use ::assembler::mnemonic_parameter_types::immediates::*;
//! use ::assembler::mnemonic_parameter_types::registers::*;
//!
//! fn main()
//! {
//! 	// SOME_LENGTH will be rounded up to the nearest power of two, with a minimum of the page size (4Kb).
//! 	const SOME_LENGTH: usize = 4096;
//! 	let memory_map = ExecutableAnonymousMemoryMap::new(SOME_LENGTH).unwrap();
//!
//! 	const SOME_LIKELY_NUMBER_OF_LABELS: usize = 512;
//!		let instruction_stream = memory_map.instruction_stream(SOME_LIKELY_NUMBER_OF_LABELS);
//!
//! 	instruction_stream.mov_...();
//!
//! 	let label = instruction_stream.create_and_attach_label();
//!
//! 	instruction_stream.jmp_Label(label)?;
//!
//! 	let function_pointer = instruction_stream.nullary_function::<()>():
//!
//! 	instruction_stream.finish();
//!
//! 	function_pointer();
//! }
//!
//! ```


extern crate libc;
#[macro_use] extern crate likely;


use self::mnemonic_parameter_types::*;
use self::mnemonic_parameter_types::immediates::*;
use self::mnemonic_parameter_types::memory::*;
use self::mnemonic_parameter_types::memory_offsets::*;
use self::mnemonic_parameter_types::registers::*;
use self::mnemonic_parameter_types::relative_addresses::*;
use ::libc::*;
use ::std::alloc::alloc;
use ::std::alloc::realloc;
use ::std::alloc::dealloc;
use ::std::alloc::Layout;
use ::std::io;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::slice::from_raw_parts;


/// Mnemonic parameter types.
pub mod mnemonic_parameter_types;

#[cfg(test)]
mod tests;


include!("ByteEmitter.rs");
include!("Displacement.rs");
include!("ExecutableAnonymousMemoryMap.rs");
include!("InstructionPointer.rs");
include!("InstructionPointerValidity.rs");
include!("InstructionStream.rs");
include!("InstructionStreamHints.rs");
include!("LabelledLocations.rs");
include!("ShortJmpResult.rs");
