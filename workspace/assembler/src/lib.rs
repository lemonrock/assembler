// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #assembler
//!
//! This is a new code base to efficiently assemble 64-bit x86 (AMD64 long mode) machine code at runtime.
//!
//! Instructions are emitted by using the mnemonic-like methods on the `InstructionStream` trait.
//! These are designed to work with Rust's Release optimizations to compile down to just a sequence of byte stores to memory.
//!
//! In addition, labels are supported which are then calculated using a second-pass.


use self::mnemonic_parameter_types::*;
use self::mnemonic_parameter_types::immediates::*;
use self::mnemonic_parameter_types::memory_offsets::*;
use self::mnemonic_parameter_types::memory_operands::*;
use self::mnemonic_parameter_types::registers::*;
use self::mnemonic_parameter_types::registers::singleton::*;
use self::mnemonic_parameter_types::relative_addresses::*;
use self::mnemonic_parameter_types::singletons::*;
use ::std::mem::transmute;


/// Mnemonic parameter types.
pub mod mnemonic_parameter_types;


include!("ByteEmitter.rs");
include!("Displacement.rs");
include!("InstructionStream.rs");
include!("OrdinaryInstructionStream.rs");
