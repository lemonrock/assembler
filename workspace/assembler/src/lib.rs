// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #assembler
//!
//! This is a partial fork of dynasm, for compiling x86 assembler templates at build or run time.


extern crate arrayvec;
#[macro_use] extern crate bitflags;


use self::mnemomics::*;
use self::parsed_instruction::*;
use self::parsed_instruction::rust::*;
use self::size::*;
use self::relocations::*;
use ::arrayvec::ArrayVec;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::mem::swap;
use ::std::mem::transmute;


pub(crate) mod mnemomics;

pub(crate) mod parsed_instruction;

pub(crate) mod size;

pub(crate) mod relocations;


include!("AssemblingForArchitectureVariant.rs");
include!("CpuFeatures.rs");
include!("ParsedInstruction.rs");
include!("InstructionEncodingError.rs");
include!("StatementsBuffer.rs");
include!("SupportedOperationalMode.rs");
