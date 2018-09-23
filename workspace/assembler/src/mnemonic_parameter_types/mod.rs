// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


use super::*;


/// Immediates.
pub mod immediates;


/// Memory Offsets.
pub mod memory_offsets;


/// Memory Operands.
pub mod memory_operands;


/// Registers.
pub mod registers;


/// Relative Addresses.
pub mod relative_addresses;


include!("AsDisplacement.rs");
include!("BranchHint.rs");
include!("Label.rs");
include!("MemoryOrBranchHint.rs");
include!("MemoryOrRegister.rs");
include!("ToOpcode.rs");
