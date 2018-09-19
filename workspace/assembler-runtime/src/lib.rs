// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of assembler, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #assembler-runtime
//!
//! This is a partial fork of dynasm, for compiling x86 assembler templates at build time.
//!
//! This crate provides the necessary runtime support.


use self::relocations::*;
use self::relocations::long::*;
use self::relocations::protected::*;
use ::std::collections::HashMap;
use ::std::mem::size_of;
use ::std::ptr::copy_nonoverlapping;


/// Relocations.
pub mod relocations;


include!("Assembler.rs");
include!("Label.rs");
include!("PushValue.rs");
