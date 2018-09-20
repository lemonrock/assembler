// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(min_const_fn)]
#![feature(plugin)]
#![plugin(phf_macros)]


// TODO
#![feature(const_panic)]
#![feature(const_slice_len)]
#![feature(const_let)]


//! #assembler
//!
//! This is a partial fork of dynasm, for compiling x86 assembler templates at build or run time.


extern crate arrayvec;
#[macro_use] extern crate bitflags;
extern crate either;
extern crate phf;


use self::mnemomics::*;
use self::parsed_instruction::*;
use self::parsed_instruction::rust::*;
use self::parsing::*;
use self::relocations::*;
use ::arrayvec::ArrayVec;
use ::either::*;
use ::phf::Map;
use ::std::cmp::min;
use ::std::collections::HashMap;
use ::std::iter::Peekable;
use ::std::io;
use ::std::io::Write;
use ::std::mem::swap;
use ::std::mem::transmute;
use ::std::str::CharIndices;


pub(crate) mod mnemomics;

pub(crate) mod parsing;

pub(crate) mod parsed_instruction;

pub(crate) mod relocations;


include!("AssemblingForArchitectureVariant.rs");
include!("CpuFeatures.rs");
include!("ParsedInstruction.rs");
include!("Parser.rs");
include!("InstructionEncodingError.rs");
include!("InstructionStream.rs");
include!("Size.rs");
include!("StatementsBuffer.rs");
include!("SupportedOperationalMode.rs");
