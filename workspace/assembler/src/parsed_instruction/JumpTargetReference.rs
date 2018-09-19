// This file is part of assembler. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of assembler. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/assembler/master/COPYRIGHT.


/// Jump target reference.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[allow(dead_code)]
pub enum JumpTargetReference
{
	/// label PREFIX NUMBER where NUMBER is a Rust expression returning an u32.
	Labelled
	{
		prefix: String,
		number_u32: RustExpression,
	},
	
	/// An address.
	Bare
	{
		target_address_usize: RustExpression,
	},
}
